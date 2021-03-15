use std::fmt;
use std::ops::{Index, IndexMut};
use std::sync::Arc;

/// Pixel storage. Each value is stored as a u32 representation of RGBA, with the alpha value
/// in the least significant position. This matches what Color does internally. We hold the
/// values in an ARC, to avoid copying them.
#[derive(Clone, druid::Data)]
pub struct PixelState {
    dirty: bool,
    storage: Arc<[u32; 1024]>,
}

impl PixelState {
    pub fn new() -> Self {
        Self {
            dirty: false,
            storage: Arc::new([0; 1024]),
        }
    }

    pub fn len(&self) -> usize {
        self.storage.len()
    }

    pub fn is_dirty(&self) -> bool {
        self.dirty
    }

    pub fn write(&mut self, idx: usize, value: u32) {
        *Arc::make_mut(&mut self.storage).index_mut(idx) = value;
        self.dirty = true;
    }
}

impl Index<usize> for PixelState {
    type Output = u32;
    fn index(&self, idx: usize) -> &Self::Output {
        &self.storage[idx]
    }
}

/// Supported tool types.
#[derive(Clone, Copy, druid::Data, Debug, PartialEq)]
pub enum ToolType {
    Cropper,
    Dropper,
    Eraser,
    Fill,
    Lasso,
    Marquee,
    Move,
    Paint,
}

impl fmt::Display for ToolType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// Application state.
#[derive(Clone, druid::Data)]
pub struct AppState {
    pub brush_color: u32,
    pub pos_color: u32,
    pub start_pos: druid::Point,
    pub current_pos: druid::Point,
    pub selection: Option<druid::Rect>,
    pub tool_type: ToolType,
    pub pixels: PixelState,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            brush_color: 0x0ff,
            pos_color: 0x0ff,
            start_pos: druid::Point::ZERO,
            current_pos: druid::Point::ZERO,
            selection: None,
            tool_type: ToolType::Paint,
            pixels: PixelState::new(),
        }
    }
}
