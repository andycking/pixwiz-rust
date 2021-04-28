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

use crate::common::commands;
use crate::controller::undo;
use crate::model::app::AppState;
use crate::model::document::MoveInfo;
use crate::model::types::*;
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

pub fn eraser(_ctx: &mut druid::DelegateCtx, cmd: &druid::Command, data: &mut AppState) {
    if *cmd.get_unchecked(commands::IMAGE_ERASER) != ToolState::Up {
        let current_pos = data.current_pos();
        undo::push_point(data, current_pos);

        let color = druid::Color::rgba8(0, 0, 0, 0);
        data.doc.pixels_mut().write(current_pos, &color);
    }
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

pub fn move_(ctx: &mut druid::DelegateCtx, cmd: &druid::Command, data: &mut AppState) {
    if let Some(selection) = data.doc().selection() {
        match *cmd.get_unchecked(commands::IMAGE_MOVE) {
            ToolState::Down => {
                let current_pos = data.current_pos();
                let bytes = data.doc().pixels().read_area(selection);
                let move_info = MoveInfo::new(current_pos, selection, bytes);
                data.doc.set_move_info(move_info);

                clear(ctx, cmd, data);
            }

            ToolState::Move => {
                if let Some(move_info) = data.doc().move_info() {
                    let bounds = data.doc().pixels().header().bounds();
                    let current_pos = data.current_pos();
                    let offset = move_info.offset();

                    let rect = druid::Rect::new(
                        current_pos.x - offset.x,
                        current_pos.y - offset.y,
                        (current_pos.x - offset.x) + selection.width(),
                        (current_pos.y - offset.y) + selection.height(),
                    );

                    let new_selection = constrain(rect, bounds);

                    data.doc.set_selection(new_selection);
                }
            }

            ToolState::Up => {}
        };
    }
}

pub fn paint(_ctx: &mut druid::DelegateCtx, cmd: &druid::Command, data: &mut AppState) {
    if *cmd.get_unchecked(commands::IMAGE_PAINT) != ToolState::Up {
        let current_pos = data.current_pos();
        undo::push_point(data, current_pos);

        let color = data.brush_color().clone();
        data.doc.pixels_mut().write(current_pos, &color);
    }
}

fn constrain(area: druid::Rect, bounds: druid::Rect) -> druid::Rect {
    let width = area.width();
    let height = area.height();

    let mut tl = (area.x0, area.y0);
    let mut br = (area.x1, area.y1);

    if tl.0 < bounds.x0 {
        tl.0 = bounds.x0;
        br.0 = 1.0 + width;
    }
    if tl.1 < bounds.y0 {
        tl.1 = bounds.y0;
        br.1 = 1.0 + height;
    }
    if br.0 > bounds.x1 {
        tl.0 = bounds.x1 - width;
        br.0 = bounds.x1;
    }
    if br.1 > bounds.y1 {
        tl.1 = bounds.y1 - height;
        br.1 = bounds.y1;
    }

    druid::Rect::from_points(tl, br)
}
