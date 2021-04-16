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

use crate::model::mods::ModStack;
use crate::model::pixels::PixelState;
use crate::model::types::*;

/// Per-document state.
#[derive(Clone, druid::Data, Default)]
pub struct Document {
    pub selection: Option<druid::Rect>,
    pub move_bytes: Option<PixelBytes>,
    pub pixels: PixelState,
    pub path: Option<String>,
    pub new_path: Option<String>,
    pub undo: ModStack,
    pub redo: ModStack,
}

impl Document {
    /// Get the current boundary. If a selection exists, then that's the boundary.
    /// Otherwise, it's the entire canvas. The result is in canvas coords.
    pub fn bounds(&self) -> druid::Rect {
        let b = self
            .selection
            .unwrap_or_else(|| self.pixels.header.bounds());
        druid::Rect::new(b.x0, b.y0, b.x1 + 1.0, b.y1 + 1.0)
    }
}
