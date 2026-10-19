//! Painting egui's pictures in memory, without a graphics card.
//!
//! On screen, egui turns everything it draws (text, buttons, lines) into
//! triangles, and the graphics card paints them. Here the same triangles
//! are painted pixel by pixel into a picture in memory, which becomes a
//! frame of the video. The rules are the graphics card's, as egui's own
//! painter sets them up:
//!
//! - Each corner of a triangle has a colour, and a point in a texture
//!   (the font picture, for text). A pixel inside the triangle gets a mix
//!   of the corners' colours, weighted by how close it is to each, times
//!   the texture's colour at the mixed point.
//! - Colours are "premultiplied": red, green and blue are already scaled
//!   by how opaque the colour is. Painting a colour over a pixel is then
//!   `new + old × (1 − new opacity)`.
//! - Only pixels inside the triangle's clip rectangle are painted.
//!
//! Smooth edges come from egui itself: it adds a thin band of fading
//! triangles around every shape.

use std::collections::HashMap;
use std::thread;

use epaint::textures::{TextureFilter, TexturesDelta};
use epaint::{ClippedPrimitive, Color32, ImageData, Mesh, Primitive, Rect, TextureId};

/// A texture egui asked for: the font picture, or an image.
struct Texture {
    width: usize,
    height: usize,
    pixels: Vec<Color32>,
    smooth: bool,
}

impl Texture {
    /// The colour at `(u, v)`, from 0 to 1 across the texture, as egui's
    /// painter samples it: mixed from the four nearest texels.
    fn sample(&self, u: f32, v: f32) -> [f32; 4] {
        let texel = |x: isize, y: isize| {
            let x = x.clamp(0, self.width as isize - 1) as usize;
            let y = y.clamp(0, self.height as isize - 1) as usize;
            self.pixels[y * self.width + x].to_array()
        };
        let x = u * self.width as f32;
        let y = v * self.height as f32;
        if !self.smooth {
            return texel(x.floor() as isize, y.floor() as isize).map(f32::from);
        }
        // Texel centres are at half-way points.
        let (x, y) = (x - 0.5, y - 0.5);
        let (left, top) = (x.floor(), y.floor());
        let (fx, fy) = (x - left, y - top);
        let (left, top) = (left as isize, top as isize);
        let [a, b, c, d] = [
            texel(left, top),
            texel(left + 1, top),
            texel(left, top + 1),
            texel(left + 1, top + 1),
        ];
        std::array::from_fn(|i| {
            let upper = f32::from(a[i]) + (f32::from(b[i]) - f32::from(a[i])) * fx;
            let lower = f32::from(c[i]) + (f32::from(d[i]) - f32::from(c[i])) * fx;
            upper + (lower - upper) * fy
        })
    }
}

