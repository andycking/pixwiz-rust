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

use crate::commands;
use crate::controller::undo;
use crate::model::app::AppState;
use crate::transforms;

pub fn black_and_white(_ctx: &mut druid::DelegateCtx, _cmd: &druid::Command, data: &mut AppState) {
    transforms::apply(data, transforms::colors::black_and_white, 0.5);
}

pub fn brighten(_ctx: &mut druid::DelegateCtx, _cmd: &druid::Command, data: &mut AppState) {
    transforms::apply(data, transforms::colors::brightness, 0.05);
}

pub fn clear(_ctx: &mut druid::DelegateCtx, _cmd: &druid::Command, data: &mut AppState) {
    transforms::apply(data, transforms::simple::clear, 0.0);
}

pub fn darken(_ctx: &mut druid::DelegateCtx, _cmd: &druid::Command, data: &mut AppState) {
    transforms::apply(data, transforms::colors::brightness, -0.05);
}

pub fn desaturate(_ctx: &mut druid::DelegateCtx, _cmd: &druid::Command, data: &mut AppState) {
    transforms::apply(data, transforms::colors::desaturate, 0.0);
}

pub fn dither_floyd(_ctx: &mut druid::DelegateCtx, _cmd: &druid::Command, data: &mut AppState) {
    transforms::apply(data, transforms::colors::dither_floyd, 0.0);
}

pub fn eraser(_ctx: &mut druid::DelegateCtx, _cmd: &druid::Command, data: &mut AppState) {
    let current_pos = data.current_pos();
    undo::push_point(data, current_pos);

    let idx = data.doc().pixels().point_to_idx(current_pos);
    let color = druid::Color::rgba8(0, 0, 0, 0);
    data.doc.pixels_mut().write(idx, &color);
}

pub fn fill(_ctx: &mut druid::DelegateCtx, cmd: &druid::Command, data: &mut AppState) {
    let f = match *cmd.get_unchecked(commands::IMAGE_FILL) {
        true => transforms::colors::flood_fill,
        _ => transforms::colors::fill,
    };
    transforms::apply(data, f, 0.0);
}

pub fn marquee(_ctx: &mut druid::DelegateCtx, _cmd: &druid::Command, data: &mut AppState) {
    let start_pos = data.start_pos();
    let current_pos = data.current_pos();

    let x0 = start_pos.x.min(current_pos.x);
    let y0 = start_pos.y.min(current_pos.y);
    let x1 = start_pos.x.max(current_pos.x);
    let y1 = start_pos.y.max(current_pos.y);

    let new_selection = druid::Rect::new(x0, y0, x1, y1);

    let old_selection = data.doc().selection().unwrap_or(druid::Rect::ZERO);

    if old_selection != new_selection {
        data.doc.set_selection(new_selection);
    }
}

pub fn move_(_ctx: &mut druid::DelegateCtx, _cmd: &druid::Command, _data: &mut AppState) {}

pub fn paint(_ctx: &mut druid::DelegateCtx, _cmd: &druid::Command, data: &mut AppState) {
    let current_pos = data.current_pos();
    undo::push_point(data, current_pos);

    let idx = data.doc().pixels().point_to_idx(current_pos);
    let color = data.brush_color().clone();
    data.doc.pixels_mut().write(idx, &color);
}
