use std::collections::VecDeque;

use crate::model::types::PixelEnv;
use crate::model::types::PixelHeader;
use crate::transforms::util;

pub fn black_and_white(header: &PixelHeader, env: &PixelEnv, bytes: &mut Vec<u8>) {
    for y in env.bounds.y0 as usize..env.bounds.y1 as usize + 1 {
        for x in env.bounds.x0 as usize..env.bounds.x1 as usize + 1 {
            let color = util::read(x, y, header, bytes);
            let bw = util::black_and_white(color, 0.5);
            util::write(x, y, header, bytes, bw);
        }
    }
}

pub fn desaturate(header: &PixelHeader, env: &PixelEnv, bytes: &mut Vec<u8>) {
    for y in env.bounds.y0 as usize..env.bounds.y1 as usize + 1 {
        for x in env.bounds.x0 as usize..env.bounds.x1 as usize + 1 {
            let color = util::read(x, y, header, bytes);
            let gray = util::desaturate(color);
            util::write(x, y, header, bytes, gray);
        }
    }
}

pub fn fill(header: &PixelHeader, env: &PixelEnv, bytes: &mut Vec<u8>) {
    if !env.bounds.contains(env.pos) {
        return;
    }

    let x = env.pos.x as usize;
    let y = env.pos.y as usize;
    let start_color = util::read(x, y, header, bytes);
    if start_color == env.color {
        return;
    }

    let mut q: VecDeque<druid::Point> = VecDeque::new();
    q.push_back(env.pos);
    while !q.is_empty() {
        let node = q.pop_front().unwrap();
        let x = node.x as usize;
        let y = node.y as usize;
        if util::read(x, y, header, bytes) == start_color {
            util::write(x, y, header, bytes, env.color.clone());

            if node.x > env.bounds.x0 {
                q.push_back(druid::Point::new(node.x - 1.0, node.y));
            }
            if node.x < env.bounds.x1 {
                q.push_back(druid::Point::new(node.x + 1.0, node.y));
            }
            if node.y > env.bounds.y0 {
                q.push_back(druid::Point::new(node.x, node.y - 1.0));
            }
            if node.y < env.bounds.y1 {
                q.push_back(druid::Point::new(node.x, node.y + 1.0));
            }
        }
    }
}

pub fn _floyd_steinberg(_header: &PixelHeader, _bytes: &mut Vec<u8>, _bounds: druid::Rect) {}
