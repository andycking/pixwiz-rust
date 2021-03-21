use crate::model::types::PixelHeader;
use crate::model::types::Rgba;
use crate::transforms::util;

pub fn grayscale(header: &PixelHeader, bytes: &mut Vec<u8>, bounds: (usize, usize, usize, usize)) {
    for y in bounds.1..bounds.3 + 1 {
        for x in bounds.0..bounds.2 + 1 {
            let rgba = util::read(x, y, header, bytes);

            // TODO: Find nearest color in the current palette. I guess that means passing
            // in the palette too.
            let avg = ((rgba.r as u32 + rgba.g as u32 + rgba.b as u32) / 3) as u8;

            util::write(x, y, header, bytes, Rgba::new(avg, avg, avg, rgba.a));
        }
    }
}

pub fn _floyd_steinberg(
    _header: &PixelHeader,
    _bytes: &mut Vec<u8>,
    _bounds: (usize, usize, usize, usize),
) {
}
