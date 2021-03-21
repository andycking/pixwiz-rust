use crate::model::types::PixelHeader;
use crate::transforms::util;

pub fn erase(header: &PixelHeader, bytes: &mut Vec<u8>, bounds: (usize, usize, usize, usize)) {
    for col in bounds.0..bounds.2 + 1 {
        for row in bounds.1..bounds.3 + 1 {
            util::write(col, row, header, bytes, 0, 0, 0, 0);
        }
    }
}
