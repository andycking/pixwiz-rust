use crate::controller::undo;
use crate::model::app_state::AppState;

pub fn undo(_ctx: &mut druid::DelegateCtx, _cmd: &druid::Command, data: &mut AppState) {
    undo::pop(data);
}

pub fn redo(_ctx: &mut druid::DelegateCtx, _cmd: &druid::Command, data: &mut AppState) {
    undo::pop_redo(data);
}

pub fn cut(_ctx: &mut druid::DelegateCtx, _cmd: &druid::Command, _data: &mut AppState) {}

pub fn copy(_ctx: &mut druid::DelegateCtx, _cmd: &druid::Command, _data: &mut AppState) {}

pub fn paste(_ctx: &mut druid::DelegateCtx, _cmd: &druid::Command, _data: &mut AppState) {}

pub fn select_all(_ctx: &mut druid::DelegateCtx, _cmd: &druid::Command, data: &mut AppState) {
    data.selection = Some(druid::Rect::new(
        1.0,
        1.0,
        data.pixels.header.width as f64,
        data.pixels.header.height as f64,
    ));
}

pub fn deselect(_ctx: &mut druid::DelegateCtx, _cmd: &druid::Command, data: &mut AppState) {
    data.selection = None;
}
