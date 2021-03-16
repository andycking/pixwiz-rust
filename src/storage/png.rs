use std::fs::File;
use std::io::BufWriter;
use std::io::Error;
use std::io::ErrorKind;
use std::io::Result;
use std::path::Path;

use png;

use crate::storage::image_data::ImageData;

pub fn write(path_str: &str, data: &ImageData) -> Result<()> {
    let path = Path::new(path_str);
    let file = File::create(path)?;
    let ref mut buf_writer = BufWriter::new(file);

    let mut encoder = png::Encoder::new(buf_writer, data.width as u32, data.height as u32);
    encoder.set_color(png::ColorType::RGBA);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header()?;

    match writer.write_image_data(&data.bytes) {
        Ok(()) => Ok(()),
        Err(e) => Err(Error::new(ErrorKind::InvalidInput, e)),
    }
}

pub fn read(path_str: &str) -> Result<ImageData> {
    let path = Path::new(path_str);
    let file = File::open(path)?;

    let decoder = png::Decoder::new(file);
    let (info, mut reader) = decoder.read_info()?;

    let mut bytes = vec![0; info.buffer_size()];

    reader.next_frame(&mut bytes)?;

    Ok(ImageData::new(
        info.width as usize,
        info.height as usize,
        8,
        None,
        bytes,
    ))
}
