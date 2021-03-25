/// Capture environment.
pub struct PixelEnv {
    pub color: druid::Color,
    pub pos: druid::Point,
    pub bounds: druid::Rect,
}

impl PixelEnv {
    pub fn new(color: druid::Color, pos: druid::Point, bounds: druid::Rect) -> Self {
        Self {
            color: color,
            pos: pos,
            bounds: bounds,
        }
    }
}
