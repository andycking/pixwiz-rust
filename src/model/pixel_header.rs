/// Generic pixel header.
#[derive(Clone, druid::Data)]
pub struct PixelHeader {
    pub width: usize,
    pub height: usize,
    pub depth: u8,
    pub bytes_per_pixel: usize,
}

impl PixelHeader {
    const DEFAULT_WIDTH: usize = 32;
    const DEFAULT_HEIGHT: usize = 32;
    const DEFAULT_DEPTH: u8 = 8;
    const DEFAULT_BYTES_PER_PIXEL: usize = 4;

    pub fn new(width: usize, height: usize, depth: u8, bytes_per_pixel: usize) -> Self {
        assert!(width == Self::DEFAULT_WIDTH);
        assert!(height == Self::DEFAULT_HEIGHT);
        assert!(depth == Self::DEFAULT_DEPTH);
        assert!(bytes_per_pixel == Self::DEFAULT_BYTES_PER_PIXEL);

        Self {
            width: width,
            height: height,
            depth: depth,
            bytes_per_pixel: bytes_per_pixel,
        }
    }
}

impl Default for PixelHeader {
    fn default() -> Self {
        Self {
            width: Self::DEFAULT_WIDTH,
            height: Self::DEFAULT_HEIGHT,
            depth: Self::DEFAULT_DEPTH,
            bytes_per_pixel: Self::DEFAULT_BYTES_PER_PIXEL,
        }
    }
}
