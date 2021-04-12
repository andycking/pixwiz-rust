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

use super::button::Button;
use super::theme;
use crate::commands;
use crate::global;
use crate::model::app::AppState;
use crate::model::document::StateMachine;

/// Build an unsaved changes alert window. This will present the usual (on Mac OS) three options
/// of save, don't save, and cancel. The alert is modal; the doc window will block input until the
/// alert is dismissed.
pub fn unsaved(parent_pos: druid::Point) -> druid::WindowDesc<AppState> {
    let message = build_message("Do you want to save the changes you made?", true);
    let sub_message = build_message("Your changes will be lost if you don't save them.", false);

    let save = Button::new("Save", true).on_click(|ctx, data, _env| {
        // Transition to the next state. Just do this directly in the handler.
        data.doc.state_machine = StateMachine::UnsavedSave;

        // Close the alert, because we're going to show the save panel.
        ctx.submit_command(druid::commands::CLOSE_WINDOW);

        // Note that we send it to the app window here. The alert will already be gone by the time
        // the command is delivered to the delegate.
        ctx.submit_command(
            druid::commands::SHOW_SAVE_PANEL
                .with(global::file_dialog_opts())
                .to(data.id),
        );
    });

    let dont_save = Button::new("Don't Save", false).on_click(|ctx, data, _env| {
        data.doc.state_machine = Default::default();
        ctx.submit_command(druid::commands::CLOSE_WINDOW);
        ctx.submit_command(commands::OPEN_FILE_INTERNAL);
    });

    let cancel = Button::new("Cancel", false).on_click(|ctx, data, _env| {
        data.doc.state_machine = Default::default();
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
        .with_child(cancel.expand_width());

    build_alert(parent_pos, theme::UNSAVED_ALERT_SIZE, panel)
}

fn build_alert(
    parent_pos: druid::Point,
    size: druid::Size,
    panel: impl druid::Widget<AppState> + 'static,
) -> druid::WindowDesc<AppState> {
    let themed = panel
        .border(theme::MAIN_FILL, druid::theme::WIDGET_PADDING_VERTICAL)
        .background(theme::MAIN_FILL);

    let pos = center(parent_pos, theme::WINDOW_SIZE, size);

    druid::WindowDesc::new(themed)
        .set_level(druid::WindowLevel::Modal)
        .show_titlebar(false)
        .set_position(pos)
        .window_size(size)
        .resizable(false)
}

fn build_message(message: &'static str, bold: bool) -> druid::widget::Label<AppState> {
    let font = match bold {
        true => theme::ALERT_MESSAGE_FONT_BOLD,
        _ => theme::ALERT_MESSAGE_FONT,
    };

    druid::widget::Label::new(message)
        .with_text_color(druid::Color::BLACK)
        .with_line_break_mode(druid::widget::LineBreaking::WordWrap)
        .with_text_alignment(druid::TextAlignment::Center)
        .with_font(font)
}

fn center(parent_pos: druid::Point, parent_size: druid::Size, size: druid::Size) -> druid::Point {
    let center = druid::Point::new(
        parent_pos.x + parent_size.width / 2.0,
        parent_pos.y + parent_size.height / 2.0,
    );

    druid::Point::new(center.x - size.width / 2.0, center.y - size.width / 2.0)
}
