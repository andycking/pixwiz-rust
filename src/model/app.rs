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
    brush_color: druid::Color,
    pos_color: druid::Color,
    start_pos: druid::Point,
    current_pos: druid::Point,
    window_pos: druid::Point,
    window_state: WindowState,
    tool_type: ToolType,
    show_grid: bool,
    pub doc: Document,

    #[data(same_fn = "PartialEq::eq")]
    window_id: druid::WindowId,
}

impl AppState {
    pub fn new(window_id: druid::WindowId) -> Self {
        Self {
            brush_color: druid::Color::BLACK,
            pos_color: druid::Color::rgba8(0, 0, 0, 0),
            start_pos: Default::default(),
            current_pos: Default::default(),
            window_pos: Default::default(),
            window_state: Default::default(),
            tool_type: ToolType::Paint,
            show_grid: true,
            doc: Default::default(),
            window_id,
        }
    }

    pub fn brush_color(&self) -> &druid::Color {
        &self.brush_color
    }

    pub fn set_brush_color(&mut self, brush_color: druid::Color) {
        self.brush_color = brush_color;
    }

    pub fn pos_color(&self) -> &druid::Color {
        &self.pos_color
    }

    pub fn set_pos_color(&mut self, pos_color: druid::Color) {
        self.pos_color = pos_color;
    }

    pub fn start_pos(&self) -> druid::Point {
        self.start_pos
    }

    pub fn set_start_pos(&mut self, start_pos: druid::Point) {
        self.start_pos = start_pos;
    }

    pub fn current_pos(&self) -> druid::Point {
        self.current_pos
    }

    pub fn set_current_pos(&mut self, current_pos: druid::Point) {
        self.current_pos = current_pos;
    }

    pub fn window_pos(&self) -> druid::Point {
        self.window_pos
    }

    pub fn set_window_pos(&mut self, window_pos: druid::Point) {
        self.window_pos = window_pos;
    }

    pub fn window_state(&self) -> WindowState {
        self.window_state
    }

    pub fn set_window_state(&mut self, window_state: WindowState) {
        self.window_state = window_state;
    }

    pub fn reset_window_state(&mut self) {
        self.window_state = Default::default();
    }

    pub fn tool_type(&self) -> ToolType {
        self.tool_type
    }

    pub fn set_tool_type(&mut self, tool_type: ToolType) {
        self.tool_type = tool_type;
    }

    pub fn show_grid(&self) -> bool {
        self.show_grid
    }

    pub fn flip_grid(&mut self) {
        self.show_grid = !self.show_grid
    }

    pub fn window_id(&self) -> druid::WindowId {
        self.window_id
    }
}
