//! Braille dots shared by the camera painter.

/// Braille dot bit for a point inside a cell: column 0 or 1, row 0..4.
pub(crate) const BRAILLE_BITS: [[u8; 4]; 2] = [[0x01, 0x02, 0x04, 0x40], [0x08, 0x10, 0x20, 0x80]];

/// Ordered thresholds for the eight dots of a Braille cell, eighths. A dot
/// lights when the level it stands for passes its own threshold, so the
/// dot count follows the level and two neighbouring cells at the same level
/// do not light identically: shading, out of glyphs that have none of their
/// own. Shared, so two Braille pictures on one screen dither alike.
pub(crate) const BRAILLE_DITHER: [[u8; 2]; 4] = [[0, 4], [6, 2], [1, 5], [7, 3]];

/// Whether the dot at `(dx, dy)` of a Braille cell lights for `level`,
/// 0 through 1.
pub(crate) fn braille_dot(level: f32, dx: usize, dy: usize) -> bool {
    level * 8.0 > f32::from(BRAILLE_DITHER[dy][dx]) + 0.5
}

/// A 2×4 patch of luminance as one Braille glyph, indexed `[dy][dx]`.
/// Empty is `⠀` and full is `⣿`; everything between is a dot count, which
/// is the only shading a glyph raster has.
pub(crate) fn braille_luma_glyph(patch: [[u8; 2]; 4]) -> char {
    let mut bits = 0u8;
    for (dy, row) in patch.iter().enumerate() {
        for (dx, level) in row.iter().enumerate() {
            if braille_dot(f32::from(*level) / 255.0, dx, dy) {
                bits |= BRAILLE_BITS[dx][dy];
            }
        }
    }
    char::from_u32(0x2800 + u32::from(bits)).unwrap_or(' ')
}
