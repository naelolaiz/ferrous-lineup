//! Audio preferences: real instrument sounds from a SoundFont.
//!
//! The Lineup Osc makes every instrument out of a few simple waves. A
//! SoundFont holds recordings of real ones (see
//! [`lineup_audio::soundfont`]). With Settings > Audio > SoundFont instruments on,
//! every channel that has a General MIDI instrument (chosen in the
//! channel rack, or given by a demo or the Library) plays it from the
//! SoundFont, and with SoundFont drums on, the built-in drums play the
//! SoundFont's drum kit.
//!
//! The app does not come with a SoundFont, because a good one is tens of
//! megabytes. When the app opens or anything plays with the sounds off,
//! until the offer is answered, and when the sounds are turned on, it
//! offers to download GeneralUser GS, a free General MIDI SoundFont by
//! S. Christian Collins, and keep it in the app's data folder, or to use
//! it if it is there already. Any other `.sf2` file can be chosen
//! instead. Downloading and reading a SoundFont take a while, so both run
//! on a thread of their own, like the WAV export, and the window keeps
//! drawing meanwhile. When the SoundFont is ready, every channel is given
//! its instrument again.
//!
//! The sound preferences are remembered between runs, by name (see
//! [`super::preferences`]), so an update keeps them.

use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::thread::{self, JoinHandle};
use std::time::Duration;

use eframe::egui;
use lineup_audio::SoundFont;
use lineup_audio::soundfont;
use serde::{Deserialize, Serialize};

use super::{APP_NAME, LineupApp, preferences};
use crate::i18n::{tr, trf};

/// The SoundFont offered for download.
const GENERAL_USER: &str = "GeneralUser GS";
/// Its file name, in the app's data folder.
const GENERAL_USER_FILE: &str = "GeneralUser-GS.sf2";
/// Where it is downloaded from: its author's own repository.
const GENERAL_USER_URL: &str =
    "https://github.com/mrbumpy409/GeneralUser-GS/raw/main/GeneralUser-GS.sf2";
/// Its page, with its licence and its documentation.
const GENERAL_USER_PAGE: &str = "https://github.com/mrbumpy409/GeneralUser-GS";
/// An environment variable that gives another address to download it
/// from, such as a copy on your own network.
const URL_VARIABLE: &str = "FERROUS_LINEUP_SOUNDFONT_URL";
/// The biggest download taken: no General MIDI SoundFont is this big, so
/// a bigger file is something else.
const MAX_DOWNLOAD: u64 = 1 << 30;
/// What a download that was stopped returns, which is not a problem to
/// report.
const CANCELLED: &str = "cancelled";

/// Sound preferences, kept between runs.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
struct Settings {
    /// Whether channels play from the SoundFont.
    on: bool,
    /// Whether the built-in drums do too.
    drums: bool,
    /// A SoundFont file chosen with "Choose SoundFont…", or `None` for
    /// GeneralUser GS.
    file: Option<PathBuf>,
    /// Whether the offer of real instrument sounds was turned down, with
    /// "Not now", or the sounds were turned off in Settings. Until then,
    /// the app offers them as it opens and on Play whenever they are off.
    turned_down: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            on: false,
            drums: true,
            file: None,
            turned_down: false,
        }
    }
}

/// The name the settings are kept under between runs.
const SETTINGS_NAME: &str = "sound-font";

/// Where GeneralUser GS is kept once downloaded, such as
/// `~/.local/share/ferrouslineup/soundfonts/` on Linux. `None` on a
/// system with no folder for app data.
fn general_user_path() -> Option<PathBuf> {
    eframe::storage_dir(APP_NAME).map(|dir| dir.join("soundfonts").join(GENERAL_USER_FILE))
}

