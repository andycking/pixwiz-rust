/// Supported tool types.
#[derive(Clone, Copy, druid::Data, Debug, PartialEq)]
pub enum ToolType {
    Dropper,
    Eraser,
    Fill,
    Marquee,
    Move,
    Paint,
}
