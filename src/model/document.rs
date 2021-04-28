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

use std::sync::Arc;

use crate::model::mod_stack::ModStack;
use crate::model::pixels::PixelState;
use crate::model::types::*;

#[derive(Clone, druid::Data, Default)]
pub struct MoveInfo {
    start_point: druid::Point,
    start_area: druid::Rect,
    bytes: PixelBytes,
}

impl MoveInfo {
    pub fn new(start_point: druid::Point, start_area: druid::Rect, bytes: Vec<u8>) -> Self {
        Self {
            start_point,
            start_area,
            bytes: Arc::new(bytes),
        }
    }

    pub fn _bytes(&self) -> &PixelBytes {
        &self.bytes
    }

    pub fn offset(&self) -> druid::Point {
        self.start_point - (self.start_area.x0, self.start_area.y0)
    }
}

/// Per-document state.
#[derive(Clone, druid::Data, Default)]
pub struct Document {
    selection: Option<druid::Rect>,
    move_info: Option<MoveInfo>,
    pixels: PixelState,
    path: Option<String>,
    new_path: Option<String>,
    undo: ModStack,
    redo: ModStack,
}

impl Document {
    pub fn new(pixels: PixelState, path: String) -> Self {
        Self {
            pixels,
            path: Some(path),
            ..Default::default()
        }
    }

    pub fn selection(&self) -> Option<druid::Rect> {
        self.selection
    }

    pub fn clear_selection(&mut self) {
        self.selection = None;
    }

    pub fn set_selection(&mut self, selection: druid::Rect) {
        self.selection = Some(selection);
    }

    pub fn _is_moving(&self) -> bool {
        self.move_info.is_some()
    }

    pub fn move_info(&self) -> Option<&MoveInfo> {
        self.move_info.as_ref()
    }

    pub fn clear_move_info(&mut self) {
        self.move_info = None;
    }

    pub fn set_move_info(&mut self, move_info: MoveInfo) {
        self.move_info = Some(move_info);
    }

    pub fn pixels(&self) -> &PixelState {
        &self.pixels
    }

    pub fn pixels_mut(&mut self) -> &mut PixelState {
        &mut self.pixels
    }

    pub fn path(&self) -> Option<String> {
        self.path.clone()
    }

    pub fn set_path(&mut self, path: String) {
        self.path = Some(path);
    }

    pub fn new_path(&self) -> Option<String> {
        self.new_path.clone()
    }

    pub fn set_new_path(&mut self, new_path: String) {
        self.new_path = Some(new_path);
    }

    pub fn undo(&mut self) -> &mut ModStack {
        &mut self.undo
    }

    pub fn redo(&mut self) -> &mut ModStack {
        &mut self.redo
    }

    /// Get the current boundary. If a selection exists, then that's the boundary.
    /// Otherwise, it's the entire canvas. The result is in canvas coords.
    pub fn bounds(&self) -> druid::Rect {
        let b = self
            .selection
            .unwrap_or_else(|| self.pixels.header().bounds());
        druid::Rect::new(b.x0, b.y0, b.x1 + 1.0, b.y1 + 1.0)
    }
}