/// A folder as people write it: `~/.local/share/…` rather than the whole
/// path of the home folder, on Linux and macOS.
fn shown(folder: &Path) -> String {
    let home = std::env::home_dir().filter(|_| cfg!(unix));
    match home.and_then(|home| folder.strip_prefix(home).ok().map(Path::to_path_buf)) {
        Some(inside) => format!("~/{}", inside.display()),
        None => folder.display().to_string(),
    }
}

fn file_name(path: &Path) -> String {
    path.file_name().map_or_else(
        || path.display().to_string(),
        |name| name.to_string_lossy().into_owned(),
    )
}

/// The SoundFont, its settings, and any download or
/// reading going on.
pub(super) struct Sounds {
    settings: Settings,
    /// The SoundFont read last, and the file it came from. Kept while
    /// the sounds are off, so turning them on again is instant.
    loaded: Option<(PathBuf, Arc<SoundFont>)>,
    /// Where GeneralUser GS is kept: [`general_user_path`].
    general_user: Option<PathBuf>,
    /// A download or a reading running in the background.
    job: Option<Job>,
    /// Whether the window that offers the download is open.
    offer_open: bool,
    /// Why the last download failed, shown in that window.
    failure: Option<String>,
    /// Whether the settings changed since they were last stored.
    changed: bool,
}

/// A SoundFont being downloaded or read.
struct Job {
    path: PathBuf,
    /// Whether it is downloaded first, or only read.
    download: bool,
    progress: Arc<Progress>,
    worker: JoinHandle<Result<Arc<SoundFont>, String>>,
}

/// How far a download is. Its thread writes it and the UI thread reads
/// it; atomics need no lock for that.
#[derive(Default)]
struct Progress {
    /// Bytes downloaded so far.
    done: AtomicU64,
    /// The size of the file, or 0 while it is not known.
    total: AtomicU64,
    /// Set by the UI thread to stop the download.
    cancel: AtomicBool,
}

impl Progress {
    /// How far along, from 0.0 to 1.0, if the size is known.
    fn fraction(&self) -> Option<f32> {
        let total = self.total.load(Ordering::Relaxed);
        (total > 0).then(|| self.done.load(Ordering::Relaxed) as f32 / total as f32)
    }

    /// "12.3 of 30.7 MB".
    fn text(&self) -> String {
        let megabytes = |bytes: u64| bytes as f64 / 1_000_000.0;
        let done = megabytes(self.done.load(Ordering::Relaxed));
        match self.total.load(Ordering::Relaxed) {
            0 => format!("{done:.1} MB"),
            total => trf(
                "{} of {} MB",
                &[&format!("{done:.1}"), &format!("{:.1}", megabytes(total))],
            ),
        }
    }
}

impl Sounds {
    /// The settings stored last time. If the SoundFont was on, it is read
    /// again in the background; if its file is gone, it is turned off.
    pub(super) fn new(ctx: &egui::Context) -> Self {
        let mut sounds = Sounds {
            settings: preferences::get(ctx, SETTINGS_NAME).unwrap_or_default(),
            loaded: None,
            general_user: general_user_path(),
            job: None,
            offer_open: false,
            failure: None,
            changed: false,
        };
        if sounds.settings.on {
            match sounds.wanted_path().filter(|path| path.is_file()) {
                Some(path) => sounds.start(path, false),
                None => sounds.turn_off(),
            }
        }
        sounds
    }

    /// The SoundFont the channels play and whether the drums play it too,
    /// when the sounds are on and it has been read.
    pub(super) fn in_use(&self) -> Option<(&Arc<SoundFont>, bool)> {
        let (path, sound_font) = self.loaded.as_ref()?;
        let wanted = self.settings.on && Some(path) == self.wanted_path().as_ref();
        wanted.then_some((sound_font, self.settings.drums))
    }

    /// The SoundFont file the settings ask for.
    fn wanted_path(&self) -> Option<PathBuf> {
        self.settings
            .file
            .clone()
            .or_else(|| self.general_user.clone())
    }

