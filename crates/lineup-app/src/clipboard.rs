//! Typed clipboard contents keep notes and clips distinct from chord text.
use eframe::egui::{Context, Event};
use serde::{Serialize, de::DeserializeOwned};

pub fn copy(ctx: &Context, kind: &str, value: &impl Serialize) {
    if let Ok(text) = ron::to_string(value) {
        ctx.copy_text(format!("Ferrous Lineup {kind}\n{text}"));
    }
}
pub fn paste<T: DeserializeOwned>(ctx: &Context, kind: &str) -> Option<T> {
    let prefix = format!("Ferrous Lineup {kind}\n");
    ctx.input(|input| {
        input.events.iter().find_map(|event| match event {
            Event::Paste(text) => text
                .strip_prefix(&prefix)
                .and_then(|text| ron::from_str(text).ok()),
            _ => None,
        })
    })
}
pub fn has_paste(ctx: &Context) -> bool {
    ctx.input(|input| {
        input
            .events
            .iter()
            .any(|event| matches!(event, Event::Paste(_)))
    })
}
