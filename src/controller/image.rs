use crate::model::state::AppState;
use crate::transforms;

pub fn black_and_white(_ctx: &mut druid::DelegateCtx, _cmd: &druid::Command, data: &mut AppState) {
    transforms::apply(data, transforms::colors::black_and_white);
}

pub fn desaturate(_ctx: &mut druid::DelegateCtx, _cmd: &druid::Command, data: &mut AppState) {
    transforms::apply(data, transforms::colors::desaturate);
}

pub fn dither_floyd(_ctx: &mut druid::DelegateCtx, _cmd: &druid::Command, data: &mut AppState) {
    transforms::apply(data, transforms::colors::dither_floyd);
}

pub fn erase(_ctx: &mut druid::DelegateCtx, _cmd: &druid::Command, data: &mut AppState) {
    transforms::apply(data, transforms::simple::erase);
}

pub fn fill(_ctx: &mut druid::DelegateCtx, _cmd: &druid::Command, data: &mut AppState) {
    transforms::apply(data, transforms::colors::fill);
}
