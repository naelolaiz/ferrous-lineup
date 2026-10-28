//! The lead sheet window: the song's tune on a staff with its chords
//! above, laid out on pages as they will be printed.
//!
//! Everything shown is worked out by `lineup_core::lead_sheet` from the
//! song: the melody from one channel's notes, the chord symbols from
//! the chord sheet, the sections, repeats and key and time signatures
//! from the playlist. This window only picks what to write (which
//! channel is the melody, the clef, who reads it, the paper), draws the
//! pages, lights up the bar that plays, and saves or prints a PDF of the
//! same pages.

mod fonts;
mod pdf;

use std::path::{Path, PathBuf};

use eframe::egui::{self, Color32, FontId, Pos2, Sense, Stroke, StrokeKind, Ui, pos2, vec2};
use lineup_core::Project;
use lineup_core::lead_sheet::{
    self, Clef, LeadSheet, Mark, Options, Paper, Point, Transposition, default_melody_channel,
    melody_channels,
};
use serde::{Deserialize, Serialize};

use fonts::{Font, Fonts};

use crate::i18n::{tr, tr_name, trc, trf};
use crate::twelve_tone::Front;

/// Paper is white and ink is black, in dark mode too: the page is shown
/// as it will be printed.
const PAPER: Color32 = Color32::WHITE;
const INK: Color32 = Color32::BLACK;
/// The bar that plays.
const PLAYING: Color32 = Color32::from_rgb(255, 236, 153);
const ZOOMS: [f32; 7] = [0.5, 0.67, 0.8, 1.0, 1.25, 1.5, 2.0];
/// The room between two pages.
const PAGE_GAP: f32 = 16.0;

pub(crate) fn window_id() -> egui::Id {
    egui::Id::unique("lead-sheet-window")
}

fn settings_id() -> egui::Id {
    egui::Id::unique("lead-sheet-settings")
}

/// What the window remembers from one run of the app to the next.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
struct Settings {
    paper: Paper,
    zoom: f32,
    /// Who reads the sheet: a player keeps their instrument from song to
    /// song.
    transposition: Transposition,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            paper: Paper::A4,
            zoom: 1.0,
            transposition: Transposition::Concert,
        }
    }
}

/// How the window names who reads the sheet.
fn transposition_name(transposition: Transposition) -> &'static str {
    match transposition {
        Transposition::Concert => "Concert pitch",
        Transposition::BFlat => "Bb instruments",
        Transposition::EFlat => "Eb instruments",
        Transposition::F => "F instruments",
    }
}

/// Which instruments read it, and how it is written for them.
fn transposition_hint(transposition: Transposition) -> &'static str {
    match transposition {
        Transposition::Concert => {
            "As it sounds: piano, guitar, bass, voice, flute, violin, trombone"
        }
        Transposition::BFlat => {
            "Trumpet, clarinet, soprano and tenor sax: written a major second higher, \
             in the octave that fits the staff"
        }
        Transposition::EFlat => {
            "Alto and baritone sax: written a major sixth higher, \
             in the octave that fits the staff"
        }
        Transposition::F => {
            "Horn and English horn: written a perfect fifth higher, \
             in the octave that fits the staff"
        }
    }
}

/// The PDF's file name: for a transposing instrument, with who reads it
/// before the extension, "Song (Bb instruments).pdf".
fn part_file_name(file_name: &str, transposition: Transposition) -> String {
    if transposition == Transposition::Concert {
        return file_name.to_string();
    }
    let name = tr(transposition_name(transposition));
    match file_name.rsplit_once('.') {
        Some((stem, extension)) => format!("{stem} ({name}).{extension}"),
        None => format!("{file_name} ({name})"),
    }
}

/// Which channel's notes are the melody.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Melody {
    /// The channel called "Lead", or the highest one.
    Auto,
    Channel(usize),
    /// No melody: chords over rhythm slashes.
    None,
}

/// A lead sheet laid out, and what it was laid out from.
struct Laid {
    project: Project,
    options: Options,
    /// The language and note names it was written in.
    written_in: (crate::i18n::Language, lineup_core::NoteNaming),
    sheet: LeadSheet,
}

