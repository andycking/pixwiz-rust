use crate::model::types::PixelHeader;
use crate::model::types::ToolType;

use std::sync::Arc;

/// Pixel storage. Each value is stored as four contiguous bytes representing RGBA,
/// respectively. We hold the values in an ARC, to avoid copying them.
#[derive(Clone, druid::Data)]
pub struct PixelState {
    pub header: PixelHeader,
    pub dirty: bool,
    pub bytes: Arc<Vec<u8>>,
}

impl PixelState {
    /// Create new pixel state with given bytes.
    pub fn new(header: PixelHeader, bytes: Vec<u8>) -> Self {
        assert!(bytes.len() == header.width * header.height * header.bytes_per_pixel);

        Self {
            header: header,
            dirty: false,
            bytes: Arc::new(bytes),
        }
    }

    /// Get the length of the pixel state in bytes.
    #[inline]
    pub fn len(&self) -> usize {
        // We always want len and capacity to be the same. The entire vector must have been
        // initialized so that later on we don't access an invalid pixel.
        assert!(self.bytes.len() == self.bytes.capacity());
        self.bytes.len() / self.header.bytes_per_pixel
    }

    /// Convert coordinates to an index within storage.
    #[inline]
    pub fn xy_to_idx(&self, x: usize, y: usize) -> usize {
        (y - 1) * self.header.width + (x - 1)
    }

    /// Convert point coordinates to an index within storage.
    #[inline]
    pub fn point_to_idx(&self, p: druid::Point) -> usize {
        self.xy_to_idx(p.x as usize, p.y as usize)
    }

    /// Read a value from an index in storage.
    #[inline]
    pub fn read(&self, idx: usize) -> druid::Color {
        let byte_idx = idx * self.header.bytes_per_pixel;
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
        let byte_idx = idx * self.header.bytes_per_pixel;
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
        F: Fn(&PixelHeader, &mut Vec<u8>, (usize, usize, usize, usize)),
    {
        let bounds = match selection {
            Some(rect) => (
                rect.x0 as usize,
                rect.y0 as usize,
                rect.x1 as usize,
                rect.y1 as usize,
            ),
            _ => (1, 1, self.header.width, self.header.height),
        };

        let bytes = Arc::make_mut(&mut self.bytes);
        f(&self.header, bytes, bounds);

        self.dirty = true;
    }
}

impl Default for PixelState {
    fn default() -> Self {
        let header: PixelHeader = Default::default();

        let size: usize = header.width * header.height * header.bytes_per_pixel;

        Self {
            header: header,
            dirty: false,
            bytes: Arc::new(vec![0; size]),
        }
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
            pixels: Default::default(),
            path: None,
        }
    }
}
