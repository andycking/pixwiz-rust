use std::sync::Arc;

use crate::model;
use crate::model::app_state::AppState;
use crate::model::pixel_env::PixelEnv;
use crate::model::pixel_header::PixelHeader;

pub mod colors;
pub mod simple;

pub fn apply<F>(data: &mut AppState, f: F)
where
    F: Fn(&PixelHeader, &PixelEnv, &mut Vec<u8>),
{
    // We have all the information we need for a mod record, so just create it here.
    // That way the caller, and the f() we're applying, don't need to worry about it.
    let bounds = model::get_bounds(data);
    model::push_undo(data, bounds);

    let env = PixelEnv::new(data.brush_color.clone(), data.current_pos, bounds);
    let bytes = Arc::make_mut(&mut data.pixels.bytes);

    f(&data.pixels.header, &env, bytes);

    data.pixels.dirty = true;
}

/// Read RGBA from bytes. The underlying storage doesn't really matter: it can be a
/// PixelState, or a copy thereof, or something else, as long as it's bytes.
fn read(x: usize, y: usize, header: &PixelHeader, bytes: &Vec<u8>) -> druid::Color {
    let idx = (y - 1) * header.width + (x - 1);
    let byte_idx = idx * header.bytes_per_pixel;

    let r = bytes[byte_idx + 0];
    let g = bytes[byte_idx + 1];
    let b = bytes[byte_idx + 2];
    let a = bytes[byte_idx + 3];

    druid::Color::rgba8(r, g, b, a)
}

/// Write RGBA to bytes. The underlying storage doesn't really matter; it can be a
/// PixelState, or a copy thereof, or something else, as long as it's bytes.
fn write(x: usize, y: usize, header: &PixelHeader, bytes: &mut Vec<u8>, color: &druid::Color) {
    let idx = (y - 1) * header.width + (x - 1);
    let byte_idx = idx * header.bytes_per_pixel;

    let (r, g, b, a) = color.as_rgba8();

    bytes[byte_idx + 0] = r;
    bytes[byte_idx + 1] = g;
    bytes[byte_idx + 2] = b;
    bytes[byte_idx + 3] = a;
}

/// Convert given color to black and white. This will desaturate the color first, and then
/// pick black or white depending on which side of the threshold they land.
fn black_and_white(color: &druid::Color, threshold: f64) -> druid::Color {
    let gray = desaturate(color);
    let (r, _, _, a) = gray.as_rgba();
    let bw = match r < threshold {
        true => 0.0,
        _ => 1.0,
    };
    druid::Color::rgba(bw, bw, bw, a)
}

/// Desaturate the given color (make it grayscale).
fn desaturate(color: &druid::Color) -> druid::Color {
    let (r, g, b, a) = color.as_rgba();
    let gray = r * 0.299 + g * 0.587 + b * 0.114;
    druid::Color::rgba(gray, gray, gray, a)
}
