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

use druid::widget::Flex;
use druid::WidgetExt;

use super::theme;
use super::window::WINDOW_SIZE;
use crate::commands;
use crate::model::app_state::AppState;
use crate::view::button::Button;

const UNSAVED_ALERT_SIZE: druid::Size = druid::Size::new(208.0, 212.0);

const MESSAGE_FONT: druid::FontDescriptor =
    druid::FontDescriptor::new(druid::FontFamily::SYSTEM_UI);
const MESSAGE_FONT_BOLD: druid::FontDescriptor = MESSAGE_FONT.with_weight(druid::FontWeight::BOLD);

pub fn unsaved(parent_pos: druid::Point) -> druid::WindowDesc<AppState> {
    let message =
        build_message("Do you want to save the changes you made?").with_font(MESSAGE_FONT_BOLD);
    let sub_message =
        build_message("Your changes will be lost if you don't save them.").with_font(MESSAGE_FONT);

    let save = Button::new("Save", true);

    let dont_save = Button::new("Don't Save", false).on_click(|ctx, data, _env| {
        data.alert = false;
        ctx.submit_command(druid::commands::CLOSE_WINDOW);
        ctx.submit_command(commands::INTERNAL_CLEAR_DOCUMENT);
    });

    let cancel = Button::new("Cancel", false).on_click(|ctx, data, _env| {
        data.alert = false;
        ctx.submit_command(druid::commands::CLOSE_WINDOW);
    });

    let panel = Flex::column()
        .with_child(message)
        .with_default_spacer()
        .with_child(sub_message)
        .with_default_spacer()
        .with_default_spacer()
        .with_child(save.expand_width())
        .with_default_spacer()
        .with_child(dont_save.expand_width())
        .with_default_spacer()
        .with_default_spacer()
        .with_child(cancel.expand_width())
        .border(theme::MAIN_FILL, druid::theme::WIDGET_PADDING_VERTICAL)
        .background(theme::MAIN_FILL);

    let pos = center(parent_pos, WINDOW_SIZE, UNSAVED_ALERT_SIZE);

    druid::WindowDesc::new(panel)
        .set_level(druid::WindowLevel::Modal)
        .show_titlebar(false)
        .set_position(pos)
        .window_size(UNSAVED_ALERT_SIZE)
        .resizable(false)
}

fn build_message(message: &'static str) -> druid::widget::Label<AppState> {
    druid::widget::Label::new(message)
        .with_text_color(druid::Color::BLACK)
        .with_line_break_mode(druid::widget::LineBreaking::WordWrap)
        .with_text_alignment(druid::TextAlignment::Center)
}

fn center(parent_pos: druid::Point, parent_size: druid::Size, size: druid::Size) -> druid::Point {
    let center = druid::Point::new(
        parent_pos.x + parent_size.width / 2.0,
        parent_pos.y + parent_size.height / 2.0,
    );

    druid::Point::new(center.x - size.width / 2.0, center.y - size.width / 2.0)
}
