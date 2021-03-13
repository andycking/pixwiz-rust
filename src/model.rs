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
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

impl<T: Default + Ord> Point<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x: x, y: y }
    }

    pub fn empty() -> Self {
        Self {
            x: Default::default(),
            y: Default::default(),
        }
    }

    pub fn min(a: Self, b: Self) -> Self {
        Self::new(std::cmp::min(a.x, b.x), std::cmp::min(a.y, b.y))
    }

    pub fn max(a: Self, b: Self) -> Self {
        Self::new(std::cmp::max(a.x, b.x), std::cmp::max(a.y, b.y))
    }
}

impl<T: Default + Ord> From<(T, T)> for Point<T> {
    fn from(item: (T, T)) -> Self {
        Self::new(item.0, item.1)
    }
}

#[derive(Clone, Copy, druid::Data)]
pub struct Rect<T> {
    pub x0: T,
    pub y0: T,
    pub x1: T,
    pub y1: T,
}

impl<T: Default> Rect<T> {
    pub fn new(x0: T, y0: T, x1: T, y1: T) -> Self {
        Self {
            x0: x0,
            y0: y0,
            x1: x1,
            y1: y1,
        }
    }

    pub fn empty() -> Self {
        Self {
            x0: Default::default(),
            y0: Default::default(),
            x1: Default::default(),
            y1: Default::default(),
        }
    }
}

impl<T: Default + Ord> From<((T, T), (T, T))> for Rect<T> {
    fn from(item: ((T, T), (T, T))) -> Self {
        Self::new(item.0 .0, item.0 .1, item.1 .0, item.1 .1)
    }
}

impl<T: Default + Ord> From<(Point<T>, Point<T>)> for Rect<T> {
    fn from(item: (Point<T>, Point<T>)) -> Self {
        Self::new(item.0.x, item.0.y, item.1.x, item.1.y)
    }
}

#[derive(Clone, druid::Data)]
pub struct AppState {
    pub brush_color: u32,
    pub pos_color: u32,
    pub start_pos: Point<usize>,
    pub current_pos: Point<usize>,
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
