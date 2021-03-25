use crate::commands;
use crate::controller;
use crate::model::state::AppState;

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
        match cmd {
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
            _ if cmd.is(commands::IMAGE_FILL) => {
                controller::image::fill(ctx, cmd, data);
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
        }
    }
}
