use std::fmt;

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