    /// The name of the SoundFont the settings ask for.
    fn name(&self) -> String {
        self.settings
            .file
            .as_deref()
            .map_or_else(|| GENERAL_USER.to_string(), file_name)
    }

    fn turn_off(&mut self) {
        self.settings.on = false;
        self.changed = true;
    }

    /// Starts downloading (if `download`) and reading `path`, in place of
    /// any job running: a download is stopped, a reading is left to end
    /// on its own.
    fn start(&mut self, path: PathBuf, download: bool) {
        if let Some(job) = self.job.take() {
            job.progress.cancel.store(true, Ordering::Relaxed);
        }
        self.failure = None;
        let progress = Arc::new(Progress::default());
        let worker = {
            let progress = Arc::clone(&progress);
            let path = path.clone();
            thread::spawn(move || {
                if download {
                    let url = std::env::var(URL_VARIABLE)
                        .unwrap_or_else(|_| GENERAL_USER_URL.to_string());
                    download_to(&url, &path, &progress)
                } else {
                    soundfont::load(&path).map_err(|err| err.to_string())
                }
            })
        };
        self.job = Some(Job {
            path,
            download,
            progress,
            worker,
        });
    }

    /// What Settings and the status bar say is going on, if anything.
    fn activity(&self) -> Option<String> {
        let job = self.job.as_ref()?;
        Some(if job.download {
            trf("Downloading {}: {}", &[&GENERAL_USER, &job.progress.text()])
        } else {
            trf("Reading {}…", &[&file_name(&job.path)])
        })
    }
}

/// Downloads `url` to `path` and reads it, which also checks that it is
/// a SoundFont. It is written to a `.part` file first, which takes the
/// name `path` only once it is whole and readable, so a download that
/// stops halfway never leaves a broken SoundFont behind.
fn download_to(url: &str, path: &Path, progress: &Progress) -> Result<Arc<SoundFont>, String> {
    let part = path.with_extension("sf2.part");
    let result = download_part(url, &part, progress).and_then(|()| {
        let sound_font = soundfont::load(&part).map_err(|err| err.to_string())?;
        fs::rename(&part, path).map_err(|err| err.to_string())?;
        Ok(sound_font)
    });
    if result.is_err() {
        let _ = fs::remove_file(&part);
    }
    result
}

fn download_part(url: &str, part: &Path, progress: &Progress) -> Result<(), String> {
    if let Some(folder) = part.parent() {
        fs::create_dir_all(folder)
            .map_err(|err| trf("could not make {}: {}", &[&folder.display(), &err]))?;
    }
    // The certificates that prove the server is who it says are the
    // ones the system trusts. A proxy set in HTTPS_PROXY is used.
    let tls = ureq::tls::TlsConfig::builder()
        .root_certs(ureq::tls::RootCerts::PlatformVerifier)
        .build();
    let agent: ureq::Agent = ureq::Agent::config_builder()
        .tls_config(tls)
        .timeout_connect(Some(Duration::from_secs(30)))
        .build()
        .into();
    let response = agent.get(url).call().map_err(|err| err.to_string())?;
    let body = response.into_body();
    progress
        .total
        .store(body.content_length().unwrap_or(0), Ordering::Relaxed);
    let mut reader = body.into_reader();
    let mut file = File::create(part).map_err(|err| err.to_string())?;
    let mut buffer = vec![0; 64 * 1024];
    let mut done = 0;
    loop {
        if progress.cancel.load(Ordering::Relaxed) {
            return Err(CANCELLED.to_string());
        }
        let read = reader.read(&mut buffer).map_err(|err| err.to_string())?;
        if read == 0 {
            break;
        }
        done += read as u64;
        if done > MAX_DOWNLOAD {
            return Err(tr("the file is too big to be a SoundFont").to_string());
        }
        file.write_all(&buffer[..read])
            .map_err(|err| err.to_string())?;
        progress.done.store(done, Ordering::Relaxed);
    }
    file.flush().map_err(|err| err.to_string())
}

