use crate::model::types::PixelHeader;
use crate::transforms::util;

pub fn grayscale(header: &PixelHeader, bytes: &mut Vec<u8>, bounds: druid::Rect) {
    for y in bounds.y0 as usize..bounds.y1 as usize + 1 {
        for x in bounds.x0 as usize..bounds.x1 as usize + 1 {
            let rgba = util::read(x, y, header, bytes);
            util::write(x, y, header, bytes, util::as_gray(rgba));
        }
    }
}

pub fn _floyd_steinberg(
    _header: &PixelHeader,
    _bytes: &mut Vec<u8>,
    _bounds: (usize, usize, usize, usize),
) {
}
