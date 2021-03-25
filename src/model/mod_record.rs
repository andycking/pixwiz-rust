use std::sync::Arc;

/// Modification record. This holds undo state.
#[derive(Clone, druid::Data)]
pub struct ModRecord {
    pub area: druid::Rect,
    pub bytes: Arc<Vec<u8>>,
}

impl ModRecord {
    pub fn new(area: druid::Rect, bytes: Vec<u8>) -> Self {
        Self {
            area: area,
            bytes: Arc::new(bytes),
        }
    }
}