impl LineupApp {
    /// Instrument playback and the selected SoundFont in Settings > Audio.
    pub(super) fn sound_settings_ui(&mut self, ui: &mut egui::Ui) {
        super::settings::card(
            ui,
            "Instrument playback",
            "Use recorded instruments from a SoundFont, or the built-in Lineup Osc.",
            |ui| {
                let mut on = self.sounds.settings.on;
                if ui.checkbox(&mut on, tr("SoundFont instruments")).on_hover_text(
                tr("General MIDI channels use the SoundFont. Channels using a Lineup Osc sound keep it."),
            ).changed() {
                self.sounds.settings.turned_down = !on;
                self.set_sound_font(on);
            }
                let mut drums = self.sounds.settings.drums;
                if ui
                    .add_enabled(on, egui::Checkbox::new(&mut drums, tr("SoundFont drums")))
                    .on_hover_text(tr("The built-in drums play the SoundFont’s drum kit too."))
                    .changed()
                {
                    self.sounds.settings.drums = drums;
                    self.sounds.changed = true;
                    self.live.instruments_changed();
                }
            },
        );
        super::settings::card(
            ui,
            "SoundFont library",
            "A General MIDI .sf2 file supplies the instrument recordings.",
            |ui| {
                ui.strong(self.sounds.name());
                if let Some(path) = self.sounds.wanted_path() {
                    ui.label(
                        egui::RichText::new(shown(&path))
                            .small()
                            .color(crate::theme::TEXT_DIM),
                    );
                }
                if let Some(state) = self.sounds.activity() {
                    ui.label(state);
                }
                ui.horizontal_wrapped(|ui| {
                    if ui.button(tr("Choose SoundFont…")).clicked() {
                        self.choose_sound_font();
                    }
                    if self.sounds.settings.file.is_some()
                        && ui.button(trf("Use {}", &[&GENERAL_USER])).clicked()
                    {
                        self.sounds.settings.file = None;
                        self.sounds.changed = true;
                        self.set_sound_font(true);
                    }
                });
            },
        );
    }

    /// When the app opens, and whenever anything plays on the Lineup Osc,
    /// offers the SoundFont, until the offer is turned down: the Lineup Osc
    /// makes every instrument out of a few simple waves, which is not how a
    /// band should sound. Said yes to, the instruments play from GeneralUser
    /// GS as soon as it is downloaded, or read if it is on disk already.
    /// Nothing is asked while the sounds are on, even if their SoundFont is
    /// still being read.
    pub(super) fn offer_sound_font(&mut self) {
        let sounds = &mut self.sounds;
        if !sounds.settings.on && !sounds.settings.turned_down && sounds.general_user.is_some() {
            sounds.offer_open = true;
        }
    }

    /// Turns the SoundFont on or off. Turned on, it is read if it is not
    /// yet, or its download is offered.
    fn set_sound_font(&mut self, on: bool) {
        self.sounds.settings.on = on;
        self.sounds.changed = true;
        if on {
            self.prepare_sound_font();
        }
        self.live.instruments_changed();
    }

    /// Makes sure the SoundFont the settings ask for is read or being
    /// read: from its file if there is one; otherwise GeneralUser GS is
    /// offered for download.
    fn prepare_sound_font(&mut self) {
        let sounds = &mut self.sounds;
        let Some(path) = sounds.wanted_path() else {
            sounds.turn_off();
            self.message = Some(tr("There is no folder to keep a SoundFont in").to_string());
            return;
        };
        let loaded = sounds
            .loaded
            .as_ref()
            .is_some_and(|(file, _)| *file == path);
        let coming = sounds.job.as_ref().is_some_and(|job| job.path == path);
        if loaded || coming {
            return;
        }
        if path.is_file() {
            sounds.start(path, false);
        } else if sounds.settings.file.is_none() {
            sounds.offer_open = true;
        } else {
            sounds.turn_off();
            self.message = Some(trf("The SoundFont {} is gone", &[&path.display()]));
        }
    }

