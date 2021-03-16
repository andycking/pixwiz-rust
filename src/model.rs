use std::ops::{Index, IndexMut};
use std::sync::Arc;

pub struct PaletteData {
    pub bytes: Vec<u8>,
}

impl PaletteData {
    pub fn _new(bytes: Vec<u8>) -> Self {
        Self { bytes: bytes }
    }
}

pub struct ImageData {
    pub width: usize,
    pub height: usize,
    pub depth: u8,
    pub palette: Option<PaletteData>,
    pub bytes: Vec<u8>,
}

impl ImageData {
    pub fn new(
        width: usize,
        height: usize,
        depth: u8,
        palette: Option<PaletteData>,
        bytes: Vec<u8>,
    ) -> Self {
        assert!(width == 32);
        assert!(height == 32);
        assert!(depth == 8);

        Self {
            width: width,
            height: height,
            depth: depth,
            palette: palette,
            bytes: bytes,
        }
    }
}

impl From<&PixelState> for ImageData {
    fn from(pixels: &PixelState) -> Self {
        let size = pixels.width * pixels.height * std::mem::size_of::<u32>();
        let mut bytes: Vec<u8> = vec![0; size];

        for i in 0..pixels.len() {
            let j = i * 4;
            let chunk = pixels[i].to_be_bytes();
            bytes[j + 0] = chunk[0];
            bytes[j + 1] = chunk[1];
            bytes[j + 2] = chunk[2];
            bytes[j + 3] = chunk[3];
        }

        Self {
            width: pixels.width,
            height: pixels.height,
            depth: pixels.depth,
            palette: None,
            bytes: bytes,
        }
    }
}

/// Pixel storage. Each value is stored as a u32 representation of RGBA, with the alpha value
/// in the least significant position. This matches what Color does internally. We hold the
/// values in an ARC, to avoid copying them.
#[derive(Clone, druid::Data)]
pub struct PixelState {
    pub dirty: bool,
    pub width: usize,
    pub height: usize,
    pub depth: u8,
    storage: Arc<Vec<u32>>,
}

impl PixelState {
    const DEFAULT_WIDTH: usize = 32;
    const DEFAULT_HEIGHT: usize = 32;
    const DEFAULT_DEPTH: u8 = 8;

    pub fn new() -> Self {
        Self {
            dirty: false,
            width: Self::DEFAULT_WIDTH,
            height: Self::DEFAULT_HEIGHT,
            depth: Self::DEFAULT_DEPTH,
            storage: Arc::new(vec![0; Self::DEFAULT_WIDTH * Self::DEFAULT_HEIGHT]),
        }
    }

    pub fn len(&self) -> usize {
        // We always want len and capacity to be the same. The entire vector must have been
        // initialized so that later on we don't access an invalid pixel.
        assert!(self.storage.len() == self.storage.capacity());
        self.storage.len()
    }

    pub fn write(&mut self, idx: usize, value: u32) {
        *Arc::make_mut(&mut self.storage).index_mut(idx) = value;
        self.dirty = true;
    }
}

/// Implement the index trait. This is a convenient way for callers to access the pixel
/// storage directly. Note that it's immutable.
impl Index<usize> for PixelState {
    type Output = u32;
    fn index(&self, idx: usize) -> &Self::Output {
        &self.storage[idx]
    }
}

impl From<&ImageData> for PixelState {
    fn from(image_data: &ImageData) -> Self {
        let size = image_data.width * image_data.height;
        let mut storage: Vec<u32> = vec![0; size];

        for i in 0..storage.len() {
            let j = i * 4;
            let mut chunk: [u8; 4] = [0; 4];

            chunk[0] = image_data.bytes[j + 0];
            chunk[1] = image_data.bytes[j + 1];
            chunk[2] = image_data.bytes[j + 2];
            chunk[3] = image_data.bytes[j + 3];

            storage[i] = u32::from_be_bytes(chunk);
        }

        Self {
            dirty: false,
            width: image_data.width,
            height: image_data.height,
            depth: image_data.depth,
            storage: Arc::new(storage),
        }
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

/// Application state.
#[derive(Clone, druid::Data)]
pub struct AppState {
    pub brush_color: u32,
    pub pos_color: u32,
    pub start_pos: druid::Point,
    pub current_pos: druid::Point,
    pub selection: Option<druid::Rect>,
    pub tool_type: ToolType,
    pub pixels: PixelState,
    pub path: Option<String>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            brush_color: 0x0ff,
            pos_color: 0x0ff,
            start_pos: druid::Point::ZERO,
            current_pos: druid::Point::ZERO,
            selection: None,
            tool_type: ToolType::Paint,
            pixels: PixelState::new(),
            path: None,
        }
    }
}
