use std::sync::Arc;

use crate::model::state::AppState;
use crate::model::types::PixelEnv;
use crate::model::types::PixelHeader;

pub mod colors;
pub mod simple;
mod util;

pub fn apply<F>(data: &mut AppState, f: F)
where
    F: Fn(&PixelHeader, &PixelEnv, &mut Vec<u8>),
{
    let bounds = match data.selection {
        Some(rect) => rect,
        _ => druid::Rect::new(
            1.0,
            1.0,
            data.pixels.header.width as f64,
            data.pixels.header.height as f64,
        ),
    };

    let env = PixelEnv::new(data.brush_color.clone(), data.current_pos, bounds);

    let bytes = Arc::make_mut(&mut data.pixels.bytes);

    f(&data.pixels.header, &env, bytes);

    data.pixels.dirty = true;
}
