//! Lead sheets as PDF files, to print or to share.
//!
//! A PDF page is a list of drawing operations, much like the lead
//! sheet's own marks: lines, filled shapes, and text. Rather than
//! embedding the fonts as fonts, each symbol and letter is copied in as
//! its outline (the curves that draw it, read from the font file) and
//! filled. A shape used many times, such as a black notehead, is written
//! once as a "form XObject" and then placed wherever it appears, so the
//! file stays small. The PDF then looks the same in every viewer.
//!
//! PDF counts y upwards from the bottom of the page, the lead sheet
//! counts it downwards from the top, so every y is turned round:
//! `height - y`.

use std::collections::HashMap;

use lineup_core::lead_sheet::{LeadSheet, Mark, Point};
use pdf_writer::{Content, Finish, Name, Pdf, Rect, Ref, TextStr};
use ttf_parser::{Face, GlyphId, OutlineBuilder};

use super::fonts::{Font, Fonts};

/// Writes `sheet` as a PDF file and returns its bytes.
pub fn write(sheet: &LeadSheet, title: &str, fonts: &Fonts) -> Vec<u8> {
    let mut pdf = Pdf::new();
    let mut next = Ref::new(1);
    let mut new_ref = || next.bump();
    let catalog = new_ref();
    let tree = new_ref();
    let info = new_ref();

    // Each glyph used, once, as a form XObject in font units.
    let mut shapes: HashMap<(Font, u16), (Ref, String)> = HashMap::new();
    let mut page_refs = Vec::new();
    let mut pages = Vec::new();
    for page in &sheet.pages {
        let mut content = Content::new();
        let mut used: Vec<(String, Ref)> = Vec::new();
        let height = page.height;
        let flip = |point: Point| (point.x, height - point.y);
        for mark in &page.marks {
            match mark {
                Mark::Music { glyph, at, size } => {
                    let text = glyph.code.to_string();
                    place(
                        &text,
                        Font::Music,
                        *at,
                        *size,
                        height,
                        fonts,
                        &mut shapes,
                        &mut new_ref,
                        &mut used,
                        &mut content,
                    );
                }
                Mark::Text {
                    text,
                    font,
                    at,
                    size,
                } => {
                    place(
                        text,
                        Font::Text(*font),
                        *at,
                        *size,
                        height,
                        fonts,
                        &mut shapes,
                        &mut new_ref,
                        &mut used,
                        &mut content,
                    );
                }
                Mark::Line { from, to, width } => {
                    let (x1, y1) = flip(*from);
                    let (x2, y2) = flip(*to);
                    content
                        .set_line_width(*width)
                        .move_to(x1, y1)
                        .line_to(x2, y2)
                        .stroke();
                }
                Mark::Beam {
                    from,
                    to,
                    thickness,
                } => {
                    let corners = Mark::beam_corners(*from, *to, *thickness);
                    let (x, y) = flip(corners[0]);
                    content.move_to(x, y);
                    for corner in &corners[1..] {
                        let (x, y) = flip(*corner);
                        content.line_to(x, y);
                    }
                    content.close_path().fill_nonzero();
                }
                Mark::Tie {
                    from,
                    to,
                    up,
                    thickness,
                } => {
                    let [outer, inner] = Mark::tie_curves(*from, *to, *up, *thickness);
                    let (x, y) = flip(outer[0]);
                    content.move_to(x, y);
                    for curve in [outer, inner] {
                        let (x1, y1) = flip(curve[1]);
                        let (x2, y2) = flip(curve[2]);
                        let (x3, y3) = flip(curve[3]);
                        content.cubic_to(x1, y1, x2, y2, x3, y3);
                    }
                    content.close_path().fill_nonzero();
                }
            }
        }
        let content_ref = new_ref();
        let page_ref = new_ref();
        page_refs.push(page_ref);
        pages.push((
            page_ref,
            content_ref,
            content.finish(),
            used,
            page.width,
            height,
        ));
    }

    pdf.catalog(catalog).pages(tree);
    pdf.pages(tree)
        .kids(page_refs.iter().copied())
        .count(page_refs.len() as i32);
    for (page_ref, content_ref, content, used, width, height) in &pages {
        let mut page = pdf.page(*page_ref);
        page.media_box(Rect::new(0.0, 0.0, *width, *height));
        page.parent(tree);
        page.contents(*content_ref);
        let mut resources = page.resources();
        let mut objects = resources.x_objects();
        for (name, shape) in used {
            objects.pair(Name(name.as_bytes()), *shape);
        }
        objects.finish();
        resources.finish();
        page.finish();
        pdf.stream(*content_ref, content);
    }
    for ((font, glyph), (shape, _)) in &shapes {
        let face = fonts.face(*font);
        let mut outline = Outline::default();
        let bounds = face.outline_glyph(GlyphId(*glyph), &mut outline);
        outline.fill();
        let content = outline.content.finish();
        let mut form = pdf.form_xobject(*shape, &content);
        let rect = bounds.map_or(Rect::new(0.0, 0.0, 0.0, 0.0), |b| {
            Rect::new(
                f32::from(b.x_min),
                f32::from(b.y_min),
                f32::from(b.x_max),
                f32::from(b.y_max),
            )
        });
        form.bbox(rect);
        form.finish();
    }
    pdf.document_info(info)
        .title(TextStr(title))
        .producer(TextStr("Ferrous Lineup"));
    pdf.finish()
}

