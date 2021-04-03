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
use crate::model::app_state::AppState;
use crate::view::button::Button;

const UNSAVED_ALERT_SIZE: druid::Size = druid::Size::new(208.0, 212.0);

pub fn unsaved(parent_window_pos: druid::Point) -> druid::WindowDesc<AppState> {
    let font = druid::FontDescriptor::new(druid::FontFamily::SYSTEM_UI)
        .with_weight(druid::FontWeight::BOLD);
    let message = druid::widget::Label::new("Do you want to save the changes you made?")
        .with_font(font)
        .with_text_color(druid::Color::BLACK)
        .with_line_break_mode(druid::widget::LineBreaking::WordWrap)
        .with_text_alignment(druid::TextAlignment::Center);

    let sub_font = druid::FontDescriptor::new(druid::FontFamily::SYSTEM_UI);
    let sub_message =
        druid::widget::Label::new("Your changes will be lost if you don't save them.")
            .with_font(sub_font)
            .with_text_color(druid::Color::BLACK)
            .with_line_break_mode(druid::widget::LineBreaking::WordWrap)
            .with_text_alignment(druid::TextAlignment::Center);

    let save = Button::new("Save", true);
    let dont_save = Button::new("Don't Save", false);
    let cancel = Button::new("Cancel", false);

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

    let center = druid::Point::new(
        parent_window_pos.x + WINDOW_SIZE.width / 2.0,
        parent_window_pos.y + WINDOW_SIZE.height / 2.0,
    );

    let alert_pos = druid::Point::new(
        center.x - UNSAVED_ALERT_SIZE.width / 2.0,
        center.y - UNSAVED_ALERT_SIZE.width / 2.0,
    );

    druid::WindowDesc::new(panel)
        .set_level(druid::WindowLevel::Modal)
        .show_titlebar(false)
        .set_position(alert_pos)
        .window_size(UNSAVED_ALERT_SIZE)
        .resizable(false)
}
