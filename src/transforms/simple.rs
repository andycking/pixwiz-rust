use crate::model::types::PixelHeader;
use crate::transforms::util;

pub fn erase(header: &PixelHeader, bytes: &mut Vec<u8>, bounds: druid::Rect) {
    for y in bounds.y0 as usize..bounds.y1 as usize + 1 {
        for x in bounds.x0 as usize..bounds.x1 as usize + 1 {
            util::write(x, y, header, bytes, druid::Color::rgba(0.0, 0.0, 0.0, 0.0));
        }
    }
}