/// A picture to paint egui's shapes on.
pub struct Canvas {
    width: usize,
    height: usize,
    /// The pixels, row by row: red, green, blue and opacity.
    pixels: Vec<[u8; 4]>,
    textures: HashMap<TextureId, Texture>,
    /// What was painted last, to notice when nothing changed.
    painted: Vec<ClippedPrimitive>,
    background: Color32,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        Canvas {
            width,
            height,
            pixels: vec![[0, 0, 0, 255]; width * height],
            textures: HashMap::new(),
            painted: Vec::new(),
            background: Color32::BLACK,
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    /// The pixels, four bytes each (red, green, blue, opacity), row by row.
    pub fn rgba(&self) -> &[u8] {
        self.pixels.as_flattened()
    }

    /// The colour of the pixel at `(x, y)`.
    pub fn pixel(&self, x: usize, y: usize) -> Color32 {
        let [r, g, b, a] = self.pixels[y * self.width + x];
        Color32::from_rgba_premultiplied(r, g, b, a)
    }

    /// Takes in the new and changed textures egui made this frame: the
    /// font picture grows as new letters and sizes are needed.
    fn set_textures(&mut self, delta: &mut TexturesDelta) {
        for (id, deltas) in delta.set.drain() {
            for delta in deltas {
                let ImageData::Color(image) = &delta.image;
                let [width, height] = image.size;
                let smooth = delta.options.magnification == TextureFilter::Linear;
                match delta.pos {
                    None => {
                        self.textures.insert(
                            id,
                            Texture {
                                width,
                                height,
                                pixels: image.pixels.clone(),
                                smooth,
                            },
                        );
                    }
                    Some([left, top]) => {
                        let Some(texture) = self.textures.get_mut(&id) else {
                            continue;
                        };
                        for row in 0..height {
                            let start = (top + row) * texture.width + left;
                            texture.pixels[start..start + width]
                                .copy_from_slice(&image.pixels[row * width..(row + 1) * width]);
                        }
                    }
                }
            }
        }
    }

    /// Fills the picture with `background`, then paints `primitives` on
    /// it. Their positions are in points, `pixels_per_point` pixels each.
    /// `textures` are the textures egui made, changed or let go of since
    /// the last picture; they are taken in, as a graphics card would.
    ///
    /// Returns whether the picture changed. Most of a song's pictures are
    /// the same as the one before (the playhead moves a few times a
    /// second, a video shows thirty pictures), and those are not painted
    /// again.
    pub fn paint(
        &mut self,
        primitives: Vec<ClippedPrimitive>,
        textures: &mut TexturesDelta,
        pixels_per_point: f32,
        background: Color32,
    ) -> bool {
        let new_textures = !textures.set.is_empty();
        self.set_textures(textures);
        let changed =
            new_textures || background != self.background || !same(&primitives, &self.painted);
        if changed {
            self.paint_primitives(&primitives, pixels_per_point, background);
            self.painted = primitives;
            self.background = background;
        }
        for id in textures.free.drain() {
            self.textures.remove(&id);
        }
        changed
    }

    /// Fills the picture, then paints `primitives` on it, in bands of
    /// rows painted at the same time on the computer's cores.
    fn paint_primitives(
        &mut self,
        primitives: &[ClippedPrimitive],
        pixels_per_point: f32,
        background: Color32,
    ) {
        self.pixels.fill(background.to_array());
        let width = self.width;
        let cores = thread::available_parallelism().map_or(1, |n| n.get().min(8));
        let rows_per_band = self.height.div_ceil(cores).max(1);
        let textures = &self.textures;
        thread::scope(|scope| {
            for (band, pixels) in self.pixels.chunks_mut(rows_per_band * width).enumerate() {
                let top = band * rows_per_band;
                let bottom = top + pixels.len() / width;
                scope.spawn(move || {
                    let mut band = Band {
                        pixels,
                        width,
                        top,
                        bottom,
                    };
                    for clipped in primitives {
                        if let Primitive::Mesh(mesh) = &clipped.primitive {
                            band.paint_mesh(mesh, clipped.clip_rect, pixels_per_point, textures);
                        }
                    }
                });
            }
        });
    }
}

/// Whether two lists of shapes paint the same picture.
fn same(new: &[ClippedPrimitive], old: &[ClippedPrimitive]) -> bool {
    new.len() == old.len()
        && new.iter().zip(old).all(|(new, old)| {
            new.clip_rect == old.clip_rect
                && match (&new.primitive, &old.primitive) {
                    (Primitive::Mesh(new), Primitive::Mesh(old)) => new == old,
                    _ => false,
                }
        })
}

/// Some rows of the picture, painted by one thread.
struct Band<'a> {
    pixels: &'a mut [[u8; 4]],
    width: usize,
    /// The rows of the whole picture it holds.
    top: usize,
    bottom: usize,
}

/// A corner of a triangle, in pixels.
#[derive(Clone, Copy)]
struct Corner {
    x: f32,
    y: f32,
    u: f32,
    v: f32,
    color: [f32; 4],
}

