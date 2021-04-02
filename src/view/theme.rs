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
