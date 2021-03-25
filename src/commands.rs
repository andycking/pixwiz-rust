use druid::Selector;

/// Info for image command selectors.
pub struct ImageInfo {
    pub p: druid::Point,
}

impl ImageInfo {
    pub fn new(p: druid::Point) -> Self {
        Self { p: p }
    }
}

pub const IMAGE_BLACK_AND_WHITE: Selector = Selector::new("image-black-and-white");
pub const IMAGE_CLEAR: Selector = Selector::new("image-clear");
pub const IMAGE_DESATURATE: Selector = Selector::new("image-desaturate");
pub const IMAGE_DITHER_FLOYD: Selector = Selector::new("image-dither-floyd");
pub const IMAGE_ERASER: Selector<ImageInfo> = Selector::new("image-eraser");
pub const IMAGE_FILL: Selector = Selector::new("image-fill");
pub const IMAGE_PAINT: Selector<ImageInfo> = Selector::new("image-paint");

pub const VIEW_SHOW_GRID: Selector = Selector::new("view-show-grid");
