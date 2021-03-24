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
    let bounds = data.get_bounds();
    let env = PixelEnv::new(data.brush_color.clone(), data.current_pos, bounds);
    let bytes = Arc::make_mut(&mut data.pixels.bytes);

    f(&data.pixels.header, &env, bytes);

    data.pixels.dirty = true;
}
