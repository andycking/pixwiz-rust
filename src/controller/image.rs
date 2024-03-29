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
use crate::util::shapes;

pub fn black_and_white(ctx: &mut druid::DelegateCtx, cmd: &druid::Command, data: &mut AppState) {
    move_drop(ctx, cmd, data);
    transforms::apply(data, transforms::colors::black_and_white, 0.5);
}

pub fn brighten(ctx: &mut druid::DelegateCtx, cmd: &druid::Command, data: &mut AppState) {
    move_drop(ctx, cmd, data);
    transforms::apply(data, transforms::colors::brightness, 0.05);
}

pub fn clear(ctx: &mut druid::DelegateCtx, cmd: &druid::Command, data: &mut AppState) {
    move_drop(ctx, cmd, data);
    transforms::apply(data, transforms::simple::clear, 0.0);
}

pub fn darken(ctx: &mut druid::DelegateCtx, cmd: &druid::Command, data: &mut AppState) {
    move_drop(ctx, cmd, data);
    transforms::apply(data, transforms::colors::brightness, -0.05);
}

pub fn desaturate(ctx: &mut druid::DelegateCtx, cmd: &druid::Command, data: &mut AppState) {
    move_drop(ctx, cmd, data);
    transforms::apply(data, transforms::colors::desaturate, 0.0);
}

pub fn dither_floyd(ctx: &mut druid::DelegateCtx, cmd: &druid::Command, data: &mut AppState) {
    move_drop(ctx, cmd, data);
    transforms::apply(data, transforms::colors::dither_floyd, 0.0);
}

pub fn eraser(_ctx: &mut druid::DelegateCtx, cmd: &druid::Command, data: &mut AppState) {
    if *cmd.get_unchecked(commands::IMAGE_ERASER) != ToolState::End {
        let current_pos = data.current_pos();
        undo::push_point(data, current_pos);

        let color = druid::Color::rgba8(0, 0, 0, 0);
        data.doc_mut().pixels_mut().write(current_pos, &color);
    }
}

pub fn fill(ctx: &mut druid::DelegateCtx, cmd: &druid::Command, data: &mut AppState) {
    move_drop(ctx, cmd, data);

    let f = if *cmd.get_unchecked(commands::IMAGE_FILL) {
        transforms::colors::flood_fill
    } else {
        transforms::colors::fill
    };

    transforms::apply(data, f, 0.0);
}

pub fn marquee(_ctx: &mut druid::DelegateCtx, _cmd: &druid::Command, data: &mut AppState) {
    let start_pos = data.start_pos();
    let current_pos = data.current_pos();

    let old_selection = data.doc().selection().unwrap_or(druid::Rect::ZERO);
    let new_selection = shapes::enclosing_rect(start_pos, current_pos);

    if old_selection != new_selection {
        data.doc_mut().set_selection(new_selection);
    }
}

pub fn move_(_ctx: &mut druid::DelegateCtx, cmd: &druid::Command, data: &mut AppState) {
    if let Some(selection) = data.doc().selection() {
        match *cmd.get_unchecked(commands::IMAGE_MOVE) {
            ToolState::Start => {
                if !data.doc().is_moving() {
                    let current_pos = data.current_pos();
                    let clone_rect = shapes::inflate_rect(selection);
                    let pixels = data.doc().pixels().clone_area(clone_rect);

                    let move_info = MoveInfo::new(current_pos, selection, pixels);
                    data.doc_mut().set_move_info(move_info);

                    // FIXME: Yikes, we're always pushing the entire image onto the undo
                    // stack. The problem is that we need to remember the pixels that we're
                    // about to drag, *and* the pixels that we'll replace when we eventually
                    // drop the drag. These might be non-overlapping areas, which our undo
                    // stack doesn't support. One alternative is to use two undo records, but
                    // that gets super messy. Another alternative is to push the union onto the
                    // undo stack at the time we drop.
                    let bounds = data.doc().pixels().header().bounds();
                    undo::push(data, bounds);

                    transforms::apply_no_undo(data, transforms::simple::clear, 0.0);
                }
            }

            ToolState::Move => {
                if let Some(move_info) = data.doc().move_info() {
                    let bounds = data.doc().pixels().header().bounds();
                    let current_pos = data.current_pos();
                    let offset = move_info.offset();

                    let point_rect = druid::Rect::from_origin_size(
                        (current_pos.x, current_pos.y),
                        (selection.width(), selection.height()),
                    );
                    let rect = shapes::offset_rect(point_rect, offset);
                    let new_selection = shapes::constrain_rect(rect, bounds);

                    data.doc_mut().set_selection(new_selection);
                }
            }

            _ => {}
        };
    }
}

pub fn move_drop(_ctx: &mut druid::DelegateCtx, _cmd: &druid::Command, data: &mut AppState) {
    if data.doc().is_moving() {
        let selection = shapes::inflate_rect(data.doc().selection().unwrap());
        let move_info = data.doc().move_info().unwrap().to_owned();
        let bytes = move_info.pixels().bytes();

        data.doc_mut().pixels_mut().write_area(selection, bytes);
    }

    data.doc_mut().clear_move_info();
}

pub fn paint(_ctx: &mut druid::DelegateCtx, cmd: &druid::Command, data: &mut AppState) {
    if *cmd.get_unchecked(commands::IMAGE_PAINT) != ToolState::End {
        let current_pos = data.current_pos();
        undo::push_point(data, current_pos);

        let color = data.brush_color().clone();
        data.doc_mut().pixels_mut().write(current_pos, &color);
    }
}
