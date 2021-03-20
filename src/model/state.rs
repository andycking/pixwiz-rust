use crate::model::types::ToolType;

use std::sync::Arc;

/// Pixel storage. Each value is stored as four contiguous bytes representing RGBA,
/// respectively. We hold the values in an ARC, to avoid copying them.
#[derive(Clone, druid::Data)]
pub struct PixelState {
    pub dirty: bool,
    pub width: usize,
    pub height: usize,
    pub depth: u8,
    pub bytes: Arc<Vec<u8>>,
}

impl PixelState {
    const DEFAULT_WIDTH: usize = 32;
    const DEFAULT_HEIGHT: usize = 32;
    const DEFAULT_DEPTH: u8 = 8;

    /// Create new pixel state with given bytes.
    pub fn new(width: usize, height: usize, depth: u8, bytes: Vec<u8>) -> Self {
        assert!(width == 32);
        assert!(height == 32);
        assert!(depth == 8);
        assert!(bytes.len() == width * height * 4);

        Self {
            dirty: false,
            width: width,
            height: height,
            depth: depth,
            bytes: Arc::new(bytes),
        }
    }

    /// Create an empty (transparent) pixel state of the default size.
    pub fn empty() -> Self {
        let size = Self::DEFAULT_WIDTH * Self::DEFAULT_HEIGHT * 4;

        Self {
            dirty: false,
            width: Self::DEFAULT_WIDTH,
            height: Self::DEFAULT_HEIGHT,
            depth: Self::DEFAULT_DEPTH,
            bytes: Arc::new(vec![0; size]),
        }
    }

    /// Get the length of the pixel state in bytes.
    #[inline]
    pub fn len(&self) -> usize {
        // We always want len and capacity to be the same. The entire vector must have been
        // initialized so that later on we don't access an invalid pixel.
        assert!(self.bytes.len() == self.bytes.capacity());
        self.bytes.len() / 4
    }

    /// Convert coordinates to an index within storage.
    #[inline]
    pub fn xy_to_idx(&self, x: usize, y: usize) -> usize {
        Self::xy_to_idx_helper(self.height, x, y)
    }

    pub fn xy_to_idx_helper(height: usize, x: usize, y: usize) -> usize {
        (y - 1) * height + (x - 1)
    }

    pub fn byte_idx(idx: usize) -> usize {
        idx * 4
    }

    /// Convert point coordinates to an index within storage.
    #[inline]
    pub fn point_to_idx(&self, p: druid::Point) -> usize {
        self.xy_to_idx(p.x as usize, p.y as usize)
    }

    /// Read a value from an index in storage.
    #[inline]
    pub fn read(&self, idx: usize) -> druid::Color {
        let byte_idx = Self::byte_idx(idx);
        druid::Color::rgba8(
            self.bytes[byte_idx + 0],
            self.bytes[byte_idx + 1],
            self.bytes[byte_idx + 2],
            self.bytes[byte_idx + 3],
        )
    }

    /// Write a value to an index in storage. This is a function and not an IndexMut
    /// because we want to control the dirty flag.
    #[inline]
    pub fn write(&mut self, idx: usize, color: &druid::Color) {
        let byte_idx = Self::byte_idx(idx);
        let (r, g, b, a) = color.as_rgba8();

        let pixels = Arc::make_mut(&mut self.bytes);
        pixels[byte_idx + 0] = r;
        pixels[byte_idx + 1] = g;
        pixels[byte_idx + 2] = b;
        pixels[byte_idx + 3] = a;

        self.dirty = true;
    }

    /// Apply a transformation to the pixels, or some selection thereof.
    pub fn apply<F>(&mut self, selection: Option<druid::Rect>, f: F)
    where
        F: Fn(usize, usize, &mut Vec<u8>, &druid::Rect),
    {
        let bounds = selection.unwrap_or(druid::Rect::new(
            0.0,
            0.0,
            self.width as f64,
            self.height as f64,
        ));

        let bytes = Arc::make_mut(&mut self.bytes);
        f(self.width, self.height, bytes, &bounds);

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
    /// Create default application state.
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
