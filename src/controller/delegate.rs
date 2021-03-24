use crate::model::commands;
use crate::model::state::AppState;
use crate::storage;
use crate::transforms;
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
            // File.
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

            // Image.
            _ if cmd.is(commands::IMAGE_BLACK_AND_WHITE) => {
                black_and_white(ctx, cmd, data);
                druid::Handled::Yes
            }
            _ if cmd.is(commands::IMAGE_DESATURATE) => {
                desaturate(ctx, cmd, data);
                druid::Handled::Yes
            }
            _ if cmd.is(commands::IMAGE_DITHER_FLOYD) => {
                dither_floyd(ctx, cmd, data);
                druid::Handled::Yes
            }
            _ if cmd.is(commands::IMAGE_ERASE) => {
                erase(ctx, cmd, data);
                druid::Handled::Yes
            }
            _ if cmd.is(commands::IMAGE_FILL) => {
                fill(ctx, cmd, data);
                druid::Handled::Yes
            }

            // View.
            _ if cmd.is(commands::VIEW_SHOW_GRID) => {
                show_grid(ctx, cmd, data);
                druid::Handled::Yes
            }

            _ => druid::Handled::No,
        }
    }
}

fn new_file(_ctx: &mut druid::DelegateCtx, _cmd: &druid::Command, data: &mut AppState) {
    check_for_save(data);

    data.pixels = Default::default();
}

fn open_file(ctx: &mut druid::DelegateCtx, cmd: &druid::Command, data: &mut AppState) {
    check_for_save(data);

    let file_info = cmd.get_unchecked(druid::commands::OPEN_FILE);

    // If the file dialog passes us an invalid path then all bets are off. Just let it panic.
    let path = file_info.path().to_str().unwrap();

    match storage::png::read(path) {
        Ok(pixels) => {
            data.pixels = pixels;
            data.path = Some(String::from(path));
            enable_save(ctx, cmd);
        }
        Err(_e) => {}
    }
}

fn save_file(_ctx: &mut druid::DelegateCtx, _cmd: &druid::Command, data: &mut AppState) {
    match &data.path {
        Some(path) => match storage::png::write(path, &data.pixels) {
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

    match storage::png::write(path, &data.pixels) {
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
            let mut menu_opts: view::MenuOpts = Default::default();
            menu_opts
                .disabled
                .insert("common-menu-file-save".to_string(), false);
            let menu_bar: druid::MenuDesc<AppState> = view::build_menu_bar(&menu_opts);
            ctx.set_menu(menu_bar, id);
        }

        _ => {}
    }
}

fn black_and_white(_ctx: &mut druid::DelegateCtx, _cmd: &druid::Command, data: &mut AppState) {
    transforms::apply(data, transforms::colors::black_and_white);
}

fn desaturate(_ctx: &mut druid::DelegateCtx, _cmd: &druid::Command, data: &mut AppState) {
    transforms::apply(data, transforms::colors::desaturate);
}

fn dither_floyd(_ctx: &mut druid::DelegateCtx, _cmd: &druid::Command, data: &mut AppState) {
    transforms::apply(data, transforms::colors::dither_floyd);
}

fn erase(_ctx: &mut druid::DelegateCtx, _cmd: &druid::Command, data: &mut AppState) {
    transforms::apply(data, transforms::simple::erase);
}

fn fill(_ctx: &mut druid::DelegateCtx, _cmd: &druid::Command, data: &mut AppState) {
    transforms::apply(data, transforms::colors::fill);
}

fn show_grid(ctx: &mut druid::DelegateCtx, cmd: &druid::Command, _data: &mut AppState) {
    match cmd.target() {
        druid::Target::Window(id) => {
            let mut menu_opts: view::MenuOpts = Default::default();
            menu_opts
                .selected
                .insert("menu-view-show-grid".to_string(), false);
            let menu_bar: druid::MenuDesc<AppState> = view::build_menu_bar(&menu_opts);
            ctx.set_menu(menu_bar, id);
        }

        _ => {}
    }
}
