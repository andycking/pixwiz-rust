use druid::Color;

pub const MAIN_FILL: Color = Color::rgb8(240, 240, 240);
pub const MAIN_STROKE: Color = Color::rgb8(220, 220, 220);

pub const TOOLS_STROKE: Color = MAIN_STROKE;
pub const TOOLS_STROKE_SELECTED: Color = Color::BLACK;

pub const STATUS_BAR_FILL: Color = MAIN_FILL;
pub const STATUS_BAR_STROKE: Color = Color::BLACK;

pub const COLOR_WELL_STROKE: Color = MAIN_STROKE;

pub const PREVIEW_FILL: u32 = CANVAS_FILL_LIGHT;
pub const PREVIEW_STROKE: Color = MAIN_STROKE;

pub const PALETTE_FILL: Color = Color::BLACK;
pub const PALETTE_STROKE_SELECTED: Color = Color::BLACK;

pub const CANVAS_FILL_DARK: u32 = 0x505050ff;
pub const CANVAS_FILL_LIGHT: u32 = 0x606060ff;
pub const CANVAS_STROKE: Color = MAIN_STROKE;
pub const CANVAS_STROKE_SELECTED_DARK: Color = Color::BLACK;
pub const CANVAS_STROKE_SELECTED_LIGHT: Color = Color::WHITE;
