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
    let bounds = data.doc().bounds();
    undo::push(data, bounds);

    // The transform function gets copies of the header and the bytes. We don't want
    // it mucking directly with our pixels.
    let header = data.doc().pixels().header().clone();
    let brush_color = data.brush_color().clone();
    let current_pos = data.current_pos();
    let env = PixelEnv::new(brush_color, current_pos, bounds, param);
    let mut bytes = data.doc().pixels().bytes().to_vec();

    f(&header, &env, &mut bytes);

    // Write back the modified pixels.
    data.doc.pixels_mut().set_bytes(bytes);
}
