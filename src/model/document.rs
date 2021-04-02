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

/// Per-document state.
#[derive(Clone, druid::Data)]
pub struct Document {
    pub selection: Option<druid::Rect>,
    pub move_bytes: Option<PixelBytes>,
    pub pixels: PixelState,
    pub path: Option<String>,
    pub undo: ModStack,
    pub redo: ModStack,
}

impl Default for Document {
    fn default() -> Self {
        Self {
            selection: None,
            move_bytes: None,
            pixels: Default::default(),
            path: None,
            undo: Default::default(),
            redo: Default::default(),
        }
    }
}
