use crate::model::state::AppState;

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
