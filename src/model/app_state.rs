use crate::model::mod_stack::ModStack;
use crate::model::pixel_state::PixelState;
use crate::model::types::ToolType;

/// Application state.
#[derive(Clone, druid::Data)]
pub struct AppState {
    pub brush_color: druid::Color,
    pub pos_color: druid::Color,
    pub start_pos: druid::Point,
    pub current_pos: druid::Point,
    pub selection: Option<druid::Rect>,
    pub tool_type: ToolType,
    pub pixels: PixelState,
    pub path: Option<String>,
    pub show_grid: bool,
    pub undo: ModStack,
    pub redo: ModStack,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            brush_color: druid::Color::BLACK,
            pos_color: druid::Color::rgba8(0, 0, 0, 0),
            start_pos: druid::Point::ZERO,
            current_pos: druid::Point::ZERO,
            selection: None,
            tool_type: ToolType::Paint,
            pixels: Default::default(),
            path: None,
            show_grid: true,
            undo: Default::default(),
            redo: Default::default(),
        }
    }
}

impl AppState {
    /// Get the current boundary. If a selection exists, then that's the boundary.
    /// Otherwise, it's the entire canvas. The result is in canvas coords.
    pub fn get_bounds(&self) -> druid::Rect {
        let mut bounds = match self.selection {
            Some(rect) => rect,
            _ => druid::Rect::new(
                1.0,
                1.0,
                self.pixels.header.width as f64,
                self.pixels.header.height as f64,
            ),
        };
        bounds.x1 += 1.0;
        bounds.y1 += 1.0;

        bounds
    }
}
