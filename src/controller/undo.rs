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
use crate::model::mod_stack::ModRecord;

/// Push a point onto the undo stack.
pub fn push_point(data: &mut AppState, p: druid::Point) {
    let area = druid::Rect::new(p.x, p.y, p.x + 1.0, p.y + 1.0);
    push(data, area);
}

/// Push an area onto the undo stack.
pub fn push(data: &mut AppState, area: druid::Rect) {
    push_inner(data, area);

    // Important: reset the redo stack!
    // This is okay: undo -> undo -> redo -> redo
    // This is not okay: undo -> paint -> redo
    data.doc_mut().redo_mut().clear();
}

fn push_inner(data: &mut AppState, area: druid::Rect) {
    let bytes = data.doc().pixels().read_area(area);
    let record = ModRecord::new(area, bytes);

    data.doc_mut().undo_mut().push(record);
}

/// Pop a record from the undo stack and apply it.
pub fn pop(data: &mut AppState) {
    if let Some(record) = data.doc_mut().undo_mut().pop() {
        // Before we undo, record what we just did, so that we can redo it again.
        let area = record.area();
        push_redo(data, area);

        data.doc_mut().pixels_mut().write_area(area, record.bytes());
    }
}

fn push_redo(data: &mut AppState, area: druid::Rect) {
    let bytes = data.doc().pixels().read_area(area);
    let record = ModRecord::new(area, bytes);

    data.doc_mut().redo_mut().push(record);
}

/// Pop a record from the redo stack and apply it.
pub fn pop_redo(data: &mut AppState) {
    if let Some(record) = data.doc_mut().redo_mut().pop() {
        // Before we redo, record what we just did, so that we can undo it again.
        // But call the inner function, so that we don't reset the redo stack!
        let area = record.area();
        push_inner(data, record.area());

        data.doc_mut().pixels_mut().write_area(area, record.bytes());
    }
}
