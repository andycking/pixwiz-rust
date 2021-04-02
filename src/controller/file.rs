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

use crate::model::app_state::AppState;
use crate::storage;

pub fn new_file(_ctx: &mut druid::DelegateCtx, _cmd: &druid::Command, data: &mut AppState) {
    check_for_save(data);

    data.doc = Default::default();
}

pub fn open_file(_ctx: &mut druid::DelegateCtx, cmd: &druid::Command, data: &mut AppState) {
    check_for_save(data);

    let file_info = cmd.get_unchecked(druid::commands::OPEN_FILE);

    // If the file dialog passes us an invalid path then all bets are off. Just let it panic.
    let path = file_info.path().to_str().unwrap();

    match storage::png::read_path(path) {
        Ok(pixels) => {
            data.doc = Default::default();
            data.doc.pixels = pixels;
            data.doc.path = Some(String::from(path));
        }
        Err(_e) => {}
    }
}

pub fn save_file(_ctx: &mut druid::DelegateCtx, _cmd: &druid::Command, data: &mut AppState) {
    match &data.doc.path {
        Some(path) => match storage::png::write_path(path, &data.doc.pixels) {
            Ok(()) => {}
            Err(_e) => {}
        },
        _ => {}
    }
}

pub fn save_file_as(_ctx: &mut druid::DelegateCtx, cmd: &druid::Command, data: &mut AppState) {
    let file_info = cmd.get_unchecked(druid::commands::SAVE_FILE_AS);

    // If the file dialog passes us an invalid path then all bets are off. Just let it panic.
    let path = file_info.path().to_str().unwrap();

    match storage::png::write_path(path, &data.doc.pixels) {
        Ok(()) => {
            data.doc.path = Some(String::from(path));
        }
        Err(_e) => {}
    }
}

fn check_for_save(data: &mut AppState) {
    if data.doc.pixels.dirty {}
}
