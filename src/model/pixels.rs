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

use crate::common::constants;
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
    pub fn new(width: u32, height: u32, depth: u8, bytes_per_pixel: u8) -> Self {
        Self {
            width,
            height,
            depth,
            bytes_per_pixel,
        }
    }

    /// Get the width, in pixels.
    pub fn width(&self) -> usize {
        self.width as usize
    }

    /// Get the height, in pixels.
    pub fn height(&self) -> usize {
        self.height as usize
    }

    /// Get the number of bytes per pixel.
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
            width: constants::DEFAULT_PIXEL_DIMS,
            height: constants::DEFAULT_PIXEL_DIMS,
            depth: 8,
            bytes_per_pixel: 4,
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

    /// Get the current color.
    pub fn color(&self) -> &druid::Color {
        &self.color
    }

    /// Get the current mouse position.
    pub fn pos(&self) -> druid::Point {
        self.pos
    }

    /// Get the bounds. Can be selection or entire image.
    pub fn bounds(&self) -> druid::Rect {
        self.bounds
    }

    /// Get the parameter that was passed.
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
    bytes: PixelBytes,
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

    /// Are pixels dirty?
    pub fn dirty(&self) -> bool {
        self.dirty
    }

    /// Clear dirty flag.
    pub fn clear_dirty(&mut self) {
        self.dirty = false;
    }

    /// Get the pixel header.
    pub fn header(&self) -> &PixelHeader {
        &self.header
    }

    /// Get pixel bytes.
    pub fn bytes(&self) -> &Vec<u8> {
        &self.bytes
    }

    /// Set the pixel bytes.
    pub fn set_bytes(&mut self, bytes: Vec<u8>) {
        self.bytes = Arc::new(bytes);
        self.dirty = true;
    }

    /// Determine if the given point is contained within the pixel bounds.
    #[inline]
    pub fn contains(&self, p: druid::Point) -> bool {
        self.contains_xy(p.x as usize, p.y as usize)
    }

    /// Determine if the given point is contained with the pixel bounds.
    #[inline]
    pub fn contains_xy(&self, x: usize, y: usize) -> bool {
        x > 0 && y > 0 && x <= self.header.width() && y <= self.header.height()
    }

    #[inline]
    fn xy_to_idx(&self, x: usize, y: usize) -> usize {
        assert!(x > 0);
        assert!(y > 0);
        let stride = self.header.width();
        (y - 1) * stride + (x - 1)
    }

    #[inline]
    fn xy_to_byte_idx(&self, x: usize, y: usize) -> usize {
        self.xy_to_idx(x, y) * self.header.bytes_per_pixel as usize
    }

    /// Read from an xy point in pixel storage. Will panic if outside bounds.
    pub fn read_xy_unchecked(&self, x: usize, y: usize) -> druid::Color {
        let byte_idx = self.xy_to_byte_idx(x, y);

        druid::Color::rgba8(
            self.bytes[byte_idx],
            self.bytes[byte_idx + 1],
            self.bytes[byte_idx + 2],
            self.bytes[byte_idx + 3],
        )
    }

    /// Read from a point in pixel storage. Will panic if outside bounds.
    #[inline]
    pub fn read_unchecked(&self, p: druid::Point) -> druid::Color {
        self.read_xy_unchecked(p.x as usize, p.y as usize)
    }

    /// Safe way to read a point in pixel storage. Will return an empty color if outside bounds.
    pub fn read(&self, p: druid::Point) -> druid::Color {
        if self.contains(p) {
            self.read_unchecked(p)
        } else {
            druid::Color::rgba8(0, 0, 0, 0)
        }
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

    /// Write to a point in pixel storage.
    pub fn write(&mut self, p: druid::Point, color: &druid::Color) {
        let byte_idx = self.xy_to_byte_idx(p.x as usize, p.y as usize);
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
