use crate::model::types::PixelHeader;
use crate::transforms::util;

pub fn erase(header: &PixelHeader, bytes: &mut Vec<u8>, bounds: (usize, usize, usize, usize)) {
    for y in bounds.1..bounds.3 + 1 {
        for x in bounds.0..bounds.2 + 1 {
            util::write(x, y, header, bytes, (0, 0, 0, 0));
        }
    }
}
