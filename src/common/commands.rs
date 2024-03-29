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

use druid::Selector;

use crate::model::types::ToolState;

pub const EDIT_SELECT_ALL: Selector = Selector::new("edit-select-all");
pub const EDIT_DESELECT: Selector = Selector::new("edit-deselect");

pub const OPEN_FILE_INTERNAL: Selector = Selector::new("open-file-internal");

pub const IMAGE_BLACK_AND_WHITE: Selector = Selector::new("image-black-and-white");
pub const IMAGE_BRIGHTEN: Selector = Selector::new("image-brighten");
pub const IMAGE_CLEAR: Selector = Selector::new("image-clear");
pub const IMAGE_DARKEN: Selector = Selector::new("image-darken");
pub const IMAGE_DESATURATE: Selector = Selector::new("image-desaturate");
pub const IMAGE_DITHER_FLOYD: Selector = Selector::new("image-dither-floyd");
pub const IMAGE_ERASER: Selector<ToolState> = Selector::new("image-eraser");
pub const IMAGE_FILL: Selector<bool> = Selector::new("image-fill");
pub const IMAGE_MARQUEE: Selector<ToolState> = Selector::new("image-marquee");
pub const IMAGE_MOVE: Selector<ToolState> = Selector::new("image-move");
pub const IMAGE_MOVE_DROP: Selector = Selector::new("image-move-drop");
pub const IMAGE_PAINT: Selector<ToolState> = Selector::new("image-paint");

pub const VIEW_SHOW_GRID: Selector = Selector::new("view-show-grid");
