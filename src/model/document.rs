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

/// Application state machine.
#[derive(Clone, druid::Data, PartialEq)]
pub enum StateMachine {
    Idle,
    UnsavedAlert,
    UnsavedSave,
}

impl Default for StateMachine {
    fn default() -> Self {
        Self::Idle
    }
}

impl StateMachine {
    pub fn is_idle(&self) -> bool {
        *self == Self::Idle
    }
}

/// Per-document state.
#[derive(Clone, druid::Data)]
pub struct Document {
    pub selection: Option<druid::Rect>,
    pub move_bytes: Option<PixelBytes>,
    pub pixels: PixelState,
    pub path: Option<String>,
    pub new_path: Option<String>,
    pub undo: ModStack,
    pub redo: ModStack,
    pub state_machine: StateMachine,
}

impl Default for Document {
    fn default() -> Self {
        Self {
            selection: None,
            move_bytes: None,
            pixels: Default::default(),
            path: None,
            new_path: None,
            undo: Default::default(),
            redo: Default::default(),
            state_machine: Default::default(),
        }
    }
}

impl Document {
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
