use std::sync::Arc;

pub mod app_state;
pub mod mod_record;
pub mod pixel_env;
pub mod pixel_header;
pub mod pixel_state;
pub mod tool_type;

use crate::model::app_state::AppState;
use crate::model::mod_record::ModRecord;

pub fn get_bounds(data: &AppState) -> druid::Rect {
    let mut bounds = match data.selection {
        Some(rect) => rect,
        _ => druid::Rect::new(
            1.0,
            1.0,
            data.pixels.header.width as f64,
            data.pixels.header.height as f64,
        ),
    };
    bounds.x1 += 1.0;
    bounds.y1 += 1.0;

    bounds
}

pub fn push_mod_record_point(data: &mut AppState, p: druid::Point) {
    let area = druid::Rect::new(p.x, p.y, p.x + 1.0, p.y + 1.0);
    push_mod_record_rect(data, area);
}

pub fn push_mod_record_rect(data: &mut AppState, r: druid::Rect) {
    let bytes = vec![0; 0]; // Vile lies.
    let record = ModRecord::new(r, bytes);

    let mod_stack = Arc::make_mut(&mut data.mod_stack);

    mod_stack.push_front(record);
    mod_stack.truncate(1);
}
