use std::fmt;

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

/// Generic RGBA color type. Yes, I know we already use druid::Color, but that doesn't expose
/// the components directly. Blah.
pub struct Rgba {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Rgba {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self {
            r: r,
            g: g,
            b: b,
            a: a,
        }
    }
}

impl From<(u8, u8, u8, u8)> for Rgba {
    fn from(v: (u8, u8, u8, u8)) -> Self {
        Rgba::new(v.0, v.1, v.2, v.3)
    }
}

/// Supported tool types.
#[derive(Clone, Copy, druid::Data, Debug, PartialEq)]
pub enum ToolType {
    Cropper,
    Dropper,
    Eraser,
    Fill,
    Lasso,
    Marquee,
    Move,
    Paint,
}

impl fmt::Display for ToolType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
