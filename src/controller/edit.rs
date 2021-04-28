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

use super::image;
use crate::controller::undo;
use crate::model::app::AppState;

pub fn undo(_ctx: &mut druid::DelegateCtx, _cmd: &druid::Command, data: &mut AppState) {
    undo::pop(data);
}

pub fn redo(_ctx: &mut druid::DelegateCtx, _cmd: &druid::Command, data: &mut AppState) {
    undo::pop_redo(data);
}

pub fn cut(_ctx: &mut druid::DelegateCtx, _cmd: &druid::Command, _data: &mut AppState) {}

pub fn copy(_ctx: &mut druid::DelegateCtx, _cmd: &druid::Command, _data: &mut AppState) {}

pub fn paste(_ctx: &mut druid::DelegateCtx, _cmd: &druid::Command, _data: &mut AppState) {}

pub fn select_all(ctx: &mut druid::DelegateCtx, cmd: &druid::Command, data: &mut AppState) {
    image::move_drop(ctx, cmd, data);

    let bounds = data.doc().pixels().header().bounds();
    data.doc.set_selection(bounds);
}

pub fn deselect(ctx: &mut druid::DelegateCtx, cmd: &druid::Command, data: &mut AppState) {
    image::move_drop(ctx, cmd, data);

    data.doc.clear_selection();
}
