// Copyright 2021 Andy King
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::sync::Arc;

use crate::model::types::PixelBytes;

/// Generic pixel header.
#[derive(Clone, druid::Data)]
pub struct PixelHeader {
    width: u32,
    height: u32,
    depth: u8,
    bytes_per_pixel: u8,
}

impl PixelHeader {
    const DEFAULT_WIDTH: u32 = 32;
    const DEFAULT_HEIGHT: u32 = 32;
    const DEFAULT_DEPTH: u8 = 8;
    const DEFAULT_BYTES_PER_PIXEL: u8 = 4;

    pub fn new(width: u32, height: u32, depth: u8, bytes_per_pixel: u8) -> Self {
        assert!(width == Self::DEFAULT_WIDTH);
        assert!(height == Self::DEFAULT_HEIGHT);
        assert!(depth == Self::DEFAULT_DEPTH);
        assert!(bytes_per_pixel == Self::DEFAULT_BYTES_PER_PIXEL);

        Self {
            width,
            height,
            depth,
            bytes_per_pixel,
        }
    }

    pub fn width(&self) -> usize {
        self.width as usize
    }

    pub fn height(&self) -> usize {
        self.height as usize
    }

    pub fn bytes_per_pixel(&self) -> u8 {
        self.bytes_per_pixel
    }

    /// Get bounding box for pixels.
    pub fn bounds(&self) -> druid::Rect {
        druid::Rect::new(1.0, 1.0, self.width as f64, self.height as f64)
    }
}

impl Default for PixelHeader {
    fn default() -> Self {
        Self {
            width: Self::DEFAULT_WIDTH,
            height: Self::DEFAULT_HEIGHT,
            depth: Self::DEFAULT_DEPTH,
            bytes_per_pixel: Self::DEFAULT_BYTES_PER_PIXEL,
        }
    }
}

/// Capture environment.
pub struct PixelEnv {
    color: druid::Color,
    pos: druid::Point,
    bounds: druid::Rect,
    param: f64,
}

impl PixelEnv {
    pub fn new(color: druid::Color, pos: druid::Point, bounds: druid::Rect, param: f64) -> Self {
        Self {
            color,
            pos,
            bounds,
            param,
        }
    }

    pub fn color(&self) -> &druid::Color {
        &self.color
    }

    pub fn pos(&self) -> &druid::Point {
        &self.pos
    }

    pub fn bounds(&self) -> &druid::Rect {
        &self.bounds
    }

    pub fn param(&self) -> f64 {
        self.param
    }
}

/// Pixel storage. Each value is stored as four contiguous bytes representing RGBA,
/// respectively. We hold the values in an ARC, to avoid copying them.
#[derive(Clone, druid::Data)]
pub struct PixelState {
    header: PixelHeader,
    dirty: bool,
    pub bytes: PixelBytes,
}

impl PixelState {
    /// Create new pixel state with given bytes.
    pub fn new(header: PixelHeader, bytes: Vec<u8>) -> Self {
        let dim = header.width() * header.height();
        let len = dim * header.bytes_per_pixel as usize;
        assert!(bytes.len() == len);

        Self {
            header,
            dirty: false,
            bytes: Arc::new(bytes),
        }
    }

    pub fn dirty(&self) -> bool {
        self.dirty
    }

    pub fn set_dirty(&mut self) {
        self.dirty = true;
    }

    pub fn header(&self) -> &PixelHeader {
        &self.header
    }

    /// Get the length of the pixel state in bytes.
    #[inline]
    pub fn len(&self) -> usize {
        // We always want len and capacity to be the same. The entire vector must have been
        // initialized so that later on we don't access an invalid pixel.
        assert!(self.bytes.len() == self.bytes.capacity());
        self.bytes.len() / self.header.bytes_per_pixel as usize
    }

    /// Convert coordinates to an index within storage.
    #[inline]
    pub fn xy_to_idx(&self, x: usize, y: usize) -> usize {
        (y - 1) * self.header.width() + (x - 1)
    }

    /// Convert point coordinates to an index within storage.
    #[inline]
    pub fn point_to_idx(&self, p: druid::Point) -> usize {
        self.xy_to_idx(p.x as usize, p.y as usize)
    }

    /// Read a value from an index in storage.
    #[inline]
    pub fn read(&self, idx: usize) -> druid::Color {
        let byte_idx = idx * self.header.bytes_per_pixel as usize;

        druid::Color::rgba8(
            self.bytes[byte_idx],
            self.bytes[byte_idx + 1],
            self.bytes[byte_idx + 2],
            self.bytes[byte_idx + 3],
        )
    }

    /// Read an area of storage.
    pub fn read_area(&self, area: druid::Rect) -> Vec<u8> {
        let dim = (area.width() * area.height()) as usize;
        let len = dim * self.header.bytes_per_pixel as usize;
        let mut dst_bytes = Vec::with_capacity(len);

        for y in area.y0 as usize..area.y1 as usize {
            for x in area.x0 as usize..area.x1 as usize {
                let idx = (y - 1) * self.header.width() + (x - 1);
                let src_idx = idx * self.header.bytes_per_pixel as usize;

                dst_bytes.extend_from_slice(&self.bytes[src_idx..src_idx + 4]);
            }
        }

        dst_bytes
    }

    /// Write a value to an index in storage. This is a function and not an IndexMut
    /// because we want to control the dirty flag.
    #[inline]
    pub fn write(&mut self, idx: usize, color: &druid::Color) {
        let byte_idx = idx * self.header.bytes_per_pixel as usize;
        let (red, green, blue, alpha) = color.as_rgba8();

        let pixels = Arc::make_mut(&mut self.bytes);
        pixels[byte_idx] = red;
        pixels[byte_idx + 1] = green;
        pixels[byte_idx + 2] = blue;
        pixels[byte_idx + 3] = alpha;

        self.dirty = true;
    }

    /// Write an area of storage.
    pub fn write_area(&mut self, area: druid::Rect, src_bytes: &[u8]) {
        let dst_bytes = Arc::make_mut(&mut self.bytes);

        let mut src_idx = 0;

        for y in area.y0 as usize..area.y1 as usize {
            for x in area.x0 as usize..area.x1 as usize {
                let idx = (y - 1) * self.header.width() + (x - 1);
                let dst_idx = idx * self.header.bytes_per_pixel as usize;

                dst_bytes[dst_idx] = src_bytes[src_idx];
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

        let dim = header.width() * header.height();
        let size = dim * header.bytes_per_pixel as usize;

        Self {
            header,
            dirty: false,
            bytes: Arc::new(vec![0; size]),
        }
    }
}
