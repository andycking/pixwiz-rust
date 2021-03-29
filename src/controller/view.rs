use crate::model::app_state::AppState;

pub fn show_grid(_ctx: &mut druid::DelegateCtx, _cmd: &druid::Command, data: &mut AppState) {
    data.show_grid = !data.show_grid;
}