impl Band<'_> {
    fn paint_mesh(
        &mut self,
        mesh: &Mesh,
        clip: Rect,
        pixels_per_point: f32,
        textures: &HashMap<TextureId, Texture>,
    ) {
        let Some(texture) = textures.get(&mesh.texture_id) else {
            return;
        };
        // The clip rectangle in whole pixels, rounded as the graphics
        // card's scissor rectangle is.
        let clip_x = |x: f32| ((x * pixels_per_point).round().max(0.0) as usize).min(self.width);
        let clip_y = |y: f32| (y * pixels_per_point).round().max(0.0) as usize;
        let left = clip_x(clip.min.x);
        let right = clip_x(clip.max.x);
        let top = clip_y(clip.min.y).max(self.top);
        let bottom = clip_y(clip.max.y).min(self.bottom);
        if left >= right || top >= bottom {
            return;
        }
        let area = (left, right, top, bottom);
        let corner = |index: u32| {
            let vertex = &mesh.vertices[index as usize];
            Corner {
                x: vertex.pos.x * pixels_per_point,
                y: vertex.pos.y * pixels_per_point,
                u: vertex.uv.x,
                v: vertex.uv.y,
                color: vertex.color.to_array().map(f32::from),
            }
        };
        for &triangle in mesh.indices.as_chunks::<3>().0 {
            self.paint_triangle(triangle.map(corner), area, texture);
        }
    }

    fn paint_triangle(
        &mut self,
        corners: [Corner; 3],
        (left, right, top, bottom): (usize, usize, usize, usize),
        texture: &Texture,
    ) {
        // Positions in 1/256 of a pixel, as whole numbers: two triangles
        // sharing an edge then agree exactly on which pixels are whose.
        let fixed = |corner: Corner| {
            (
                (corner.x * 256.0).round() as i64,
                (corner.y * 256.0).round() as i64,
            )
        };
        let [a, mut b, mut c] = corners;
        let mut area = edge(fixed(a), fixed(b), fixed(c));
        if area == 0 {
            return;
        }
        // egui's triangles go both ways round; make them all go one way.
        if area < 0 {
            std::mem::swap(&mut b, &mut c);
            area = -area;
        }
        let [pa, pb, pc] = [a, b, c].map(fixed);

        let min_x = a.x.min(b.x).min(c.x).floor().max(left as f32) as usize;
        let max_x = (a.x.max(b.x).max(c.x).ceil() as usize).min(right);
        let min_y = a.y.min(b.y).min(c.y).floor().max(top as f32) as usize;
        let max_y = (a.y.max(b.y).max(c.y).ceil() as usize).min(bottom);
        if min_x >= max_x || min_y >= max_y {
            return;
        }

        // Each edge's "inside" number for the first pixel's centre, and
        // how it changes one pixel to the right and one down. A pixel is
        // inside where all three are positive. Exactly on an edge, it
        // belongs to the triangle only if the edge is a top or a left
        // one, so where two triangles meet each pixel is painted once:
        // the others lose 1 (`bias`).
        let edges = [(pb, pc), (pc, pa), (pa, pb)].map(|(from, to)| {
            let (dx, dy) = (to.0 - from.0, to.1 - from.1);
            let bias = if dy < 0 || (dy == 0 && dx > 0) { 0 } else { -1 };
            let first = ((min_x as i64) * 256 + 128, (min_y as i64) * 256 + 128);
            EdgeStep {
                value: edge(from, to, first) + bias,
                step_x: -dy * 256,
                step_y: dx * 256,
            }
        });

        // Plain shapes sample the font picture's white corner: their
        // colour needs no texture, and is often the same at all corners.
        let plain = a.u == b.u && a.u == c.u && a.v == b.v && a.v == c.v && {
            let sample = texture.sample(a.u, a.v);
            sample.iter().all(|&channel| channel >= 254.5)
        };
        if plain && a.color == b.color && a.color == c.color {
            let color = a.color.map(|channel| channel.round() as u8);
            if color[3] == 0 && color == [0; 4] {
                return;
            }
            self.fill(edges, (min_x, max_x, min_y, max_y), color);
            return;
        }

        let area = area as f32;
        let [ca, cb, cc] = [a.color, b.color, c.color].map(|color| color.map(|c| c / 255.0));
        let mut rows = edges;
        for y in min_y..max_y {
            let row = (y - self.top) * self.width;
            let mut weights = rows.map(|edge| edge.value);
            for x in min_x..max_x {
                if weights.iter().all(|&w| w >= 0) {
                    // How close the pixel is to each corner. (The bias is
                    // a 65 536th of a pixel: too little to matter here.)
                    let [wa, wb, wc] = weights.map(|w| w as f32 / area);
                    let mut color: [f32; 4] =
                        std::array::from_fn(|i| ca[i] * wa + cb[i] * wb + cc[i] * wc);
                    if plain {
                        for channel in &mut color {
                            *channel *= 255.0;
                        }
                    } else {
                        let u = a.u * wa + b.u * wb + c.u * wc;
                        let v = a.v * wa + b.v * wb + c.v * wc;
                        let sample = texture.sample(u, v);
                        for (channel, texel) in color.iter_mut().zip(sample) {
                            *channel *= texel;
                        }
                    }
                    blend(&mut self.pixels[row + x], color);
                }
                for (w, edge) in weights.iter_mut().zip(&rows) {
                    *w += edge.step_x;
                }
            }
            for edge in &mut rows {
                edge.value += edge.step_y;
            }
        }
    }

    /// Paints the pixels inside `edges` with one colour: most of what
    /// egui draws, from panels to the bars of a chord sheet.
    fn fill(
        &mut self,
        edges: [EdgeStep; 3],
        (min_x, max_x, min_y, max_y): (usize, usize, usize, usize),
        color: [u8; 4],
    ) {
        let opaque = color[3] == 255;
        let keep = 255 - u32::from(color[3]);
        let mut rows = edges;
        for y in min_y..max_y {
            let row = (y - self.top) * self.width;
            let mut weights = rows.map(|edge| edge.value);
            for x in min_x..max_x {
                if weights[0] >= 0 && weights[1] >= 0 && weights[2] >= 0 {
                    let pixel = &mut self.pixels[row + x];
                    if opaque {
                        *pixel = color;
                    } else {
                        for (old, new) in pixel.iter_mut().zip(color) {
                            *old = (u32::from(new) + (u32::from(*old) * keep + 127) / 255).min(255)
                                as u8;
                        }
                    }
                }
                weights[0] += rows[0].step_x;
                weights[1] += rows[1].step_x;
                weights[2] += rows[2].step_x;
            }
            for edge in &mut rows {
                edge.value += edge.step_y;
            }
        }
    }
}

