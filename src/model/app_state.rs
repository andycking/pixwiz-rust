// Copyright 2021 Andy King
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use super::document::Document;
use crate::model::types::*;

/// Application state.
#[derive(Clone, druid::Data)]
pub struct AppState {
    pub brush_color: druid::Color,
    pub pos_color: druid::Color,
    pub start_pos: druid::Point,
    pub current_pos: druid::Point,
    pub window_pos: druid::Point,
    pub tool_type: ToolType,
    pub show_grid: bool,
    pub doc: Document,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            brush_color: druid::Color::BLACK,
            pos_color: druid::Color::rgba8(0, 0, 0, 0),
            start_pos: druid::Point::ZERO,
            current_pos: druid::Point::ZERO,
            window_pos: druid::Point::ZERO,
            tool_type: ToolType::Paint,
            show_grid: true,
            doc: Default::default(),
        }
    }
}

impl AppState {
    /// Get the current boundary. If a selection exists, then that's the boundary.
    /// Otherwise, it's the entire canvas. The result is in canvas coords.
    pub fn get_bounds(&self) -> druid::Rect {
        let mut bounds = match self.doc.selection {
            Some(rect) => rect,
            _ => druid::Rect::new(
                1.0,
                1.0,
                self.doc.pixels.header.width as f64,
                self.doc.pixels.header.height as f64,
            ),
        };
        bounds.x1 += 1.0;
        bounds.y1 += 1.0;

        bounds
    }
}
