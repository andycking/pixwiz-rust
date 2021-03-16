use crate::model::state::PixelState;

pub struct ImageData {
    pub width: usize,
    pub height: usize,
    pub depth: u8,
    pub palette: Option<Vec<u8>>,
    pub bytes: Vec<u8>,
}

impl ImageData {
    pub fn new(
        width: usize,
        height: usize,
        depth: u8,
        palette: Option<Vec<u8>>,
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

impl From<&ImageData> for PixelState {
    fn from(image_data: &ImageData) -> Self {
        let size = image_data.width * image_data.height;
        let mut vec: Vec<u32> = vec![0; size];

        for i in 0..vec.len() {
            let j = i * 4;
            let mut chunk: [u8; 4] = [0; 4];

            chunk[0] = image_data.bytes[j + 0];
            chunk[1] = image_data.bytes[j + 1];
            chunk[2] = image_data.bytes[j + 2];
            chunk[3] = image_data.bytes[j + 3];

            vec[i] = u32::from_be_bytes(chunk);
        }

        Self::new(
            false,
            image_data.width,
            image_data.height,
            image_data.depth,
            vec,
        )
    }
}
