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

    #[data(same_fn = "PartialEq::eq")]
    pub id: druid::WindowId,
}

impl AppState {
    pub fn new(id: druid::WindowId) -> Self {
        Self {
            brush_color: druid::Color::BLACK,
            pos_color: druid::Color::rgba8(0, 0, 0, 0),
            start_pos: druid::Point::ZERO,
            current_pos: druid::Point::ZERO,
            window_pos: druid::Point::ZERO,
            tool_type: ToolType::Paint,
            show_grid: true,
            doc: Default::default(),
            id,
        }
    }
}
