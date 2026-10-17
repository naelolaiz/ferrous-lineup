//! The fonts a lead sheet is drawn with, bundled with the app so it looks
//! the same on every computer, on screen and on paper.
//!
//! - **Bravura**, by Steinberg, for the music: clefs, notes, rests,
//!   accidentals. It is the reference font of SMuFL, the standard that
//!   gives every music symbol its code point.
//! - **Inter**, by Rasmus Andersson, for the chord symbols and the words.
//!
//! Both are free fonts under the SIL Open Font License; their licenses
//! are next to them in `assets/fonts`. Bravura's license reserves its
//! name, so the file is shipped exactly as published.
//!
//! The same font files are read twice: by egui, to draw on screen, and
//! by `ttf-parser`, to measure text for the layout and to copy the
//! outlines of the letters into PDF files.

use eframe::egui;
use lineup_core::lead_sheet::{TextFont, TextWidth};
use ttf_parser::Face;

const BRAVURA: &[u8] = include_bytes!("../../assets/fonts/Bravura.otf");
const INTER_SEMIBOLD: &[u8] = include_bytes!("../../assets/fonts/Inter-SemiBold.otf");
const INTER_REGULAR: &[u8] = include_bytes!("../../assets/fonts/Inter-Regular.otf");

/// The names the fonts get in egui.
const MUSIC_NAME: &str = "Lead sheet music";
const SEMIBOLD_NAME: &str = "Lead sheet semibold";
const REGULAR_NAME: &str = "Lead sheet regular";

/// One of the three fonts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Font {
    Music,
    Text(TextFont),
}

impl Font {
    pub const ALL: [Font; 3] = [
        Font::Music,
        Font::Text(TextFont::SemiBold),
        Font::Text(TextFont::Regular),
    ];

    fn bytes(self) -> &'static [u8] {
        match self {
            Font::Music => BRAVURA,
            Font::Text(TextFont::SemiBold) => INTER_SEMIBOLD,
            Font::Text(TextFont::Regular) => INTER_REGULAR,
        }
    }

    fn name(self) -> &'static str {
        match self {
            Font::Music => MUSIC_NAME,
            Font::Text(TextFont::SemiBold) => SEMIBOLD_NAME,
            Font::Text(TextFont::Regular) => REGULAR_NAME,
        }
    }

    /// The egui font family that draws with it, and only with it.
    pub fn family(self) -> egui::FontFamily {
        egui::FontFamily::Name(self.name().into())
    }
}

/// Gives egui the three fonts, each as a family of its own. Called once,
/// when the app starts: a family must exist before anything is drawn
/// with it.
pub fn install(ctx: &egui::Context) {
    for font in Font::ALL {
        ctx.add_font(egui::FontInsert::new(
            font.name(),
            egui::FontData::from_static(font.bytes()),
            vec![egui::InsertFontFamily {
                family: font.family(),
                priority: egui::FontPriority::Highest,
            }],
        ));
    }
}

/// The fonts read with `ttf-parser`.
pub struct Fonts {
    music: Face<'static>,
    semibold: Face<'static>,
    regular: Face<'static>,
}

impl Fonts {
    pub fn load() -> Self {
        let face = |font: Font| Face::parse(font.bytes(), 0).expect("the bundled fonts are valid");
        Fonts {
            music: face(Font::Music),
            semibold: face(Font::Text(TextFont::SemiBold)),
            regular: face(Font::Text(TextFont::Regular)),
        }
    }

    pub fn face(&self, font: Font) -> &Face<'static> {
        match font {
            Font::Music => &self.music,
            Font::Text(TextFont::SemiBold) => &self.semibold,
            Font::Text(TextFont::Regular) => &self.regular,
        }
    }
}

impl TextWidth for Fonts {
    fn text_width(&self, text: &str, font: TextFont, size: f32) -> f32 {
        let face = self.face(Font::Text(font));
        let units: u32 = text
            .chars()
            .filter_map(|c| face.glyph_index(c))
            .filter_map(|glyph| face.glyph_hor_advance(glyph))
            .map(u32::from)
            .sum();
        units as f32 * size / f32::from(face.units_per_em())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use lineup_core::lead_sheet::glyphs;

    #[test]
    fn the_music_font_has_every_symbol_at_its_size() {
        let fonts = Fonts::load();
        let face = fonts.face(Font::Music);
        let em = f32::from(face.units_per_em());
        for glyph in glyphs::ALL {
            let id = face
                .glyph_index(glyph.code)
                .unwrap_or_else(|| panic!("U+{:04X} is missing", glyph.code as u32));
            // The widths the layout uses are Bravura's own, in staff
            // spaces: a quarter of an em.
            let advance = f32::from(face.glyph_hor_advance(id).unwrap()) / em * 4.0;
            let bounds = face.glyph_bounding_box(id).unwrap();
            let top = f32::from(bounds.y_max) / em * 4.0;
            let bottom = f32::from(bounds.y_min) / em * 4.0;
            let code = glyph.code as u32;
            assert!(
                (top - glyph.top).abs() < 0.05 && (bottom - glyph.bottom).abs() < 0.05,
                "U+{code:04X}: {bottom}..{top}, not {}..{}",
                glyph.bottom,
                glyph.top
            );
            // Some symbols advance by less than they are wide.
            assert!(advance <= glyph.width + 0.05, "U+{code:04X}: {advance}");
        }
    }

    #[test]
    fn text_is_measured_with_the_font() {
        let fonts = Fonts::load();
        let narrow = fonts.text_width("i", TextFont::Regular, 10.0);
        let wide = fonts.text_width("M", TextFont::Regular, 10.0);
        assert!(narrow > 1.0 && wide > narrow * 2.0);
        assert!(
            fonts.text_width("Am7", TextFont::SemiBold, 20.0)
                > fonts.text_width("Am7", TextFont::Regular, 20.0)
        );
    }
}