pub struct LeadSheetWindow {
    pub open: bool,
    settings: Settings,
    saved: Settings,
    melody: Melody,
    clef: Option<Clef>,
    fonts: Fonts,
    laid: Option<Laid>,
    /// The bar last scrolled to while playing.
    followed: Option<usize>,
    /// What happened to the last PDF.
    message: Option<String>,
    front: Front,
}

impl LeadSheetWindow {
    /// The window, closed, with the paper and zoom it had last time.
    pub fn new(ctx: &egui::Context) -> Self {
        fonts::install(ctx);
        let settings: Settings = ctx
            .data_mut(|data| data.get_persisted(settings_id()))
            .unwrap_or_default();
        LeadSheetWindow {
            open: false,
            saved: settings.clone(),
            settings,
            melody: Melody::Auto,
            clef: None,
            fonts: Fonts::load(),
            laid: None,
            followed: None,
            message: None,
            front: Front::default(),
        }
    }

    /// Draws the window, if open. `title` goes at the top of the sheet,
    /// `file_name` is offered for the PDF, and `playing` is the step of
    /// the timeline that plays, if the song is playing.
    pub fn show(
        &mut self,
        ctx: &egui::Context,
        area: egui::Rect,
        project: &Project,
        title: &str,
        file_name: &str,
        playing: Option<usize>,
    ) {
        if !self.open {
            // The layout is kept only while the window is open.
            self.laid = None;
            return;
        }
        self.front.keep(ctx, window_id());
        let mut open = true;
        let window = egui::Window::new(crate::i18n::tr("Lead sheet"))
            .id(window_id())
            .open(&mut open)
            .default_pos([240.0, 90.0])
            .min_width(420.0)
            .constrain_to(area)
            .resizable(true);
        let size = vec2(780.0, 760.0);
        crate::widgets::open_at(
            ctx,
            window,
            window_id(),
            size,
            crate::widgets::Opening::FirstTime,
        )
        .show(ctx, |ui| {
            let file_name = part_file_name(file_name, self.settings.transposition);
            self.contents(ui, project, title, &file_name, playing)
        });
        self.open = open;
        if self.settings != self.saved {
            self.saved = self.settings.clone();
            let settings = self.saved.clone();
            ctx.data_mut(|data| data.insert_persisted(settings_id(), settings));
        }
    }

    fn options(&self, project: &Project, title: &str) -> Options {
        let channels = melody_channels(project);
        let melody = match self.melody {
            Melody::Channel(channel) if channels.contains(&channel) => Some(channel),
            Melody::None => None,
            _ => default_melody_channel(project),
        };
        Options {
            title: title.to_string(),
            melody,
            clef: self.clef,
            paper: self.settings.paper,
            transposition: self.settings.transposition,
        }
    }

    /// Lays the sheet out again if the song or the choices changed.
    fn sheet(&mut self, project: &Project, title: &str) -> &LeadSheet {
        let options = self.options(project, title);
        let written_in = (
            crate::i18n::Language::current(),
            lineup_core::NoteNaming::current(),
        );
        let stale = self.laid.as_ref().is_none_or(|laid| {
            laid.options != options || laid.project != *project || laid.written_in != written_in
        });
        if stale {
            let sheet = lead_sheet::lead_sheet(project, &options, &self.fonts);
            self.laid = Some(Laid {
                project: project.clone(),
                options,
                written_in,
                sheet,
            });
        }
        &self.laid.as_ref().expect("laid out just now").sheet
    }

    fn contents(
        &mut self,
        ui: &mut Ui,
        project: &Project,
        title: &str,
        file_name: &str,
        playing: Option<usize>,
    ) {
        ui.horizontal_wrapped(|ui| {
            self.choices(ui, project);
            ui.separator();
            if ui
                .button(tr("Export PDF…"))
                .on_hover_text(tr("Save the lead sheet as a PDF file"))
                .clicked()
            {
                self.export(project, title, file_name);
            }
            if ui
                .button(tr("Print…"))
                .on_hover_text(tr(
                    "Open the lead sheet in your PDF viewer, to print it from there",
                ))
                .clicked()
            {
                self.print(project, title, file_name);
            }
        });
        if let Some(message) = &self.message {
            ui.label(egui::RichText::new(message).small().weak());
        }
        ui.separator();

        self.pages(ui, project, title, playing);
    }

