use crate::model::app_state::AppState;
use crate::view;

pub fn show_grid(ctx: &mut druid::DelegateCtx, cmd: &druid::Command, data: &mut AppState) {
    data.show_grid = !data.show_grid;

    let mut menu_opts: view::MenuOpts = Default::default();
    menu_opts.select(view::MENU_VIEW_SHOW_GRID.to_string(), data.show_grid);
    view::rebuild_menu_bar(ctx, cmd, &menu_opts);
}
