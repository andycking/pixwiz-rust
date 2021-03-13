use std::fmt;
use std::ops::{Index, IndexMut};
use std::sync::Arc;

#[derive(Clone, druid::Data)]
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

#[derive(Clone, Copy, druid::Data)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl Point {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x: x, y: y }
    }

    pub fn empty() -> Self {
        Self { x: 0, y: 0 }
    }
}

impl From<(usize, usize)> for Point {
    fn from(item: (usize, usize)) -> Self {
        Self::new(item.0, item.1)
    }
}

impl From<(f64, f64)> for Point {
    fn from(item: (f64, f64)) -> Self {
        Self::new(item.0 as usize, item.1 as usize)
    }
}

#[derive(Clone, druid::Data)]
pub struct AppState {
    pub brush_color: u32,
    pub pos_color: u32,
    pub start_pos: Point,
    pub current_pos: Point,
    pub selection: ((usize, usize), (usize, usize)),
    pub tool_type: ToolType,
    pub pixels: PixelState,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            brush_color: 0x0ff,
            pos_color: 0x0ff,
            start_pos: Point::empty(),
            current_pos: Point::empty(),
            selection: ((0, 0), (0, 0)),
            tool_type: ToolType::Paint,
            pixels: PixelState::new(),
        }
    }

    pub fn has_selection(&self) -> bool {
        self.selection != ((0, 0), (0, 0))
    }
}
