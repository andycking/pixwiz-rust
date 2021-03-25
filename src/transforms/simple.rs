use crate::model::pixel_env::PixelEnv;
use crate::model::pixel_header::PixelHeader;

pub fn clear(header: &PixelHeader, env: &PixelEnv, bytes: &mut Vec<u8>) {
    for y in env.bounds.y0 as usize..env.bounds.y1 as usize {
        for x in env.bounds.x0 as usize..env.bounds.x1 as usize {
            super::write(x, y, header, bytes, &druid::Color::rgba(0.0, 0.0, 0.0, 0.0));
        }
    }
}
