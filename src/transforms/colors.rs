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

use std::collections::VecDeque;

use super::util;
use crate::model::pixels::PixelEnv;
use crate::model::pixels::PixelHeader;

/// Convert pixels to black & white.
pub fn black_and_white(header: &PixelHeader, env: &PixelEnv, bytes: &mut Vec<u8>) {
    let bounds = env.bounds();
    for y in bounds.y0 as usize..bounds.y1 as usize {
        for x in bounds.x0 as usize..bounds.x1 as usize {
            let color = util::read(x, y, header, bytes);
            let bw = util::black_and_white(&color, env.param());
            util::write(x, y, header, bytes, &bw);
        }
    }
}

pub fn brightness(header: &PixelHeader, env: &PixelEnv, bytes: &mut Vec<u8>) {
    let bounds = env.bounds();
    for y in bounds.y0 as usize..bounds.y1 as usize {
        for x in bounds.x0 as usize..bounds.x1 as usize {
            let color = util::read(x, y, header, bytes);
            let new_color = util::brightness(&color, env.param());
            util::write(x, y, header, bytes, &new_color);
        }
    }
}

/// Desaturate pixels (make them grayscale).
pub fn desaturate(header: &PixelHeader, env: &PixelEnv, bytes: &mut Vec<u8>) {
    let bounds = env.bounds();
    for y in bounds.y0 as usize..bounds.y1 as usize {
        for x in bounds.x0 as usize..bounds.x1 as usize {
            let color = util::read(x, y, header, bytes);
            let gray = util::desaturate(&color);
            util::write(x, y, header, bytes, &gray);
        }
    }
}

/// Fill the given pixels to the boundary.
pub fn fill(header: &PixelHeader, env: &PixelEnv, bytes: &mut Vec<u8>) {
    let bounds = env.bounds();
    for y in bounds.y0 as usize..bounds.y1 as usize {
        for x in bounds.x0 as usize..bounds.x1 as usize {
            util::write(x, y, header, bytes, env.color());
        }
    }
}

/// Flood fill the given pixels starting from a seed position.
pub fn flood_fill(header: &PixelHeader, env: &PixelEnv, bytes: &mut Vec<u8>) {
    let x = env.pos().x as usize;
    let y = env.pos().y as usize;
    let start_color = util::read(x, y, header, bytes);
    if start_color == *env.color() {
        return;
    }

    let mut q: VecDeque<druid::Point> = VecDeque::new();
    q.push_back(*env.pos());
    while !q.is_empty() {
        let node = q.pop_front().unwrap();
        let x = node.x as usize;
        let y = node.y as usize;
        if util::read(x, y, header, bytes) == start_color {
            util::write(x, y, header, bytes, env.color());

            let bounds = env.bounds();

            let left = node - (1.0, 0.0);
            if bounds.contains(left) {
                q.push_back(left);
            }
            let right = node + (1.0, 0.0);
            if bounds.contains(right) {
                q.push_back(right);
            }
            let up = node - (0.0, 1.0);
            if bounds.contains(up) {
                q.push_back(up);
            }
            let down = node + (0.0, 1.0);
            if bounds.contains(down) {
                q.push_back(down);
            }
        }
    }
}

/// Dither pixels using Floyd–Steinberg.
pub fn dither_floyd(header: &PixelHeader, env: &PixelEnv, bytes: &mut Vec<u8>) {
    fn calculate_error(oldpixel: &druid::Color, newpixel: &druid::Color) -> (f64, f64, f64) {
        let (old_r, old_g, old_b, _) = oldpixel.as_rgba();
        let (new_r, new_g, new_b, _) = newpixel.as_rgba();

        (old_r - new_r, old_g - new_g, old_b - new_b)
    }

    fn apply_error(
        color: &druid::Color,
        quant_error: (f64, f64, f64),
        weight: f64,
    ) -> druid::Color {
        let (mut r, mut g, mut b, a) = color.as_rgba();
        r += quant_error.0 * weight / 16.0;
        g += quant_error.1 * weight / 16.0;
        b += quant_error.2 * weight / 16.0;
        druid::Color::rgba(r, g, b, a)
    }

    fn mod_pixel(
        x: usize,
        y: usize,
        quant_error: (f64, f64, f64),
        weight: f64,
        header: &PixelHeader,
        env: &PixelEnv,
        bytes: &mut Vec<u8>,
    ) {
        let p = druid::Point::new(x as f64, y as f64);
        if env.bounds().contains(p) {
            let oldpixel = util::read(x, y, header, bytes);
            let newpixel = apply_error(&oldpixel, quant_error, weight);
            util::write(x, y, header, bytes, &newpixel);
        }
    }

    let bounds = env.bounds();
    for y in bounds.y0 as usize..bounds.y1 as usize {
        for x in bounds.x0 as usize..bounds.x1 as usize {
            let oldpixel = util::read(x, y, header, bytes);
            let newpixel = util::black_and_white(&oldpixel, 0.5);
            util::write(x, y, header, bytes, &newpixel);

            let quant_error = calculate_error(&oldpixel, &newpixel);

            mod_pixel(x + 1, y, quant_error, 7.0, header, env, bytes);
            mod_pixel(x - 1, y + 1, quant_error, 3.0, header, env, bytes);
            mod_pixel(x, y + 1, quant_error, 5.0, header, env, bytes);
            mod_pixel(x + 1, y + 1, quant_error, 1.0, header, env, bytes);
        }
    }
}
