use crate::commands;
use crate::controller;
use crate::model::app_state::AppState;
use crate::view;

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
                controller::file::new_file(ctx, cmd, data);
                druid::Handled::Yes
            }
            _ if cmd.is(druid::commands::OPEN_FILE) => {
                controller::file::open_file(ctx, cmd, data);
                druid::Handled::Yes
            }
            _ if cmd.is(druid::commands::SAVE_FILE) => {
                controller::file::save_file(ctx, cmd, data);
                druid::Handled::Yes
            }

            _ if cmd.is(druid::commands::SAVE_FILE_AS) => {
                controller::file::save_file_as(ctx, cmd, data);
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
            _ if cmd.is(commands::IMAGE_CLEAR) => {
                controller::image::clear(ctx, cmd, data);
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
    let mut menu_opts: view::MenuOpts = Default::default();

    menu_opts.disable(
        view::COMMON_MENU_FILE_SAVE.to_string(),
        data.path.is_none() || !data.pixels.dirty,
    );

    let selection = data.selection.is_none();

    menu_opts.disable(view::COMMON_MENU_UNDO.to_string(), data.undo.is_empty());
    menu_opts.disable(view::COMMON_MENU_REDO.to_string(), data.redo.is_empty());
    menu_opts.disable(view::COMMON_MENU_CUT.to_string(), selection);
    menu_opts.disable(view::COMMON_MENU_COPY.to_string(), selection);
    menu_opts.disable(view::EDIT_MENU_DESELECT.to_string(), selection);

    menu_opts.select(view::MENU_VIEW_SHOW_GRID.to_string(), data.show_grid);

    view::rebuild_menu_bar(ctx, cmd, &menu_opts);
}
