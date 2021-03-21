use crate::model::types::PixelHeader;
use crate::model::types::Rgba;
use crate::transforms::util;

pub fn grayscale(header: &PixelHeader, bytes: &mut Vec<u8>, bounds: (usize, usize, usize, usize)) {
    for y in bounds.1..bounds.3 + 1 {
        for x in bounds.0..bounds.2 + 1 {
            let rgba = util::read(x, y, header, bytes);
            util::write(x, y, header, bytes, util::as_gray(&rgba));
        }
    }
}

pub fn _floyd_steinberg(
    _header: &PixelHeader,
    _bytes: &mut Vec<u8>,
    _bounds: (usize, usize, usize, usize),
) {
}