    /// Asks for a SoundFont file, then plays from it.
    fn choose_sound_font(&mut self) {
        let Some(path) = rfd::FileDialog::new()
            .add_filter("SoundFont", &["sf2", "SF2"])
            .pick_file()
        else {
            return;
        };
        self.sounds.settings.file = Some(path);
        self.sounds.settings.turned_down = false;
        self.sounds.offer_open = false;
        self.sounds.changed = true;
        self.set_sound_font(true);
    }

    /// Once per frame: stores changed settings, and takes in a SoundFont
    /// that has been downloaded or read.
    pub(super) fn check_sounds(&mut self, ctx: &egui::Context) {
        if self.sounds.changed {
            preferences::set(ctx, SETTINGS_NAME, &self.sounds.settings);
            self.sounds.changed = false;
        }
        let Some(job) = &self.sounds.job else {
            return;
        };
        if !job.worker.is_finished() {
            ctx.request_repaint_after(Duration::from_millis(100));
            return;
        }
        let Some(job) = self.sounds.job.take() else {
            return;
        };
        let name = file_name(&job.path);
        match job.worker.join() {
            Ok(Ok(sound_font)) => {
                self.sounds.loaded = Some((job.path, sound_font));
                self.sounds.offer_open = false;
                if self.sounds.in_use().is_some() {
                    self.notice = Some(trf("Instruments play from {}", &[&self.sounds.name()]));
                    self.live.instruments_changed();
                }
            }
            Ok(Err(err)) if err == CANCELLED => {}
            Ok(Err(err)) => {
                if job.download {
                    self.sounds.failure = Some(err);
                    self.sounds.offer_open = true;
                } else {
                    self.message = Some(trf("Could not read {}: {}", &[&name, &err]));
                }
                if self.sounds.in_use().is_none() {
                    self.sounds.turn_off();
                }
            }
            Err(_) => {
                self.message = Some(trf("Reading {} failed", &[&name]));
                self.sounds.turn_off();
            }
        }
    }

    /// The status bar's part: a download's progress, or a SoundFont being
    /// read.
    pub(super) fn sounds_status(&self, ui: &mut egui::Ui) {
        if let Some(activity) = self.sounds.activity() {
            ui.separator();
            ui.spinner();
            ui.label(activity);
        }
    }