    /// Draws only the pages in `ui`, for a video, scrolled to the bar
    /// that plays: `playing` is the step of the playlist's timeline.
    pub fn video(&mut self, ui: &mut Ui, project: &Project, title: &str, playing: Option<usize>) {
        // The pages as wide as the picture allows.
        let width = self
            .sheet(project, title)
            .pages
            .first()
            .map(|page| page.width);
        if let Some(width) = width {
            self.settings.zoom = ((ui.available_width() - 24.0) / width).clamp(0.5, 2.0);
        }
        self.pages(ui, project, title, playing);
    }

    /// The pages, with the bar that plays lit up and scrolled to.
    fn pages(&mut self, ui: &mut Ui, project: &Project, title: &str, playing: Option<usize>) {
        let zoom = self.settings.zoom;
        self.sheet(project, title);
        let Some(laid) = self.laid.take() else {
            return;
        };
        let sheet = &laid.sheet;
        let playing_bar = playing.and_then(|step| {
            sheet
                .bars
                .iter()
                .position(|bar| (bar.start..bar.end).contains(&step))
        });
        egui::ScrollArea::both()
            .auto_shrink([false, false])
            .show(ui, |ui| {
                // Pages narrower than the window sit in its middle.
                let room = ui.available_width();
                for (number, page) in sheet.pages.iter().enumerate() {
                    let size = vec2(page.width, page.height) * zoom;
                    let (row, _) =
                        ui.allocate_exact_size(vec2(room.max(size.x), size.y), Sense::hover());
                    let rect = egui::Rect::from_center_size(row.center(), size);
                    if ui.is_rect_visible(rect) {
                        let painter = ui.painter_at(rect.expand(2.0));
                        painter.rect_filled(rect, 0.0, PAPER);
                        painter.rect_stroke(
                            rect,
                            0.0,
                            Stroke::new(1.0, Color32::from_gray(150)),
                            StrokeKind::Outside,
                        );
                        if let Some(bar) = playing_bar.map(|bar| &sheet.bars[bar])
                            && bar.page == number
                        {
                            let at = |point: Point| rect.min + vec2(point.x, point.y) * zoom;
                            let lit =
                                egui::Rect::from_min_max(at(bar.top_left), at(bar.bottom_right));
                            painter.rect_filled(lit, 2.0, PLAYING);
                        }
                        for mark in &page.marks {
                            paint(&painter, rect.min, zoom, mark);
                        }
                    }
                    if let Some(bar) = playing_bar.map(|bar| &sheet.bars[bar])
                        && bar.page == number
                        && self.followed != playing_bar
                    {
                        let at = |point: Point| rect.min + vec2(point.x, point.y) * zoom;
                        let lit = egui::Rect::from_min_max(at(bar.top_left), at(bar.bottom_right));
                        ui.scroll_to_rect(lit, None);
                    }
                    ui.add_space(PAGE_GAP);
                }
            });
        self.followed = playing_bar;
        self.laid = Some(laid);
    }

