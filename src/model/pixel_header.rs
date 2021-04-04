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

/// Generic pixel header.
#[derive(Clone, druid::Data)]
pub struct PixelHeader {
    pub width: usize,
    pub height: usize,
    pub depth: u8,
    pub bytes_per_pixel: usize,
}

impl PixelHeader {
    const DEFAULT_WIDTH: usize = 32;
    const DEFAULT_HEIGHT: usize = 32;
    const DEFAULT_DEPTH: u8 = 8;
    const DEFAULT_BYTES_PER_PIXEL: usize = 4;

    pub fn new(width: usize, height: usize, depth: u8, bytes_per_pixel: usize) -> Self {
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
