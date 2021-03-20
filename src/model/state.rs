use crate::model::types::ToolType;

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
    pub storage: Arc<Vec<u8>>,
}

impl PixelState {
    const DEFAULT_WIDTH: usize = 32;
    const DEFAULT_HEIGHT: usize = 32;
    const DEFAULT_DEPTH: u8 = 8;

    pub fn new(width: usize, height: usize, depth: u8, vec: Vec<u8>) -> Self {
        assert!(width == 32);
        assert!(height == 32);
        assert!(depth == 8);
        assert!(vec.len() == width * height * 4);

        Self {
            dirty: false,
            width: width,
            height: height,
            depth: depth,
            storage: Arc::new(vec),
        }
    }

    pub fn empty() -> Self {
        let size = Self::DEFAULT_WIDTH * Self::DEFAULT_HEIGHT * 4;

        Self {
            dirty: false,
            width: Self::DEFAULT_WIDTH,
            height: Self::DEFAULT_HEIGHT,
            depth: Self::DEFAULT_DEPTH,
            storage: Arc::new(vec![0; size]),
        }
    }

    #[inline]
    pub fn len(&self) -> usize {
        // We always want len and capacity to be the same. The entire vector must have been
        // initialized so that later on we don't access an invalid pixel.
        assert!(self.storage.len() == self.storage.capacity());
        self.storage.len() / 4
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

    #[inline]
    pub fn read(&self, idx: usize) -> druid::Color {
        let byte_idx = idx * 4;
        druid::Color::rgba8(
            self.storage[byte_idx + 0],
            self.storage[byte_idx + 1],
            self.storage[byte_idx + 2],
            self.storage[byte_idx + 3],
        )
    }

    /// Write a value to an index in storage. This is a function and not an IndexMut
    /// because we want to control the dirty flag.
    #[inline]
    pub fn write(&mut self, idx: usize, color: &druid::Color) {
        let byte_idx = idx * 4;
        let (r, g, b, a) = color.as_rgba8();

        let pixels = Arc::make_mut(&mut self.storage);
        pixels[byte_idx + 0] = r;
        pixels[byte_idx + 1] = g;
        pixels[byte_idx + 2] = b;
        pixels[byte_idx + 3] = a;

        self.dirty = true;
    }

    /// Write a value to a block of storage. Probably a little bit faster than making
    /// multiple calls to write(). Probably. What even is a profiler? This is used by the
    /// eraser tool.
    pub fn write_block(
        &mut self,
        x0: usize,
        y0: usize,
        x1: usize,
        y1: usize,
        color: &druid::Color,
    ) {
        let pixels = Arc::make_mut(&mut self.storage);

        for row in x0..x1 + 1 {
            for col in y0..y1 + 1 {
                // Why can't we use our own function? Because then we'd have a mutable
                // borrow followed by an immutable borrow. So just do our own inlining.
                let idx = (col - 1) * self.height + (row - 1);
                let byte_idx = idx * 4;

                let (r, g, b, a) = color.as_rgba8();
                pixels[byte_idx + 0] = r;
                pixels[byte_idx + 1] = g;
                pixels[byte_idx + 2] = b;
                pixels[byte_idx + 3] = a;
            }
        }
        self.dirty = true;
    }
}

/// Application state.
#[derive(Clone, druid::Data)]
pub struct AppState {
    pub brush_color: druid::Color,
    pub pos_color: druid::Color,
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
            brush_color: druid::Color::BLACK,
            pos_color: druid::Color::rgba8(0, 0, 0, 0),
            start_pos: druid::Point::ZERO,
            current_pos: druid::Point::ZERO,
            selection: None,
            tool_type: ToolType::Paint,
            pixels: PixelState::empty(),
            path: None,
        }
    }
}
