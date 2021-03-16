use crate::file;
use crate::model::AppState;
use crate::model::PixelState;

pub struct Delegate;

impl druid::AppDelegate<AppState> for Delegate {
    fn command(
        &mut self,
        _ctx: &mut druid::DelegateCtx,
        _target: druid::Target,
        cmd: &druid::Command,
        data: &mut AppState,
        _env: &druid::Env,
    ) -> druid::Handled {
        match cmd {
            _ if cmd.is(druid::commands::NEW_FILE) => {
                new_file(cmd, data);
                druid::Handled::Yes
            }

            _ if cmd.is(druid::commands::SAVE_FILE) => {
                save_file(cmd, data);
                druid::Handled::Yes
            }

            _ if cmd.is(druid::commands::SAVE_FILE_AS) => {
                save_file_as(cmd, data);
                druid::Handled::Yes
            }

            _ => druid::Handled::No,
        }
    }
}

fn new_file(_cmd: &druid::Command, data: &mut AppState) {
    if data.pixels.is_dirty() {
        // Ask the user if they'd like to save.
    }

    data.pixels = PixelState::new();
}

fn save_file(_cmd: &druid::Command, data: &mut AppState) {
    match &data.path {
        Some(path) => match file::write_png(path, &data) {
            Ok(()) => {}
            Err(_e) => {}
        },
        _ => {}
    }
}

fn save_file_as(cmd: &druid::Command, data: &mut AppState) {
    // If the file dialog passes us an invalid path then all bets are off. Just let it panic.
    let file_info = cmd.get(druid::commands::SAVE_FILE_AS).unwrap();
    let path = file_info.path().to_str().unwrap();

    match file::write_png(path, &data) {
        Ok(()) => {
            data.path = Some(String::from(path));
        }
        Err(_e) => {}
    }
}