/// An edge's "inside" number at the start of a row, and how it changes.
#[derive(Clone, Copy)]
struct EdgeStep {
    value: i64,
    step_x: i64,
    step_y: i64,
}

/// Twice the area of the triangle `from`, `to`, `p`: positive when `p`
/// is on the inside of the edge from `from` to `to`, zero on it.
fn edge(from: (i64, i64), to: (i64, i64), p: (i64, i64)) -> i64 {
    (to.0 - from.0) * (p.1 - from.1) - (to.1 - from.1) * (p.0 - from.0)
}

/// Paints a premultiplied colour (0 to 255 a channel) over a pixel.
fn blend(pixel: &mut [u8; 4], color: [f32; 4]) {
    let keep = 1.0 - color[3] / 255.0;
    for (old, new) in pixel.iter_mut().zip(color) {
        *old = (new + f32::from(*old) * keep).round().clamp(0.0, 255.0) as u8;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use epaint::textures::TextureOptions;
    use epaint::{ColorImage, ImageDelta, Pos2, Vec2, pos2};

    /// A canvas with egui's usual white texture for plain shapes.
    fn canvas(width: usize, height: usize) -> Canvas {
        let mut canvas = Canvas::new(width, height);
        let mut delta = TexturesDelta::default();
        delta.set.insert(
            TextureId::default(),
            [ImageDelta::full(
                ColorImage::filled([4, 4], Color32::WHITE),
                TextureOptions::LINEAR,
            )]
            .into(),
        );
        canvas.set_textures(&mut delta);
        canvas
    }

    fn rect_mesh(rect: Rect, color: Color32) -> ClippedPrimitive {
        let mut mesh = Mesh::default();
        mesh.add_colored_rect(rect, color);
        ClippedPrimitive {
            clip_rect: Rect::EVERYTHING,
            primitive: Primitive::Mesh(mesh),
        }
    }

    #[test]
    fn paints_a_rectangle_on_whole_pixels() {
        let mut canvas = canvas(8, 8);
        let rect = Rect::from_min_size(pos2(2.0, 2.0), Vec2::splat(4.0));
        canvas.paint_primitives(&[rect_mesh(rect, Color32::RED)], 1.0, Color32::BLACK);
        assert_eq!(canvas.pixel(2, 2), Color32::RED);
        assert_eq!(canvas.pixel(5, 5), Color32::RED);
        assert_eq!(canvas.pixel(1, 2), Color32::BLACK);
        assert_eq!(canvas.pixel(6, 5), Color32::BLACK);
    }

    #[test]
    fn half_see_through_colours_mix_and_the_diagonal_is_painted_once() {
        let mut canvas = canvas(8, 8);
        let half_white = Color32::from_white_alpha(128);
        let rect = Rect::from_min_size(Pos2::ZERO, Vec2::splat(8.0));
        canvas.paint_primitives(&[rect_mesh(rect, half_white)], 1.0, Color32::BLACK);
        // Every pixel, on the diagonal where the two triangles of the
        // rectangle meet too, is the same grey.
        let grey = canvas.pixel(0, 7);
        assert_eq!(grey.r(), 128);
        for i in 0..8 {
            assert_eq!(canvas.pixel(i, i), grey, "pixel {i},{i}");
        }
    }

    #[test]
    fn points_are_scaled_to_pixels_and_clipped() {
        let mut canvas = canvas(8, 8);
        let rect = Rect::from_min_size(Pos2::ZERO, Vec2::splat(4.0));
        let mut primitive = rect_mesh(rect, Color32::GREEN);
        primitive.clip_rect = Rect::from_min_size(Pos2::ZERO, Vec2::new(4.0, 1.0));
        canvas.paint_primitives(&[primitive], 2.0, Color32::BLACK);
        assert_eq!(canvas.pixel(7, 1), Color32::GREEN, "4 points are 8 pixels");
        assert_eq!(canvas.pixel(7, 2), Color32::BLACK, "clipped below 1 point");
    }

    #[test]
    fn the_same_shapes_are_not_painted_again() {
        let mut canvas = canvas(8, 8);
        let rect = Rect::from_min_size(Pos2::ZERO, Vec2::splat(4.0));
        let mut no_textures = TexturesDelta::default();
        let paint = |canvas: &mut Canvas, color, textures: &mut TexturesDelta| {
            canvas.paint(vec![rect_mesh(rect, color)], textures, 1.0, Color32::BLACK)
        };
        assert!(paint(&mut canvas, Color32::RED, &mut no_textures));
        assert!(!paint(&mut canvas, Color32::RED, &mut no_textures));
        assert!(paint(&mut canvas, Color32::BLUE, &mut no_textures));
        assert_eq!(canvas.pixel(1, 1), Color32::BLUE);
    }

    #[test]
    fn a_texture_patch_changes_only_its_part() {
        let mut canvas = canvas(4, 4);
        let mut delta = TexturesDelta::default();
        delta.set.insert(
            TextureId::default(),
            [ImageDelta::partial(
                [1, 1],
                ColorImage::filled([2, 1], Color32::BLUE),
                TextureOptions::NEAREST,
            )]
            .into(),
        );
        canvas.set_textures(&mut delta);
        let texture = &canvas.textures[&TextureId::default()];
        assert_eq!(texture.pixels[5], Color32::BLUE);
        assert_eq!(texture.pixels[6], Color32::BLUE);
        assert_eq!(texture.pixels[7], Color32::WHITE);
    }
}
