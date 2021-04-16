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

/// Shared reference to pixel bytes. We use this type a lot.
pub type PixelBytes = Arc<Vec<u8>>;

/// Supported tool types.
#[derive(Clone, Copy, druid::Data, Debug, PartialEq)]
pub enum ToolType {
    Dropper,
    Eraser,
    Fill,
    Marquee,
    Move,
    Paint,
}

/// Window state.
#[derive(Clone, druid::Data, PartialEq)]
pub enum WindowState {
    Normal,
    UnsavedAlert,
    UnsavedSave,
}

impl Default for WindowState {
    fn default() -> Self {
        Self::Normal
    }
}
