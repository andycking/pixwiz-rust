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
