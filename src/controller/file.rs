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

use crate::model::app::AppState;
use crate::model::document::Document;
use crate::model::types::*;
use crate::storage;
use crate::view::alert;

pub fn new(ctx: &mut druid::DelegateCtx, _cmd: &druid::Command, data: &mut AppState) {
    assert!(data.window_state == WindowState::Normal);

    if data.doc.pixels.dirty() {
        data.window_state = WindowState::UnsavedAlert;
        let alert = alert::unsaved(data.window_pos);
        ctx.new_window(alert);
    } else {
        data.doc = Default::default();
    }
}

pub fn open(ctx: &mut druid::DelegateCtx, cmd: &druid::Command, data: &mut AppState) {
    assert!(data.window_state == WindowState::Normal);

    // If the file dialog passes us an invalid path then all bets are off. Just let it panic.
    let file_info = cmd.get_unchecked(druid::commands::OPEN_FILE);
    let path = file_info.path().to_str().unwrap();

    data.doc.set_new_path(String::from(path));

    if data.doc.pixels.dirty() {
        data.window_state = WindowState::UnsavedAlert;
        let alert = alert::unsaved(data.window_pos);
        ctx.new_window(alert);
    } else {
        open_internal(ctx, cmd, data);
    }
}

pub fn open_internal(_ctx: &mut druid::DelegateCtx, _cmd: &druid::Command, data: &mut AppState) {
    assert!(data.window_state != WindowState::UnsavedAlert);

    if let Some(new_path) = data.doc.new_path() {
        match storage::png::read_path(&new_path) {
            Ok(pixels) => data.doc = Document::new(pixels, new_path),
            Err(_e) => {}
        }
    } else {
        data.doc = Default::default();
    }
}

pub fn save(_ctx: &mut druid::DelegateCtx, _cmd: &druid::Command, data: &mut AppState) {
    assert!(data.window_state == WindowState::Normal);

    if let Some(path) = data.doc.path() {
        match storage::png::write_path(&path, &data.doc.pixels) {
            Ok(()) => {}
            Err(_e) => {}
        };
    }
}

pub fn save_as(ctx: &mut druid::DelegateCtx, cmd: &druid::Command, data: &mut AppState) {
    assert!(data.window_state != WindowState::UnsavedAlert);

    // If the file dialog passes us an invalid path then all bets are off. Just let it panic.
    let file_info = cmd.get_unchecked(druid::commands::SAVE_FILE_AS);
    let path = file_info.path().to_str().unwrap();

    match storage::png::write_path(path, &data.doc.pixels) {
        Ok(()) => {
            if data.window_state == WindowState::UnsavedSave {
                open_internal(ctx, cmd, data);
            } else {
                data.doc.set_path(String::from(path));
            }
        }
        Err(_e) => {}
    }
}

pub fn save_cancelled(_ctx: &mut druid::DelegateCtx, _cmd: &druid::Command, data: &mut AppState) {
    data.window_state = Default::default();
}
