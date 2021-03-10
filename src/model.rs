use std::fmt;
use std::ops::{Index, IndexMut};
use std::sync::Arc;

use druid::{Color, Data};

#[derive(Clone, Data)]
pub struct PixelState {
    storage: Arc<[u32; 1024]>,
}

impl PixelState {
    pub fn new() -> Self {
        Self {
            storage: Arc::new([0; 1024]),
        }
    }

    pub fn len(&self) -> usize {
        self.storage.len()
    }
}

impl Index<usize> for PixelState {
    type Output = u32;
    fn index(&self, idx: usize) -> &Self::Output {
        &self.storage[idx]
    }
}

impl IndexMut<usize> for PixelState {
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        Arc::make_mut(&mut self.storage).index_mut(idx)
    }
}

#[derive(Clone, Copy, Data, Debug, PartialEq)]
pub enum ToolType {
    Marquee,
    Lasso,
    Move,
    Zoom,
    Cropper,
    Type,
    Paint,
    Eraser,
    Fill,
    Dropper,
}

impl fmt::Display for ToolType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Clone, Data)]
pub struct AppState {
    pub brush_color: u32,
    pub pos_color: u32,
    pub pos: (usize, usize),
    pub tool_type: ToolType,
    pub pixels: PixelState,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            brush_color: Color::BLACK.as_rgba_u32(),
            pos_color: Color::BLACK.as_rgba_u32(),
            pos: (0, 0),
            tool_type: ToolType::Paint,
            pixels: PixelState::new(),
        }
    }
}
