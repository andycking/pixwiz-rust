use crate::model::types::ToolType;

use std::ops::{Index, IndexMut};
use std::sync::Arc;

/// Pixel storage. Each value is stored as a u32 representation of RGBA, with the alpha value
/// in the least significant position. This matches what Color does internally. We hold the
/// values in an ARC, to avoid copying them.
#[derive(Clone, druid::Data)]
pub struct PixelState {
    pub dirty: bool,
    pub width: usize,
    pub height: usize,
    pub depth: u8,
    storage: Arc<Vec<u32>>,
}

impl PixelState {
    const DEFAULT_WIDTH: usize = 32;
    const DEFAULT_HEIGHT: usize = 32;
    const DEFAULT_DEPTH: u8 = 8;

    pub fn new(dirty: bool, width: usize, height: usize, depth: u8, vec: Vec<u32>) -> Self {
        assert!(width == 32);
        assert!(height == 32);
        assert!(depth == 8);

        Self {
            dirty: dirty,
            width: width,
            height: height,
            depth: depth,
            storage: Arc::new(vec),
        }
    }

    pub fn empty() -> Self {
        Self {
            dirty: false,
            width: Self::DEFAULT_WIDTH,
            height: Self::DEFAULT_HEIGHT,
            depth: Self::DEFAULT_DEPTH,
            storage: Arc::new(vec![0; Self::DEFAULT_WIDTH * Self::DEFAULT_HEIGHT]),
        }
    }

    #[inline]
    pub fn len(&self) -> usize {
        // We always want len and capacity to be the same. The entire vector must have been
        // initialized so that later on we don't access an invalid pixel.
        assert!(self.storage.len() == self.storage.capacity());
        self.storage.len()
    }

    /// Convert coordinates to an index within storage.
    #[inline]
    pub fn xy_to_idx(&self, x: usize, y: usize) -> usize {
        (y - 1) * self.height + (x - 1)
    }

    /// Convert point coordinates to an index within storage.
    #[inline]
    pub fn point_to_idx(&self, p: druid::Point) -> usize {
        self.xy_to_idx(p.x as usize, p.y as usize)
    }

    /// Write a value to an index in storage. This is a function and not an IndexMut
    /// because we want to control the dirty flag.
    #[inline]
    pub fn write(&mut self, idx: usize, value: u32) {
        *Arc::make_mut(&mut self.storage).index_mut(idx) = value;
        self.dirty = true;
    }

    /// Write a value to a block of storage. Probably a little bit faster than making
    /// multiple calls to write(). Probably. What even is a profiler?
    pub fn write_block(&mut self, x0: usize, y0: usize, x1: usize, y1: usize, value: u32) {
        let pixels = Arc::make_mut(&mut self.storage);

        for row in x0..x1 + 1 {
            for col in y0..y1 + 1 {
                // Why can't we use our own function? Because then we'd have a mutable
                // borrow followed by an immutable borrow. So just do our own inlining.
                let idx = (col - 1) * self.height + (row - 1);
                pixels[idx] = value;
            }
        }
        self.dirty = true;
    }
}

/// Implement the index trait. This is a convenient way for callers to access the pixel
/// storage directly. Note that it's immutable.
impl Index<usize> for PixelState {
    type Output = u32;

    #[inline]
    fn index(&self, idx: usize) -> &Self::Output {
        &self.storage[idx]
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
    pub path: Option<String>,
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
            pixels: PixelState::empty(),
            path: None,
        }
    }
}
