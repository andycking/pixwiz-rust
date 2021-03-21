use crate::model::types::PixelHeader;

/// Read RGBA from bytes. The underlying storage doesn't really matter: it can be a
/// PixelState, or a copy thereof, or something else, as long as it's bytes.
pub fn read(x: usize, y: usize, header: &PixelHeader, bytes: &Vec<u8>) -> (u8, u8, u8, u8) {
    let idx = (y - 1) * header.height + (x - 1);
    let byte_idx = idx * header.bytes_per_pixel;

    let r = bytes[byte_idx + 0];
    let g = bytes[byte_idx + 1];
    let b = bytes[byte_idx + 2];
    let a = bytes[byte_idx + 3];

    (r, g, b, a)
}

/// Write RGBA to bytes. The underlying storage doesn't really matter; it can be a
/// PixelState, or a copy thereof, or something else, as long as it's bytes.
pub fn write(
    x: usize,
    y: usize,
    header: &PixelHeader,
    bytes: &mut Vec<u8>,
    rgba: (u8, u8, u8, u8),
) {
    let idx = (y - 1) * header.height + (x - 1);
    let byte_idx = idx * header.bytes_per_pixel;

    bytes[byte_idx + 0] = rgba.0;
    bytes[byte_idx + 1] = rgba.1;
    bytes[byte_idx + 2] = rgba.2;
    bytes[byte_idx + 3] = rgba.3;
}