    /// The melody, clef, instrument, paper and zoom pickers.
    fn choices(&mut self, ui: &mut Ui, project: &Project) {
        let channels = melody_channels(project);
        let name = |channel: usize| tr_name(&project.channels()[channel].name).into_owned();
        let auto = default_melody_channel(project).map_or(tr("none").to_string(), name);
        let shown = match self.melody {
            Melody::Auto => trf("Auto ({})", &[&auto]),
            Melody::Channel(channel) if channels.contains(&channel) => name(channel),
            Melody::Channel(_) => trf("Auto ({})", &[&auto]),
            Melody::None => tr("None: chords only").to_string(),
        };
        ui.label(tr("Melody"));
        egui::ComboBox::from_id_salt("lead-sheet-melody")
            .selected_text(shown)
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut self.melody, Melody::Auto, trf("Auto ({})", &[&auto]))
                    .on_hover_text(tr("The channel called Lead, or else the highest one"));
                for &channel in &channels {
                    ui.selectable_value(&mut self.melody, Melody::Channel(channel), name(channel));
                }
                ui.selectable_value(&mut self.melody, Melody::None, tr("None: chords only"))
                    .on_hover_text(tr("Write only the chords, over a slash for each beat"));
            });

        let chosen = self
            .laid
            .as_ref()
            .map_or(Clef::Treble, |laid| laid.sheet.clef);
        let clef_name = |clef: Clef| match clef {
            Clef::Treble => trc("clef", "Treble"),
            Clef::Bass => trc("clef", "Bass"),
        };
        ui.label(tr("Clef"));
        egui::ComboBox::from_id_salt("lead-sheet-clef")
            .selected_text(match self.clef {
                None => trf("Auto ({})", &[&clef_name(chosen).to_lowercase()]),
                Some(clef) => clef_name(clef).to_string(),
            })
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut self.clef, None, tr("Auto"))
                    .on_hover_text(tr("Bass clef for low tunes, treble clef for the others"));
                ui.selectable_value(&mut self.clef, Some(Clef::Treble), trc("clef", "Treble"));
                ui.selectable_value(&mut self.clef, Some(Clef::Bass), trc("clef", "Bass"));
            });

        ui.label(tr("For"));
        let transposition = &mut self.settings.transposition;
        egui::ComboBox::from_id_salt("lead-sheet-transposition")
            .selected_text(tr(transposition_name(*transposition)))
            .show_ui(ui, |ui| {
                for choice in Transposition::ALL {
                    ui.selectable_value(transposition, choice, tr(transposition_name(choice)))
                        .on_hover_text(tr(transposition_hint(choice)));
                }
            })
            .response
            .on_hover_text(tr(
                "Who reads it: players of Bb, Eb and F instruments read in keys of their own",
            ));

        ui.label(tr("Paper"));
        egui::ComboBox::from_id_salt("lead-sheet-paper")
            .selected_text(tr(self.settings.paper.name()))
            .show_ui(ui, |ui| {
                for paper in Paper::ALL {
                    ui.selectable_value(&mut self.settings.paper, paper, tr(paper.name()));
                }
            });

        let zoom = self.settings.zoom;
        let place = ZOOMS.iter().position(|&z| z >= zoom - 0.01).unwrap_or(3);
        if ui
            .add_enabled(place > 0, egui::Button::new("−"))
            .on_hover_text(tr("Zoom out"))
            .clicked()
        {
            self.settings.zoom = ZOOMS[place - 1];
        }
        ui.label(format!("{:.0}%", zoom * 100.0));
        if ui
            .add_enabled(place + 1 < ZOOMS.len(), egui::Button::new("+"))
            .on_hover_text(tr("Zoom in"))
            .clicked()
        {
            self.settings.zoom = ZOOMS[(place + 1).min(ZOOMS.len() - 1)];
        }
    }

    /// The PDF of the sheet, at its own size whatever the zoom on screen.
    fn pdf(&mut self, project: &Project, title: &str) -> Vec<u8> {
        self.sheet(project, title);
        let laid = self.laid.as_ref().expect("laid out just now");
        pdf::write(&laid.sheet, title, &self.fonts)
    }

    fn export(&mut self, project: &Project, title: &str, file_name: &str) {
        let Some(mut path) = rfd::FileDialog::new()
            .add_filter(tr("PDF document"), &["pdf"])
            .set_file_name(file_name)
            .save_file()
        else {
            return;
        };
        if path.extension().is_none() {
            path.set_extension("pdf");
        }
        let bytes = self.pdf(project, title);
        self.message = Some(match std::fs::write(&path, bytes) {
            Ok(()) => trf("Saved {}.", &[&shown_name(&path)]),
            Err(error) => trf("Could not save {}: {}.", &[&shown_name(&path), &error]),
        });
    }

    /// Writes the PDF to the temporary folder and opens it in the
    /// computer's PDF viewer, which prints it.
    fn print(&mut self, project: &Project, title: &str, file_name: &str) {
        let path = std::env::temp_dir().join(file_name);
        let bytes = self.pdf(project, title);
        self.message = Some(match std::fs::write(&path, bytes) {
            Ok(()) => match open_in_viewer(&path) {
                Ok(()) => tr("Opened in your PDF viewer: print it from there.").to_string(),
                Err(_) => trf(
                    "No PDF viewer opened. The file is at {}: open it to print.",
                    &[&path.display()],
                ),
            },
            Err(error) => trf("Could not write {}: {}.", &[&path.display(), &error]),
        });
    }
}

fn shown_name(path: &Path) -> String {
    path.file_name().map_or_else(
        || path.display().to_string(),
        |name| name.to_string_lossy().into_owned(),
    )
}

