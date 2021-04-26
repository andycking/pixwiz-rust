// Copyright 2021 Andy King
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use druid::Color;

pub const MAIN_FILL: Color = Color::rgb8(240, 240, 240);
pub const MAIN_STROKE: Color = Color::rgb8(208, 208, 208);

pub const TOOLS_STROKE: Color = MAIN_STROKE;
pub const TOOLS_STROKE_SELECTED: Color = Color::BLACK;
pub const TOOLS_PADDING: f64 = 8.0;

pub const STATUS_BAR_FILL: Color = MAIN_FILL;
pub const STATUS_BAR_STROKE: Color = Color::BLACK;

pub const COLOR_WELL_STROKE: Color = MAIN_STROKE;
pub const COLOR_WELL_SIZE: druid::Size = druid::Size::new(88.0, 30.0);

pub const PREVIEW_FILL: Color = CANVAS_FILL_LIGHT;
pub const PREVIEW_STROKE: Color = MAIN_STROKE;
pub const PREVIEW_SIZE: druid::Size = druid::Size::new(CANVAS_ROWS as f64, CANVAS_COLS as f64);

pub const PALETTE_FILL: Color = Color::BLACK;
pub const PALETTE_STROKE_SELECTED: Color = Color::BLACK;
pub const PALETTE_COLS: usize = 8;
pub const PALETTE_ROWS: usize = 32;
pub const PALETTE_PIXEL_SIZE: f64 = 15.0;

pub const CANVAS_FILL_DARK: Color = Color::rgb8(80, 80, 80);
pub const CANVAS_FILL_LIGHT: Color = Color::rgb8(96, 96, 96);
pub const CANVAS_STROKE: Color = MAIN_STROKE;
pub const CANVAS_STROKE_SELECTED_DARK: Color = Color::BLACK;
pub const CANVAS_STROKE_SELECTED_LIGHT: Color = Color::WHITE;
pub const CANVAS_STROKE_GRID_DARK: Color = Color::BLACK;
pub const CANVAS_STROKE_GRID_LIGHT: Color = MAIN_STROKE;
pub const CANVAS_ROWS: usize = 48;
pub const CANVAS_COLS: usize = 48;
pub const CANVAS_PIXEL_SIZE: f64 = 16.0;

pub const BUTTON_DEFAULT_DARK: Color = Color::rgb8(0, 92, 252);
pub const BUTTON_DEFAULT_LIGHT: Color = Color::rgb8(0, 124, 252);
pub const BUTTON_DARK: Color = Color::rgb8(180, 180, 180);
pub const BUTTON_LIGHT: Color = Color::rgb8(200, 200, 200);

pub const WINDOW_SIZE: druid::Size = druid::Size::new(672.0, 696.0);
pub const UNSAVED_FILE_ALERT_SIZE: druid::Size = druid::Size::new(208.0, 212.0);
pub const WARNING_ALERT_SIZE: druid::Size = druid::Size::new(208.0, 108.0);

pub const ALERT_MESSAGE_FONT: druid::FontDescriptor =
    druid::FontDescriptor::new(druid::FontFamily::SYSTEM_UI);
pub const ALERT_MESSAGE_FONT_BOLD: druid::FontDescriptor =
    ALERT_MESSAGE_FONT.with_weight(druid::FontWeight::BOLD);
