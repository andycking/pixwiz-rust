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

use std::fs::File;
use std::io::BufWriter;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use std::result::Result;

use png;

use super::error::StorageError;
use crate::model::pixel_header::PixelHeader;
use crate::model::pixel_state::PixelState;

/// Write pixel state to the given path as a PNG.
pub fn write_path(path_str: &str, pixels: &PixelState) -> Result<(), StorageError> {
    let path = Path::new(path_str);
    let file = File::create(path)?;
    let ref mut buf_writer = BufWriter::new(file);

    write(buf_writer, pixels)
}

/// Write pixel state to the given writer. This exists because I think we'll need to
/// write a fully formed PNG to memory for the clipboard.
pub fn write<W: Write>(writer: W, pixels: &PixelState) -> Result<(), StorageError> {
    let mut encoder = png::Encoder::new(
        writer,
        pixels.header.width as u32,
        pixels.header.height as u32,
    );
    encoder.set_color(png::ColorType::RGBA);
    encoder.set_depth(png::BitDepth::Eight);
    let mut encode_writer = encoder.write_header()?;

    // Oof. If this is a file the user loaded, then we're dropping all the other fields.
    // Someone is going to be super pissed when their file isn't the same.

    match encode_writer.write_image_data(&pixels.bytes) {
        Err(_) => Err(StorageError::new()),
        _ => Ok(()),
    }
}

/// Read a PNG from the given path into pixel state.
pub fn read_path(path_str: &str) -> Result<PixelState, StorageError> {
    let path = Path::new(path_str);
    let file = File::open(path)?;

    read(file)
}

/// Read a PNG from the given reader.
pub fn read<R: Read>(reader: R) -> Result<PixelState, StorageError> {
    let decoder = png::Decoder::new(reader);
    let (info, mut decode_reader) = decoder.read_info()?;

    // We support 8-bit PNGs in RGBA format for now. Let's at least be upfront about it.
    if info.bit_depth != png::BitDepth::Eight || info.color_type != png::ColorType::RGBA {
        return Err(StorageError::new());
    }

    let mut bytes = vec![0; info.buffer_size()];

    decode_reader.next_frame(&mut bytes)?;

    let header = PixelHeader::new(
        info.width as usize,
        info.height as usize,
        8, // But why you lying tho?
        4, // Ditto.
    );

    let pixels = PixelState::new(header, bytes);

    Ok(pixels)
}

impl From<png::EncodingError> for StorageError {
    fn from(_: png::EncodingError) -> Self {
        Self::new()
    }
}

impl From<png::DecodingError> for StorageError {
    fn from(_: png::DecodingError) -> Self {
        Self::new()
    }
}
