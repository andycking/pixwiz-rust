use crate::model::commands;
use crate::model::state::AppState;
use crate::model::state::PixelState;
use crate::storage;
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
        match cmd {
            // File menu.
            _ if cmd.is(druid::commands::NEW_FILE) => {
                new_file(ctx, cmd, data);
                druid::Handled::Yes
            }

            _ if cmd.is(druid::commands::OPEN_FILE) => {
                open_file(ctx, cmd, data);
                druid::Handled::Yes
            }

            _ if cmd.is(druid::commands::SAVE_FILE) => {
                save_file(ctx, cmd, data);
                druid::Handled::Yes
            }

            _ if cmd.is(druid::commands::SAVE_FILE_AS) => {
                save_file_as(ctx, cmd, data);
                druid::Handled::Yes
            }

            // Image menu.
            _ if cmd.is(commands::CONVERT_TO_GRAYSCALE) => {
                convert_to_grayscale(ctx, cmd, data);
                druid::Handled::Yes
            }

            // View menu.
            _ if cmd.is(commands::SHOW_GRID) => {
                show_grid(ctx, cmd, data);
                druid::Handled::Yes
            }

            _ => druid::Handled::No,
        }
    }
}

fn new_file(_ctx: &mut druid::DelegateCtx, _cmd: &druid::Command, data: &mut AppState) {
    check_for_save(data);

    data.pixels = PixelState::empty();
}

fn open_file(ctx: &mut druid::DelegateCtx, cmd: &druid::Command, data: &mut AppState) {
    check_for_save(data);

    let file_info = cmd.get_unchecked(druid::commands::OPEN_FILE);

    // If the file dialog passes us an invalid path then all bets are off. Just let it panic.
    let path = file_info.path().to_str().unwrap();

    match storage::png::read(path) {
        Ok(image_data) => {
            data.pixels = PixelState::from(&image_data);
            data.path = Some(String::from(path));
            enable_save(ctx, cmd);
        }
        Err(_e) => {}
    }
}

fn save_file(_ctx: &mut druid::DelegateCtx, _cmd: &druid::Command, data: &mut AppState) {
    let image_data = storage::image_data::ImageData::from(&data.pixels);

    match &data.path {
        Some(path) => match storage::png::write(path, &image_data) {
            Ok(()) => {}
            Err(_e) => {}
        },
        _ => {}
    }
}

fn save_file_as(ctx: &mut druid::DelegateCtx, cmd: &druid::Command, data: &mut AppState) {
    let file_info = cmd.get_unchecked(druid::commands::SAVE_FILE_AS);

    // If the file dialog passes us an invalid path then all bets are off. Just let it panic.
    let path = file_info.path().to_str().unwrap();

    let image_data = storage::image_data::ImageData::from(&data.pixels);

    match storage::png::write(path, &image_data) {
        Ok(()) => {
            data.path = Some(String::from(path));
            enable_save(ctx, cmd);
        }
        Err(_e) => {}
    }
}

fn check_for_save(data: &mut AppState) {
    if data.pixels.dirty {}
}

fn enable_save(ctx: &mut druid::DelegateCtx, cmd: &druid::Command) {
    match cmd.target() {
        druid::Target::Window(id) => {
            let menu_bar: druid::MenuDesc<AppState> = view::build_menu_bar(false);
            ctx.set_menu(menu_bar, id);
        }

        _ => {}
    }
}

fn convert_to_grayscale(
    _ctx: &mut druid::DelegateCtx,
    _cmd: &druid::Command,
    _data: &mut AppState,
) {
}

fn show_grid(_ctx: &mut druid::DelegateCtx, _cmd: &druid::Command, _data: &mut AppState) {}
