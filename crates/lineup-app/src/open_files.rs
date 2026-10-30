//! Files the system asks the app to open.
//!
//! On Linux and Windows, opening a project from the file manager starts
//! the app with the file's path on the command line (see `main`). macOS
//! starts it with no arguments and sends the files in an Apple event, an
//! "open documents" message, which also comes to an app already running
//! when another file is opened. This module takes those messages and
//! keeps their paths until the app asks for them, once a frame.

use std::path::PathBuf;
use std::sync::{Mutex, OnceLock};

use eframe::egui;

/// The paths asked for and not yet opened.
static WAITING: Mutex<Vec<PathBuf>> = Mutex::new(Vec::new());

/// The window, to wake when a file comes while nothing moves on it.
static WINDOW: OnceLock<egui::Context> = OnceLock::new();

/// Starts taking the system's messages. Called once, on the main thread,
/// before the window opens: the files a launch was for come while it
/// opens.
pub fn listen() {
    #[cfg(target_os = "macos")]
    mac::listen();
}

/// Lets a file that comes later wake the window.
pub fn wake(ctx: &egui::Context) {
    // Only the first window counts: there is only one.
    let _ = WINDOW.set(ctx.clone());
}

/// The files asked for since the last call, oldest first.
pub fn take() -> Vec<PathBuf> {
    std::mem::take(
        &mut *WAITING
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner()),
    )
}

/// Keeps `paths` for [`take`], and wakes the window to take them.
#[cfg_attr(not(target_os = "macos"), allow(dead_code))]
fn arrived(paths: impl IntoIterator<Item = PathBuf>) {
    WAITING
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
        .extend(paths);
    if let Some(ctx) = WINDOW.get() {
        ctx.request_repaint();
    }
}

/// The "open documents" Apple event.
///
/// AppKit has a handler of its own for it, which asks the application's
/// delegate to open the files. The delegate belongs to winit, which
/// opens no files, so this handler takes its place. Apple's rule is to
/// set it when the application is about to finish launching: after
/// AppKit sets its own, and before the event of the launch arrives. An
/// object that listens for that moment is made before the window opens
/// and is kept for as long as the app runs.
#[cfg(target_os = "macos")]
mod mac {
    use objc2::rc::Retained;
    use objc2::runtime::{AnyObject, NSObject};
    use objc2::{MainThreadMarker, MainThreadOnly, define_class, msg_send, sel};
    use objc2_app_kit::NSApplicationWillFinishLaunchingNotification;
    use objc2_foundation::{
        NSAppleEventDescriptor, NSAppleEventManager, NSNotification, NSNotificationCenter,
    };

    /// Apple event codes, four letters each: the core event class, the
    /// "open documents" event and its direct object, the files.
    const CORE_EVENT_CLASS: u32 = u32::from_be_bytes(*b"aevt");
    const OPEN_DOCUMENTS: u32 = u32::from_be_bytes(*b"odoc");
    const DIRECT_OBJECT: u32 = u32::from_be_bytes(*b"----");

    define_class!(
        // SAFETY: NSObject has no rules for subclasses, and `Opener`
        // does not implement `Drop`.
        #[unsafe(super(NSObject))]
        #[thread_kind = MainThreadOnly]
        #[name = "FerrousLineupOpener"]
        struct Opener;

        impl Opener {
            #[unsafe(method(willFinishLaunching:))]
            fn will_finish_launching(&self, _notification: &NSNotification) {
                let manager = NSAppleEventManager::sharedAppleEventManager();
                let target: &AnyObject = self;
                // SAFETY: the selector is the method below, which takes
                // the event and its reply, as the manager calls it.
                unsafe {
                    let _: () = msg_send![
                        &*manager,
                        setEventHandler: target,
                        andSelector: sel!(openDocuments:withReplyEvent:),
                        forEventClass: CORE_EVENT_CLASS,
                        andEventID: OPEN_DOCUMENTS
                    ];
                }
            }

            #[unsafe(method(openDocuments:withReplyEvent:))]
            fn open_documents(
                &self,
                event: &NSAppleEventDescriptor,
                _reply: &NSAppleEventDescriptor,
            ) {
                // SAFETY: `paramDescriptorForKeyword:` takes a four-letter
                // code and gives a descriptor or nil.
                let files: Option<Retained<NSAppleEventDescriptor>> =
                    unsafe { msg_send![event, paramDescriptorForKeyword: DIRECT_OBJECT] };
                let Some(files) = files else {
                    return;
                };
                // A list of files, counted from 1, or a single one.
                let count = files.numberOfItems();
                let items: Vec<Retained<NSAppleEventDescriptor>> = if count == 0 {
                    vec![files]
                } else {
                    (1..=count)
                        .filter_map(|index| files.descriptorAtIndex(index))
                        .collect()
                };
                super::arrived(
                    items
                        .iter()
                        .filter_map(|item| item.fileURLValue())
                        .filter_map(|url| url.to_file_path()),
                );
            }
        }
    );

    pub(super) fn listen() {
        let Some(mtm) = MainThreadMarker::new() else {
            return;
        };
        // SAFETY: a plain `init`, as for any NSObject.
        let opener: Retained<Opener> = unsafe { msg_send![Opener::alloc(mtm), init] };
        let observer: &AnyObject = &opener;
        // SAFETY: the selector is the method above, which takes the
        // notification.
        unsafe {
            NSNotificationCenter::defaultCenter().addObserver_selector_name_object(
                observer,
                sel!(willFinishLaunching:),
                Some(NSApplicationWillFinishLaunchingNotification),
                None,
            );
        }
        // Neither the notification centre nor the event manager keeps the
        // opener alive, and it is needed for as long as the app runs.
        std::mem::forget(opener);
    }
}
