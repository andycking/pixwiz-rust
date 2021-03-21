use crate::model::types::PixelHeader;
use crate::transforms::util;

pub fn grayscale(header: &PixelHeader, bytes: &mut Vec<u8>, bounds: (usize, usize, usize, usize)) {
    for col in bounds.0..bounds.2 + 1 {
        for row in bounds.1..bounds.3 + 1 {
            let (r, g, b, a) = util::read(col, row, header, bytes);

            // TODO: Find nearest color in the current palette. I guess that means passing
            // in the palette too.
            let avg = ((r as u32 + g as u32 + b as u32) / 3) as u8;

            util::write(col, row, header, bytes, avg, avg, avg, a);
        }
    }
}

pub fn _floyd_steinberg(
    _header: &PixelHeader,
    _bytes: &mut Vec<u8>,
    _bounds: (usize, usize, usize, usize),
) {
}
