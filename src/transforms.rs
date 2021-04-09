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

use crate::controller::undo;
use crate::model::app::AppState;
use crate::model::pixels::PixelEnv;
use crate::model::pixels::PixelHeader;

pub mod colors;
pub mod simple;
mod util;

pub fn apply<F>(data: &mut AppState, f: F, param: f64)
where
    F: Fn(&PixelHeader, &PixelEnv, &mut Vec<u8>),
{
    // We have all the information we need for a mod record, so just create it here.
    // That way the caller, and the f() we're applying, don't need to worry about it.
    let bounds = data.get_bounds();
    undo::push(data, bounds);

    let env = PixelEnv::new(data.brush_color.clone(), data.current_pos, bounds, param);
    let bytes = Arc::make_mut(&mut data.doc.pixels.bytes);

    f(&data.doc.pixels.header, &env, bytes);

    data.doc.pixels.dirty = true;
}
