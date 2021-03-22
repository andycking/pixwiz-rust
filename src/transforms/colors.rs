use crate::model::types::PixelHeader;
use crate::transforms::util;

pub fn black_and_white(header: &PixelHeader, bytes: &mut Vec<u8>, bounds: druid::Rect) {
    for y in bounds.y0 as usize..bounds.y1 as usize + 1 {
        for x in bounds.x0 as usize..bounds.x1 as usize + 1 {
            let color = util::read(x, y, header, bytes);
            let bw = util::black_and_white(color, 0.5);
            util::write(x, y, header, bytes, bw);
        }
    }
}

pub fn desaturate(header: &PixelHeader, bytes: &mut Vec<u8>, bounds: druid::Rect) {
    for y in bounds.y0 as usize..bounds.y1 as usize + 1 {
        for x in bounds.x0 as usize..bounds.x1 as usize + 1 {
            let color = util::read(x, y, header, bytes);
            let gray = util::desaturate(color);
            util::write(x, y, header, bytes, gray);
        }
    }
}

pub fn _floyd_steinberg(_header: &PixelHeader, _bytes: &mut Vec<u8>, _bounds: druid::Rect) {}
