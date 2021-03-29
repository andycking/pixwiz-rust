use std::collections::VecDeque;
use std::sync::Arc;

use crate::model::mod_record::ModRecord;

/// Depth of the modification stack. This seems big, but remember that we're dealing
/// with tiny little bitmaps, and we only record what's changed.
const STACK_DEPTH: usize = 16;

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

    /// Clear out the modification stack.
    pub fn clear(&mut self) {
        let q = Arc::make_mut(&mut self.q);
        q.clear();
    }
}
