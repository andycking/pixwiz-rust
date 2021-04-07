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

use std::collections::VecDeque;
use std::sync::Arc;

use crate::model::types::PixelBytes;

/// Depth of the modification stack. This seems big, but remember that we're dealing
/// with tiny little bitmaps, and we only record what's changed.
const STACK_DEPTH: usize = 16;

/// Modification record. This holds undo state.
#[derive(Clone, druid::Data)]
pub struct ModRecord {
    pub area: druid::Rect,
    pub bytes: PixelBytes,
}

impl ModRecord {
    pub fn new(area: druid::Rect, bytes: Vec<u8>) -> Self {
        Self {
            area,
            bytes: Arc::new(bytes),
        }
    }
}

/// Stack of modification records. Used for undo and redo.
#[derive(Clone, druid::Data)]
pub struct ModStack {
    q: Arc<VecDeque<ModRecord>>,
}

impl Default for ModStack {
    fn default() -> Self {
        Self {
            q: Arc::new(VecDeque::with_capacity(STACK_DEPTH)),
        }
    }
}

impl ModStack {
    /// Push a modification record onto the stack. This will maintain the stack depth;
    /// any items beyond the initial capacity are discarded.
    pub fn push(&mut self, record: ModRecord) {
        let q = Arc::make_mut(&mut self.q);
        q.push_front(record);
        q.truncate(STACK_DEPTH);
    }

    /// Pop a modification record from the stack.
    pub fn pop(&mut self) -> Option<ModRecord> {
        let q = Arc::make_mut(&mut self.q);
        q.pop_front()
    }

    pub fn is_empty(&self) -> bool {
        self.q.is_empty()
    }

    /// Clear out the modification stack.
    pub fn clear(&mut self) {
        let q = Arc::make_mut(&mut self.q);
        q.clear();
    }
}
