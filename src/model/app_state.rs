use std::collections::VecDeque;
use std::sync::Arc;

use crate::model::mod_record::ModRecord;
use crate::model::pixel_state::PixelState;
use crate::model::tool_type::ToolType;

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
    pub mod_stack: Arc<VecDeque<ModRecord>>,
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
            mod_stack: Arc::new(VecDeque::with_capacity(1)),
        }
    }
}
