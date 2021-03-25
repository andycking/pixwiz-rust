use crate::model;
use crate::model::app_state::AppState;
use crate::transforms;

pub fn black_and_white(_ctx: &mut druid::DelegateCtx, _cmd: &druid::Command, data: &mut AppState) {
    transforms::apply(data, transforms::colors::black_and_white);
}

pub fn clear(_ctx: &mut druid::DelegateCtx, _cmd: &druid::Command, data: &mut AppState) {
    transforms::apply(data, transforms::simple::clear);
}

pub fn desaturate(_ctx: &mut druid::DelegateCtx, _cmd: &druid::Command, data: &mut AppState) {
    transforms::apply(data, transforms::colors::desaturate);
}

pub fn dither_floyd(_ctx: &mut druid::DelegateCtx, _cmd: &druid::Command, data: &mut AppState) {
    transforms::apply(data, transforms::colors::dither_floyd);
}

pub fn eraser(_ctx: &mut druid::DelegateCtx, _cmd: &druid::Command, data: &mut AppState) {
    model::push_mod_record_point(data, data.current_pos);

    let idx = data.pixels.point_to_idx(data.current_pos);
    data.pixels.write(idx, &druid::Color::rgba8(0, 0, 0, 0));
}

pub fn fill(_ctx: &mut druid::DelegateCtx, _cmd: &druid::Command, data: &mut AppState) {
    transforms::apply(data, transforms::colors::fill);
}

pub fn paint(_ctx: &mut druid::DelegateCtx, _cmd: &druid::Command, data: &mut AppState) {
    model::push_mod_record_point(data, data.current_pos);

    let idx = data.pixels.point_to_idx(data.current_pos);
    data.pixels.write(idx, &data.brush_color);
}