/// Opens `path` with the program the system uses for its kind of file.
fn open_in_viewer(path: &PathBuf) -> std::io::Result<()> {
    use std::process::Command;
    let mut command = if cfg!(target_os = "windows") {
        let mut command = Command::new("cmd");
        command.args(["/C", "start", ""]);
        command
    } else if cfg!(target_os = "macos") {
        Command::new("open")
    } else {
        Command::new("xdg-open")
    };
    let mut child = command.arg(path).spawn()?;
    // Waited for on the side, so the finished process is cleared away.
    std::thread::spawn(move || child.wait());
    Ok(())
}

/// Draws one mark of a page whose top left corner is at `origin`.
fn paint(painter: &egui::Painter, origin: Pos2, zoom: f32, mark: &Mark) {
    let at = |point: Point| origin + vec2(point.x, point.y) * zoom;
    match mark {
        Mark::Music {
            glyph,
            at: point,
            size,
        } => text_at(
            painter,
            glyph.code.to_string(),
            Font::Music,
            size * zoom,
            at(*point),
        ),
        Mark::Text {
            text,
            font,
            at: point,
            size,
        } => text_at(
            painter,
            text.clone(),
            Font::Text(*font),
            size * zoom,
            at(*point),
        ),
        Mark::Line { from, to, width } => {
            painter.line_segment([at(*from), at(*to)], Stroke::new(width * zoom, INK));
        }
        Mark::Beam {
            from,
            to,
            thickness,
        } => {
            let corners = Mark::beam_corners(*from, *to, *thickness).map(at).to_vec();
            painter.add(egui::Shape::convex_polygon(corners, INK, Stroke::NONE));
        }
        Mark::Tie {
            from,
            to,
            up,
            thickness,
        } => {
            let [outer, inner] = Mark::tie_curves(*from, *to, *up, *thickness);
            painter.add(tie_mesh(outer, inner, &at));
        }
    }
}

/// Draws `text` in `font` with the left end of its baseline at `baseline`.
fn text_at(painter: &egui::Painter, text: String, font: Font, size: f32, baseline: Pos2) {
    if size < 1.0 {
        return;
    }
    let galley = painter.layout_no_wrap(text, FontId::new(size, font.family()), INK);
    let Some(row) = galley.rows.first() else {
        return;
    };
    let (dx, dy) = row.glyphs.first().map_or((0.0, size * 0.8), |glyph| {
        (row.pos.x + glyph.pos.x, row.pos.y + glyph.pos.y)
    });
    painter.galley(pos2(baseline.x - dx, baseline.y - dy), galley, INK);
}

/// A point of a cubic Bézier curve.
fn bezier(curve: [Point; 4], t: f32) -> Point {
    let u = 1.0 - t;
    let weights = [u * u * u, 3.0 * u * u * t, 3.0 * u * t * t, t * t * t];
    let mut point = Point::default();
    for (control, weight) in curve.iter().zip(weights) {
        point.x += control.x * weight;
        point.y += control.y * weight;
    }
    point
}

/// A tie as triangles between its outer and inner curves: egui fills
/// only convex shapes, and a tie is a thin crescent.
fn tie_mesh(outer: [Point; 4], inner: [Point; 4], at: &dyn Fn(Point) -> Pos2) -> egui::Mesh {
    const STEPS: u32 = 16;
    let mut mesh = egui::Mesh::default();
    for step in 0..=STEPS {
        let t = step as f32 / STEPS as f32;
        mesh.colored_vertex(at(bezier(outer, t)), INK);
        // The inner curve runs back, so it is read the other way round.
        mesh.colored_vertex(at(bezier(inner, 1.0 - t)), INK);
    }
    for step in 0..STEPS {
        let i = step * 2;
        mesh.add_triangle(i, i + 1, i + 2);
        mesh.add_triangle(i + 1, i + 3, i + 2);
    }
    mesh
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_part_is_saved_under_a_name_of_its_own() {
        assert_eq!(
            part_file_name("Op. 24.pdf", Transposition::Concert),
            "Op. 24.pdf"
        );
        assert_eq!(
            part_file_name("Op. 24.pdf", Transposition::BFlat),
            "Op. 24 (Bb instruments).pdf"
        );
        assert_eq!(
            part_file_name("Malena", Transposition::F),
            "Malena (F instruments)"
        );
    }
}
