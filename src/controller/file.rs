use crate::model::app_state::AppState;
use crate::storage;

pub fn new_file(_ctx: &mut druid::DelegateCtx, _cmd: &druid::Command, data: &mut AppState) {
    check_for_save(data);

    data.pixels = Default::default();
}

pub fn open_file(_ctx: &mut druid::DelegateCtx, cmd: &druid::Command, data: &mut AppState) {
    check_for_save(data);

    let file_info = cmd.get_unchecked(druid::commands::OPEN_FILE);

    // If the file dialog passes us an invalid path then all bets are off. Just let it panic.
    let path = file_info.path().to_str().unwrap();

    match storage::png::read(path) {
        Ok(pixels) => {
            data.pixels = pixels;
            data.path = Some(String::from(path));
        }
        Err(_e) => {}
    }
}

pub fn save_file(_ctx: &mut druid::DelegateCtx, _cmd: &druid::Command, data: &mut AppState) {
    match &data.path {
        Some(path) => match storage::png::write(path, &data.pixels) {
            Ok(()) => {}
            Err(_e) => {}
        },
        _ => {}
    }
}

pub fn save_file_as(_ctx: &mut druid::DelegateCtx, cmd: &druid::Command, data: &mut AppState) {
    let file_info = cmd.get_unchecked(druid::commands::SAVE_FILE_AS);

    // If the file dialog passes us an invalid path then all bets are off. Just let it panic.
    let path = file_info.path().to_str().unwrap();

    match storage::png::write(path, &data.pixels) {
        Ok(()) => {
            data.path = Some(String::from(path));
        }
        Err(_e) => {}
    }
}

fn check_for_save(data: &mut AppState) {
    if data.pixels.dirty {}
}
