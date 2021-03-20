pub fn erase(
    _width: usize,
    height: usize,
    bytes_per_pixel: usize,
    bytes: &mut Vec<u8>,
    bounds: &druid::Rect,
) {
    let x0 = bounds.x0 as usize;
    let x1 = bounds.x1 as usize;
    let y0 = bounds.y0 as usize;
    let y1 = bounds.y1 as usize;

    for col in x0..x1 + 1 {
        for row in y0..y1 + 1 {
            let idx = (row - 1) * height + (col - 1);
            let byte_idx = idx * bytes_per_pixel;
            bytes[byte_idx + 0] = 0;
            bytes[byte_idx + 1] = 0;
            bytes[byte_idx + 2] = 0;
            bytes[byte_idx + 3] = 0;
        }
    }
}
