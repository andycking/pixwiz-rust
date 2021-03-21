use crate::model::types::PixelHeader;

pub fn read(x: usize, y: usize, header: &PixelHeader, bytes: &mut Vec<u8>) -> (u8, u8, u8, u8) {
    let idx = (y - 1) * header.height + (x - 1);
    let byte_idx = idx * header.bytes_per_pixel;

    let r = bytes[byte_idx + 0];
    let g = bytes[byte_idx + 1];
    let b = bytes[byte_idx + 2];
    let a = bytes[byte_idx + 3];

    (r, g, b, a)
}

pub fn write(
    x: usize,
    y: usize,
    header: &PixelHeader,
    bytes: &mut Vec<u8>,
    r: u8,
    g: u8,
    b: u8,
    a: u8,
) {
    let idx = (y - 1) * header.height + (x - 1);
    let byte_idx = idx * header.bytes_per_pixel;

    bytes[byte_idx + 0] = r;
    bytes[byte_idx + 1] = g;
    bytes[byte_idx + 2] = b;
    bytes[byte_idx + 3] = a;
}
