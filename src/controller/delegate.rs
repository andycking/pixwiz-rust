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

use crate::commands;
use crate::controller;
use crate::model::app::AppState;
use crate::view::menu;

pub struct Delegate;

impl druid::AppDelegate<AppState> for Delegate {
    fn command(
        &mut self,
        ctx: &mut druid::DelegateCtx,
        _target: druid::Target,
        cmd: &druid::Command,
        data: &mut AppState,
        _env: &druid::Env,
    ) -> druid::Handled {
        let handled = match cmd {
            // File.
            _ if cmd.is(druid::commands::NEW_FILE) => {
                controller::file::new(ctx, cmd, data);
                druid::Handled::Yes
            }
            _ if cmd.is(druid::commands::OPEN_FILE) => {
                controller::file::open(ctx, cmd, data);
                druid::Handled::Yes
            }
            _ if cmd.is(commands::OPEN_FILE_INTERNAL) => {
                controller::file::open_internal(ctx, cmd, data);
                druid::Handled::Yes
            }
            _ if cmd.is(druid::commands::SAVE_FILE) => {
                controller::file::save(ctx, cmd, data);
                druid::Handled::Yes
            }
            _ if cmd.is(druid::commands::SAVE_FILE_AS) => {
                controller::file::save_as(ctx, cmd, data);
                druid::Handled::Yes
            }
            _ if cmd.is(druid::commands::SAVE_PANEL_CANCELLED) => {
                controller::file::save_cancelled(ctx, cmd, data);
                druid::Handled::Yes
            }

            // Edit.
            _ if cmd.is(druid::commands::UNDO) => {
                controller::edit::undo(ctx, cmd, data);
                druid::Handled::Yes
            }
            _ if cmd.is(druid::commands::REDO) => {
                controller::edit::redo(ctx, cmd, data);
                druid::Handled::Yes
            }
            _ if cmd.is(druid::commands::CUT) => {
                controller::edit::cut(ctx, cmd, data);
                druid::Handled::Yes
            }
            _ if cmd.is(druid::commands::COPY) => {
                controller::edit::copy(ctx, cmd, data);
                druid::Handled::Yes
            }
            _ if cmd.is(druid::commands::PASTE) => {
                controller::edit::paste(ctx, cmd, data);
                druid::Handled::Yes
            }
            _ if cmd.is(commands::EDIT_SELECT_ALL) => {
                controller::edit::select_all(ctx, cmd, data);
                druid::Handled::Yes
            }
            _ if cmd.is(commands::EDIT_DESELECT) => {
                controller::edit::deselect(ctx, cmd, data);
                druid::Handled::Yes
            }

            // Image.
            _ if cmd.is(commands::IMAGE_BLACK_AND_WHITE) => {
                controller::image::black_and_white(ctx, cmd, data);
                druid::Handled::Yes
            }
            _ if cmd.is(commands::IMAGE_BRIGHTEN) => {
                controller::image::brighten(ctx, cmd, data);
                druid::Handled::Yes
            }
            _ if cmd.is(commands::IMAGE_CLEAR) => {
                controller::image::clear(ctx, cmd, data);
                druid::Handled::Yes
            }
            _ if cmd.is(commands::IMAGE_DARKEN) => {
                controller::image::darken(ctx, cmd, data);
                druid::Handled::Yes
            }
            _ if cmd.is(commands::IMAGE_DESATURATE) => {
                controller::image::desaturate(ctx, cmd, data);
                druid::Handled::Yes
            }
            _ if cmd.is(commands::IMAGE_DITHER_FLOYD) => {
                controller::image::dither_floyd(ctx, cmd, data);
                druid::Handled::Yes
            }
            _ if cmd.is(commands::IMAGE_ERASER) => {
                controller::image::eraser(ctx, cmd, data);
                druid::Handled::Yes
            }
            _ if cmd.is(commands::IMAGE_FILL) => {
                controller::image::fill(ctx, cmd, data);
                druid::Handled::Yes
            }
            _ if cmd.is(commands::IMAGE_MARQUEE) => {
                controller::image::marquee(ctx, cmd, data);
                druid::Handled::Yes
            }
            _ if cmd.is(commands::IMAGE_MOVE) => {
                controller::image::move_(ctx, cmd, data);
                druid::Handled::Yes
            }
            _ if cmd.is(commands::IMAGE_PAINT) => {
                controller::image::paint(ctx, cmd, data);
                druid::Handled::Yes
            }

            // View.
            _ if cmd.is(commands::VIEW_SHOW_GRID) => {
                controller::view::show_grid(ctx, cmd, data);
                druid::Handled::Yes
            }

            _ => druid::Handled::No,
        };

        if handled.is_handled() {
            rebuild_menu_bar(ctx, cmd, data);
        }

        handled
    }
}

/// Rebuild the menu bar to reflect the current state of the application.
/// Druid menus are immutable, so we have to rebuild the entire thing from scratch.
fn rebuild_menu_bar(ctx: &mut druid::DelegateCtx, cmd: &druid::Command, data: &mut AppState) {
    let mut menu_opts: menu::MenuOpts = Default::default();

    menu_opts.disable(
        menu::COMMON_MENU_FILE_SAVE,
        data.doc.path().is_none() || !data.doc.pixels().dirty(),
    );

    let empty_selection = data.doc.selection().is_none();

    menu_opts.disable(menu::COMMON_MENU_UNDO, data.doc.undo().is_empty());
    menu_opts.disable(menu::COMMON_MENU_REDO, data.doc.redo().is_empty());
    menu_opts.disable(menu::COMMON_MENU_CUT, empty_selection);
    menu_opts.disable(menu::COMMON_MENU_COPY, empty_selection);
    menu_opts.disable(menu::EDIT_MENU_DESELECT, empty_selection);

    menu_opts.select(menu::MENU_VIEW_SHOW_GRID, data.show_grid());

    if let druid::Target::Window(id) = cmd.target() {
        let menu_bar: druid::MenuDesc<AppState> = menu::menu_bar(&menu_opts);
        ctx.set_menu(menu_bar, id);
    }
}