    /// The window that offers GeneralUser GS: to download it, or to use it
    /// if it is on disk already. It shows the download going on.
    pub(super) fn sound_font_window(&mut self, ctx: &egui::Context) {
        // Unsaved work from a run that did not close cleanly is offered
        // first.
        if !self.sounds.offer_open || !self.recovery.pending.is_empty() {
            return;
        }
        let downloading = self.sounds.job.as_ref().is_some_and(|job| job.download);
        let on_disk = !downloading
            && self
                .sounds
                .general_user
                .as_deref()
                .is_some_and(Path::is_file);
        let folder = self
            .sounds
            .general_user
            .as_deref()
            .and_then(|path| path.parent().map(Path::to_path_buf))
            .map_or_else(String::new, |folder| shown(&folder));
        let mut open = true;
        let mut download = false;
        let mut use_it = false;
        let mut choose = false;
        let mut stop = false;
        // In front of every window: a demo opens on the chord sheet,
        // brought to the front, as it starts to play and asks this.
        egui::Window::new(tr("Real instrument sounds"))
            .id(egui::Id::unique("real-instrument-sounds"))
            .order(egui::Order::Foreground)
            .open(&mut open)
            .collapsible(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::ZERO)
            .show(ctx, |ui| {
                ui.set_width(420.0);
                ui.label(if on_disk {
                    tr(
                        "Real instrument sounds come from a SoundFont, a file of recorded \
                        instruments. GeneralUser GS, by S. Christian Collins, a free \
                        General MIDI SoundFont, is already on this computer, in:",
                    )
                } else {
                    tr(
                        "Real instrument sounds come from a SoundFont, a file of recorded \
                        instruments. GeneralUser GS, by S. Christian Collins, is a free \
                        General MIDI SoundFont of about 30 MB. It is downloaded once and \
                        kept in:",
                    )
                });
                ui.monospace(&folder);
                ui.hyperlink_to(
                    trf("About {} and its licence", &[&GENERAL_USER]),
                    GENERAL_USER_PAGE,
                );
                ui.add_space(8.0);
                if let Some(job) = self.sounds.job.as_ref().filter(|_| downloading) {
                    let bar = egui::ProgressBar::new(job.progress.fraction().unwrap_or(0.0))
                        .text(job.progress.text());
                    ui.add(bar);
                    ui.add_space(4.0);
                    stop = ui.button(tr("Stop")).clicked();
                    return;
                }
                if let Some(failure) = &self.sounds.failure {
                    ui.colored_label(
                        ui.visuals().error_fg_color,
                        trf("Could not download it: {}", &[failure]),
                    );
                    ui.label(trf(
                        "You can also download {} from its page yourself, then choose it here.",
                        &[&GENERAL_USER_FILE],
                    ));
                    ui.add_space(4.0);
                }
                ui.label(tr("Play the instruments with it?"));
                ui.horizontal(|ui| {
                    if on_disk {
                        use_it = ui.button(tr("Use it")).clicked();
                    } else {
                        download = ui.button(tr("Download")).clicked();
                    }
                    choose = ui.button(tr("Choose a file…")).clicked();
                    if ui.button(tr("Not now")).clicked() {
                        stop = true;
                    }
                });
            });

        // Yes: GeneralUser GS, even in place of a chosen file that is
        // gone, played from as soon as it is downloaded or read.
        if (download || use_it)
            && let Some(path) = self.sounds.general_user.clone()
        {
            let sounds = &mut self.sounds;
            sounds.settings.file = None;
            sounds.settings.turned_down = false;
            if download {
                sounds.settings.on = true;
                sounds.changed = true;
                sounds.start(path, true);
            } else {
                sounds.offer_open = false;
                self.set_sound_font(true);
            }
        } else if choose {
            self.choose_sound_font();
        } else if stop {
            if let Some(job) = self.sounds.job.take() {
                job.progress.cancel.store(true, Ordering::Relaxed);
            }
            self.decline_sound_font();
        }
        // Closed while downloading, the download carries on and the
        // status bar shows it; closed before, it means "Not now".
        if !open {
            self.sounds.offer_open = false;
            if !downloading {
                self.decline_sound_font();
            }
        }
    }

    /// "Not now": the Lineup Osc plays on, and neither opening the app nor
    /// Play offers the SoundFont again. Settings > Audio has it for later.
    fn decline_sound_font(&mut self) {
        let sounds = &mut self.sounds;
        sounds.offer_open = false;
        sounds.settings.turned_down = true;
        sounds.turn_off();
    }
}

#[cfg(test)]
impl Sounds {
    /// For the tests: GeneralUser GS kept at `path`, which need not be
    /// there, instead of in the app's data folder.
    pub(super) fn keep_general_user_in(&mut self, path: PathBuf) {
        self.general_user = Some(path);
    }

    /// For the tests: whether opening the app and Play are to offer the
    /// SoundFont, as when the offer was not turned down yet.
    pub(super) fn offer(&mut self, offer: bool) {
        self.settings.turned_down = !offer;
        self.changed = true;
    }

    /// Whether the window that offers the download is open, and whether
    /// the instruments are to play from the SoundFont.
    pub(super) fn offered_and_on(&self) -> (bool, bool) {
        (self.offer_open, self.settings.on)
    }

    /// The SoundFont file being read, if one is.
    pub(super) fn reading(&self) -> Option<&Path> {
        let job = self.job.as_ref().filter(|job| !job.download)?;
        Some(&job.path)
    }
}
