use std::sync::Arc;

use crate::model::pixel_header::PixelHeader;
use crate::model::types::PixelBytes;

/// Pixel storage. Each value is stored as four contiguous bytes representing RGBA,
/// respectively. We hold the values in an ARC, to avoid copying them.
#[derive(Clone, druid::Data)]
pub struct PixelState {
    pub header: PixelHeader,
    pub dirty: bool,
    pub bytes: PixelBytes,
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

    /// Read an area of storage.
    pub fn read_area(&self, area: druid::Rect) -> Vec<u8> {
        let dst_size = (area.width() * area.height()) as usize * self.header.bytes_per_pixel;
        let mut dst_bytes = Vec::with_capacity(dst_size);

        for y in area.y0 as usize..area.y1 as usize {
            for x in area.x0 as usize..area.x1 as usize {
                let idx = (y - 1) * self.header.width + (x - 1);
                let src_idx = idx * self.header.bytes_per_pixel;

                dst_bytes.push(self.bytes[src_idx + 0]);
                dst_bytes.push(self.bytes[src_idx + 1]);
                dst_bytes.push(self.bytes[src_idx + 2]);
                dst_bytes.push(self.bytes[src_idx + 3]);
            }
        }

        dst_bytes
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

    /// Write an area of storage.
    pub fn write_area(&mut self, area: druid::Rect, src_bytes: &Vec<u8>) {
        let dst_bytes = Arc::make_mut(&mut self.bytes);

        let mut src_idx = 0;

        for y in area.y0 as usize..area.y1 as usize {
            for x in area.x0 as usize..area.x1 as usize {
                let idx = (y - 1) * self.header.width + (x - 1);
                let dst_idx = idx * self.header.bytes_per_pixel;

                dst_bytes[dst_idx + 0] = src_bytes[src_idx + 0];
                dst_bytes[dst_idx + 1] = src_bytes[src_idx + 1];
                dst_bytes[dst_idx + 2] = src_bytes[src_idx + 2];
                dst_bytes[dst_idx + 3] = src_bytes[src_idx + 3];

                src_idx += 4;
            }
        }

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
