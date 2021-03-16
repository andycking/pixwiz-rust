use crate::model::state::AppState;
use crate::model::state::PixelState;
use crate::storage;

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

            _ if cmd.is(druid::commands::OPEN_FILE) => {
                open_file(cmd, data);
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
    if data.pixels.dirty {
        // Ask the user if they'd like to save the current image first.
    }

    data.pixels = PixelState::empty();
}

fn open_file(cmd: &druid::Command, data: &mut AppState) {
    // If the file dialog passes us an invalid path then all bets are off. Just let it panic.
    let file_info = cmd.get(druid::commands::OPEN_FILE).unwrap();
    let path = file_info.path().to_str().unwrap();

    match storage::png::read(path) {
        Ok(image_data) => {
            data.pixels = PixelState::from(&image_data);
            data.path = Some(String::from(path));
        }
        Err(_e) => {}
    }
}

fn save_file(_cmd: &druid::Command, data: &mut AppState) {
    let image_data = storage::image_data::ImageData::from(&data.pixels);

    match &data.path {
        Some(path) => match storage::png::write(path, &image_data) {
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

    let image_data = storage::image_data::ImageData::from(&data.pixels);

    match storage::png::write(path, &image_data) {
        Ok(()) => {
            data.path = Some(String::from(path));
        }
        Err(_e) => {}
    }
}
