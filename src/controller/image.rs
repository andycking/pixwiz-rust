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
use crate::model::app_state::AppState;
use crate::transforms;

pub fn black_and_white(_ctx: &mut druid::DelegateCtx, _cmd: &druid::Command, data: &mut AppState) {
    transforms::apply(data, transforms::colors::black_and_white);
}

pub fn brighten(_ctx: &mut druid::DelegateCtx, _cmd: &druid::Command, data: &mut AppState) {
    transforms::apply(data, transforms::colors::brighten);
}

pub fn clear(_ctx: &mut druid::DelegateCtx, _cmd: &druid::Command, data: &mut AppState) {
    transforms::apply(data, transforms::simple::clear);
}

pub fn darken(_ctx: &mut druid::DelegateCtx, _cmd: &druid::Command, _data: &mut AppState) {}

pub fn desaturate(_ctx: &mut druid::DelegateCtx, _cmd: &druid::Command, data: &mut AppState) {
    transforms::apply(data, transforms::colors::desaturate);
}

pub fn dither_floyd(_ctx: &mut druid::DelegateCtx, _cmd: &druid::Command, data: &mut AppState) {
    transforms::apply(data, transforms::colors::dither_floyd);
}

pub fn eraser(_ctx: &mut druid::DelegateCtx, _cmd: &druid::Command, data: &mut AppState) {
    undo::push_point(data, data.current_pos);

    let idx = data.doc.pixels.point_to_idx(data.current_pos);
    data.doc.pixels.write(idx, &druid::Color::rgba8(0, 0, 0, 0));
}

pub fn fill(_ctx: &mut druid::DelegateCtx, _cmd: &druid::Command, data: &mut AppState) {
    transforms::apply(data, transforms::colors::fill);
}

pub fn marquee(_ctx: &mut druid::DelegateCtx, _cmd: &druid::Command, data: &mut AppState) {
    let x0 = data.start_pos.x.min(data.current_pos.x);
    let y0 = data.start_pos.y.min(data.current_pos.y);
    let x1 = data.start_pos.x.max(data.current_pos.x);
    let y1 = data.start_pos.y.max(data.current_pos.y);

    let new_selection = druid::Rect::new(x0, y0, x1, y1);

    let old_selection = data.doc.selection.unwrap_or(druid::Rect::ZERO);

    if old_selection != new_selection {
        data.doc.selection = Some(new_selection);
    }
}

pub fn move_(_ctx: &mut druid::DelegateCtx, _cmd: &druid::Command, _data: &mut AppState) {}

pub fn paint(_ctx: &mut druid::DelegateCtx, _cmd: &druid::Command, data: &mut AppState) {
    undo::push_point(data, data.current_pos);

    let idx = data.doc.pixels.point_to_idx(data.current_pos);
    data.doc.pixels.write(idx, &data.brush_color);
}
