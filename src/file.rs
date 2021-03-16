use std::fs::File;
use std::io::BufWriter;
use std::io::Error;
use std::io::ErrorKind;
use std::io::Result;
use std::path::Path;

use png;

use crate::model::AppState;

pub fn write_png(path_str: &str, data: &AppState) -> Result<()> {
    let path = Path::new(path_str);
    let file = File::create(path)?;
    let ref mut buf_writer = BufWriter::new(file);

    let mut encoder = png::Encoder::new(buf_writer, 32, 32);
    encoder.set_color(png::ColorType::RGBA);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();

    let mut bytes: [u8; 4096] = [0; 4096];
    for i in 0..data.pixels.len() {
        let j = i * 4;
        let chunk = data.pixels[i].to_be_bytes();
        bytes[j + 0] = chunk[0];
        bytes[j + 1] = chunk[1];
        bytes[j + 2] = chunk[2];
        bytes[j + 3] = chunk[3];
    }

    match writer.write_image_data(&bytes) {
        Ok(()) => Ok(()),
        Err(e) => Err(Error::new(ErrorKind::InvalidInput, e)),
    }
}

pub fn _read_png(_path: &str, _data: &mut AppState) -> Result<()> {
    Ok(())
}
