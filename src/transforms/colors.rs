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

use crate::model::pixel_env::PixelEnv;
use crate::model::pixel_header::PixelHeader;

pub fn black_and_white(header: &PixelHeader, env: &PixelEnv, bytes: &mut Vec<u8>) {
    for y in env.bounds.y0 as usize..env.bounds.y1 as usize {
        for x in env.bounds.x0 as usize..env.bounds.x1 as usize {
            let color = super::read(x, y, header, bytes);
            let bw = super::black_and_white(&color, 0.5);
            super::write(x, y, header, bytes, &bw);
        }
    }
}

pub fn desaturate(header: &PixelHeader, env: &PixelEnv, bytes: &mut Vec<u8>) {
    for y in env.bounds.y0 as usize..env.bounds.y1 as usize {
        for x in env.bounds.x0 as usize..env.bounds.x1 as usize {
            let color = super::read(x, y, header, bytes);
            let gray = super::desaturate(&color);
            super::write(x, y, header, bytes, &gray);
        }
    }
}

pub fn fill(header: &PixelHeader, env: &PixelEnv, bytes: &mut Vec<u8>) {
    let x = env.pos.x as usize;
    let y = env.pos.y as usize;
    let start_color = super::read(x, y, header, bytes);
    if start_color == env.color {
        return;
    }

    let mut q: VecDeque<druid::Point> = VecDeque::new();
    q.push_back(env.pos);
    while !q.is_empty() {
        let node = q.pop_front().unwrap();
        let x = node.x as usize;
        let y = node.y as usize;
        if super::read(x, y, header, bytes) == start_color {
            super::write(x, y, header, bytes, &env.color);

            let left = node - (1.0, 0.0);
            if env.bounds.contains(left) {
                q.push_back(left);
            }
            let right = node + (1.0, 0.0);
            if env.bounds.contains(right) {
                q.push_back(right);
            }
            let up = node - (0.0, 1.0);
            if env.bounds.contains(up) {
                q.push_back(up);
            }
            let down = node + (0.0, 1.0);
            if env.bounds.contains(down) {
                q.push_back(down);
            }
        }
    }
}

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
        r = r + (quant_error.0 * weight / 16.0);
        g = g + (quant_error.1 * weight / 16.0);
        b = b + (quant_error.2 * weight / 16.0);
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
        if env.bounds.contains(p) {
            let oldpixel = super::read(x, y, header, bytes);
            let newpixel = apply_error(&oldpixel, quant_error, weight);
            super::write(x, y, header, bytes, &newpixel);
        }
    }

    for y in env.bounds.y0 as usize..env.bounds.y1 as usize {
        for x in env.bounds.x0 as usize..env.bounds.x1 as usize {
            let oldpixel = super::read(x, y, header, bytes);
            let newpixel = super::black_and_white(&oldpixel, 0.5);
            super::write(x, y, header, bytes, &newpixel);

            let quant_error = calculate_error(&oldpixel, &newpixel);

            mod_pixel(x + 1, y + 0, quant_error, 7.0, header, env, bytes);
            mod_pixel(x - 1, y + 1, quant_error, 3.0, header, env, bytes);
            mod_pixel(x + 0, y + 1, quant_error, 5.0, header, env, bytes);
            mod_pixel(x + 1, y + 1, quant_error, 1.0, header, env, bytes);
        }
    }
}
