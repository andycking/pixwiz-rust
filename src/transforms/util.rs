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

use crate::model::pixel_header::PixelHeader;

/// Read RGBA from bytes. The underlying storage doesn't really matter: it can be a
/// PixelState, or a copy thereof, or something else, as long as it's bytes.
pub fn read(x: usize, y: usize, header: &PixelHeader, bytes: &[u8]) -> druid::Color {
    let idx = (y - 1) * header.width + (x - 1);
    let byte_idx = idx * header.bytes_per_pixel;

    let red = bytes[byte_idx];
    let green = bytes[byte_idx + 1];
    let blue = bytes[byte_idx + 2];
    let alpha = bytes[byte_idx + 3];

    druid::Color::rgba8(red, green, blue, alpha)
}

/// Write RGBA to bytes. The underlying storage doesn't really matter; it can be a
/// PixelState, or a copy thereof, or something else, as long as it's bytes.
pub fn write(x: usize, y: usize, header: &PixelHeader, bytes: &mut Vec<u8>, color: &druid::Color) {
    let idx = (y - 1) * header.width + (x - 1);
    let byte_idx = idx * header.bytes_per_pixel;

    let (red, green, blue, alpha) = color.as_rgba8();

    bytes[byte_idx] = red;
    bytes[byte_idx + 1] = green;
    bytes[byte_idx + 2] = blue;
    bytes[byte_idx + 3] = alpha;
}

/// Convert given color to black and white. This will desaturate the color first, and then
/// pick black or white depending on which side of the threshold they land.
pub fn black_and_white(color: &druid::Color, threshold: f64) -> druid::Color {
    let gray = desaturate(color);
    let (red, _, _, alpha) = gray.as_rgba();
    let bw = match red < threshold {
        true => 0.0,
        _ => 1.0,
    };
    druid::Color::rgba(bw, bw, bw, alpha)
}

/// Desaturate the given color (make it grayscale).
pub fn desaturate(color: &druid::Color) -> druid::Color {
    let (r, g, b, a) = color.as_rgba();
    let gray = r * 0.299 + g * 0.587 + b * 0.114;
    druid::Color::rgba(gray, gray, gray, a)
}

/// Convert RGBA bytes to HSLA.
pub fn rgba8_to_hsla(red: u8, green: u8, blue: u8, alpha: u8) -> (f64, f64, f64, f64) {
    rgba_to_hsla(
        f64_round(red as f64 / 255.0),
        f64_round(green as f64 / 255.0),
        f64_round(blue as f64 / 255.0),
        f64_round(alpha as f64 / 255.0),
    )
}

/// Convert RGBA float values to HSLA.
pub fn rgba_to_hsla(red: f64, green: f64, blue: f64, alpha: f64) -> (f64, f64, f64, f64) {
    let maxv = f64_max3(red, green, blue);
    let minv = f64_min3(red, green, blue);

    let mut hue = 0.0;
    let mut saturation = 0.0;
    let mut luminance = (maxv + minv) / 2.0;

    if !f64_eq(maxv, minv) {
        let d = maxv - minv;

        saturation = match luminance > 0.5 {
            true => d / (2.0 - maxv - minv),
            _ => d / (maxv + minv),
        };

        if f64_eq(maxv, red) {
            let weight = match green < blue {
                true => 6.0,
                _ => 0.0,
            };
            hue = (green - blue) / d + weight;
        } else if f64_eq(maxv, green) {
            hue = (blue - red) / d + 2.0;
        } else if f64_eq(maxv, blue) {
            hue = (red - green) / d + 4.0;
        }

        hue /= 6.0;
    }

    hue = f64_round(hue);
    saturation = f64_round(saturation);
    luminance = f64_round(luminance);

    (hue, saturation, luminance, alpha)
}

/// Convert HSLA to RGBA float values.
pub fn hsla_to_rgba(hue: f64, saturation: f64, luminance: f64, alpha: f64) -> (f64, f64, f64, f64) {
    fn hue_to_rgb(p: f64, q: f64, t: f64) -> f64 {
        let mut t2 = t;

        if t2 < 0.0 {
            t2 += 1.0;
        }
        if t2 > 1.0 {
            t2 -= 1.0;
        }

        if t2 < 1.0 / 6.0 {
            return p + (q - p) * 6.0 * t2;
        }
        if t2 < 0.5 {
            return q;
        }
        if t2 < 2.0 / 3.0 {
            return p + (q - p) * (2.0 / 3.0 - t2) * 6.0;
        }

        p
    }

    if f64_eq(saturation, 0.0) {
        return (luminance, luminance, luminance, alpha);
    }

    let q = match luminance < 0.5 {
        true => luminance * (1.0 + saturation),
        _ => (luminance + saturation) - (luminance * saturation),
    };
    let p = 2.0 * luminance - q;

    let red = f64_floor(hue_to_rgb(p, q, hue + 1.0 / 3.0));
    let green = f64_floor(hue_to_rgb(p, q, hue));
    let blue = f64_floor(hue_to_rgb(p, q, hue - 1.0 / 3.0));

    (red, green, blue, alpha)
}

fn f64_max3(a: f64, b: f64, c: f64) -> f64 {
    f64::max(f64::max(a, b), c)
}

fn f64_min3(a: f64, b: f64, c: f64) -> f64 {
    f64::min(f64::min(a, b), c)
}

fn f64_eq(a: f64, b: f64) -> bool {
    (a - b).abs() < f64::EPSILON
}

fn f64_round(val: f64) -> f64 {
    (val * 100.0).round() / 100.0
}

fn f64_floor(val: f64) -> f64 {
    (val * 100.0).floor() / 100.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_converts_rgba8_to_hsla() {
        let expected = (0.58, 0.47, 0.59, 1.0);
        let got = rgba8_to_hsla(100, 150, 200, 255);
        assert_eq!(expected, got);
    }

    #[test]
    fn it_converts_rgba8_to_hsla_gray() {
        let expected = (0.0, 0.0, 0.39, 1.0);
        let got = rgba8_to_hsla(100, 100, 100, 255);
        assert_eq!(expected, got);
    }

    #[test]
    fn it_converts_rgba_to_hsla() {
        let expected = (0.58, 0.47, 0.59, 1.0);
        let got = rgba_to_hsla(0.39, 0.59, 0.78, 1.0);
        assert_eq!(expected, got);
    }

    #[test]
    fn it_converts_rgba_to_hsla_gray() {
        let expected = (0.0, 0.0, 0.39, 1.0);
        let got = rgba_to_hsla(0.39, 0.39, 0.39, 1.0);
        assert_eq!(expected, got);
    }

    #[test]
    fn it_converts_hsla_to_rgba() {
        let expected = (0.39, 0.59, 0.78, 1.0);
        let got = hsla_to_rgba(0.58, 0.47, 0.59, 1.0);
        assert_eq!(expected, got);
    }

    #[test]
    fn it_converts_hsla_to_rgba_gray() {
        let expected = (0.59, 0.59, 0.59, 1.0);
        let got = hsla_to_rgba(0.58, 0.0, 0.59, 1.0);
        assert_eq!(expected, got);
    }
}
