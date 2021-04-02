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

use crate::model::mod_stack::ModStack;
use crate::model::pixel_state::PixelState;
use crate::model::types::*;

/// Application state.
#[derive(Clone, druid::Data)]
pub struct AppState {
    pub brush_color: druid::Color,
    pub pos_color: druid::Color,
    pub start_pos: druid::Point,
    pub current_pos: druid::Point,
    pub selection: Option<druid::Rect>,
    pub move_bytes: Option<PixelBytes>,
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
            move_bytes: None,
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
    /// Reset app state. This is used to blow away things that are specific to a document,
    /// like the undo stack and the marquee selection. A good example of when to use this
    /// is on New/Open File.
    pub fn reset(&mut self) {
        self.selection = None;
        self.move_bytes = None;
        self.pixels = Default::default();
        self.path = None;
        self.undo.clear();
        self.redo.clear();
    }

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
