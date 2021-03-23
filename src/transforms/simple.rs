use crate::model::types::PixelEnv;
use crate::model::types::PixelHeader;
use crate::transforms::util;

pub fn erase(header: &PixelHeader, env: &PixelEnv, bytes: &mut Vec<u8>) {
    for y in env.bounds.y0 as usize..env.bounds.y1 as usize {
        for x in env.bounds.x0 as usize..env.bounds.x1 as usize {
            util::write(x, y, header, bytes, druid::Color::rgba(0.0, 0.0, 0.0, 0.0));
        }
    }
}
