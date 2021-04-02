use std::fs::File;
use std::io::BufWriter;
use std::path::Path;
use std::result::Result;

use png;

use super::error::StorageError;
use crate::model::pixel_header::PixelHeader;
use crate::model::pixel_state::PixelState;

pub fn write(path_str: &str, pixels: &PixelState) -> Result<(), StorageError> {
    let path = Path::new(path_str);
    let file = File::create(path)?;
    let ref mut buf_writer = BufWriter::new(file);

    let mut encoder = png::Encoder::new(
        buf_writer,
        pixels.header.width as u32,
        pixels.header.height as u32,
    );
    encoder.set_color(png::ColorType::RGBA);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header()?;

    // Oof. If this is a file the user loaded, then we're dropping all the other fields.
    // Someone is going to be super pissed when their file isn't the same.

    match writer.write_image_data(&pixels.bytes) {
        Err(_) => Err(StorageError::new()),
        _ => Ok(()),
    }
}

pub fn read(path_str: &str) -> Result<PixelState, StorageError> {
    let path = Path::new(path_str);
    let file = File::open(path)?;

    let decoder = png::Decoder::new(file);
    let (info, mut reader) = decoder.read_info()?;

    // We support 8-bit PNGs in RGBA format for now. Let's at least be upfront about it.
    if info.bit_depth != png::BitDepth::Eight || info.color_type != png::ColorType::RGBA {
        return Err(StorageError::new());
    }

    let mut bytes = vec![0; info.buffer_size()];

    reader.next_frame(&mut bytes)?;

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