/// Places the glyphs of `text` with the left end of their baseline at
/// `at`, each as a scaled copy of its shape.
#[allow(clippy::too_many_arguments)]
fn place(
    text: &str,
    font: Font,
    at: Point,
    size: f32,
    height: f32,
    fonts: &Fonts,
    shapes: &mut HashMap<(Font, u16), (Ref, String)>,
    new_ref: &mut dyn FnMut() -> Ref,
    used: &mut Vec<(String, Ref)>,
    content: &mut Content,
) {
    let face: &Face = fonts.face(font);
    let scale = size / f32::from(face.units_per_em());
    let mut x = at.x;
    for c in text.chars() {
        let Some(glyph) = face.glyph_index(c) else {
            continue;
        };
        let advance = f32::from(face.glyph_hor_advance(glyph).unwrap_or(0)) * scale;
        if face.glyph_bounding_box(glyph).is_some() {
            let count = shapes.len();
            let (shape, name) = shapes
                .entry((font, glyph.0))
                .or_insert_with(|| (new_ref(), format!("G{count}")))
                .clone();
            if !used.iter().any(|(used, _)| *used == name) {
                used.push((name.clone(), shape));
            }
            content
                .save_state()
                .transform([scale, 0.0, 0.0, scale, x, height - at.y])
                .x_object(Name(name.as_bytes()))
                .restore_state();
        }
        x += advance;
    }
}

/// A glyph's outline as PDF path operations, in font units.
struct Outline {
    content: Content,
    /// The current point, to turn quadratic curves into cubic ones.
    at: (f32, f32),
    started: bool,
}

impl Default for Outline {
    fn default() -> Self {
        Outline {
            content: Content::new(),
            at: (0.0, 0.0),
            started: false,
        }
    }
}

impl OutlineBuilder for Outline {
    fn move_to(&mut self, x: f32, y: f32) {
        self.content.move_to(x, y);
        self.at = (x, y);
        self.started = true;
    }

    fn line_to(&mut self, x: f32, y: f32) {
        self.content.line_to(x, y);
        self.at = (x, y);
    }

    fn quad_to(&mut self, x1: f32, y1: f32, x: f32, y: f32) {
        let (x0, y0) = self.at;
        let c1 = (x0 + 2.0 / 3.0 * (x1 - x0), y0 + 2.0 / 3.0 * (y1 - y0));
        let c2 = (x + 2.0 / 3.0 * (x1 - x), y + 2.0 / 3.0 * (y1 - y));
        self.content.cubic_to(c1.0, c1.1, c2.0, c2.1, x, y);
        self.at = (x, y);
    }

    fn curve_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32) {
        self.content.cubic_to(x1, y1, x2, y2, x, y);
        self.at = (x, y);
    }

    fn close(&mut self) {
        if self.started {
            self.content.close_path();
        }
    }
}

impl Outline {
    /// Ends the path with a fill. Called once the glyph is outlined.
    fn fill(&mut self) {
        if self.started {
            self.content.fill_nonzero();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use lineup_core::lead_sheet::{Options, Paper, default_melody_channel, lead_sheet};
    use lineup_core::library::demo::song_demo;
    use lineup_core::library::song::Song;

    fn count(bytes: &[u8], pattern: &str) -> usize {
        bytes
            .windows(pattern.len())
            .filter(|window| *window == pattern.as_bytes())
            .count()
    }

    #[test]
    fn a_song_becomes_a_pdf_with_a_page_per_page() {
        let fonts = Fonts::load();
        let project = song_demo(Song::named("El Choclo").unwrap());
        let options = Options {
            title: "El Choclo".to_string(),
            melody: default_melody_channel(&project),
            clef: None,
            paper: Paper::Letter,
            transposition: Default::default(),
        };
        let sheet = lead_sheet(&project, &options, &fonts);
        assert!(sheet.pages.len() >= 2);
        let bytes = write(&sheet, "El Choclo", &fonts);
        assert!(bytes.starts_with(b"%PDF-"));
        assert_eq!(count(&bytes, "/Type /Page\n"), sheet.pages.len());
        assert_eq!(count(&bytes, "/MediaBox [0 0 612 792]"), sheet.pages.len());
        // Every glyph is written once and placed many times.
        let shapes = count(&bytes, "/Subtype /Form");
        let placed = count(&bytes, " Do");
        assert!(
            shapes > 20 && placed > shapes * 5,
            "{shapes} shapes, {placed} placed"
        );
        assert!(count(&bytes, "/Title (El Choclo)") == 1);
    }
}
