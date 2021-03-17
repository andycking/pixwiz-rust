use druid::Color;

pub const MAIN_FILL: Color = Color::rgb8(240, 240, 240);
pub const MAIN_STROKE: Color = Color::rgb8(208, 208, 208);

pub const TOOLS_STROKE: Color = MAIN_STROKE;
pub const TOOLS_STROKE_SELECTED: Color = Color::BLACK;

pub const STATUS_BAR_FILL: Color = MAIN_FILL;
pub const STATUS_BAR_STROKE: Color = Color::BLACK;

pub const COLOR_WELL_STROKE: Color = MAIN_STROKE;

pub const PREVIEW_FILL: Color = CANVAS_FILL_LIGHT;
pub const PREVIEW_STROKE: Color = MAIN_STROKE;

pub const PALETTE_FILL: Color = Color::BLACK;
pub const PALETTE_STROKE_SELECTED: Color = Color::BLACK;

pub const CANVAS_FILL_DARK: Color = Color::rgb8(80, 80, 80);
pub const CANVAS_FILL_LIGHT: Color = Color::rgb8(96, 96, 96);
pub const CANVAS_STROKE: Color = MAIN_STROKE;
pub const CANVAS_STROKE_SELECTED_DARK: Color = Color::BLACK;
pub const CANVAS_STROKE_SELECTED_LIGHT: Color = Color::WHITE;
pub const CANVAS_STROKE_GRID_DARK: Color = Color::BLACK;
pub const CANVAS_STROKE_GRID_LIGHT: Color = MAIN_STROKE;
