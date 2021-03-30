use druid::PlatformError;

mod commands;
mod controller;
mod model;
mod storage;
mod transforms;
mod view;

use controller::delegate::Delegate;
use model::app_state::AppState;
use view::MenuOpts;

fn main() -> Result<(), PlatformError> {
    let ui = view::build_ui();

    let menu_opts: MenuOpts = Default::default();
    let menu_bar = view::build_menu_bar(&menu_opts);

    let main_window = druid::WindowDesc::new(ui)
        .title("Pix Wiz")
        .menu(menu_bar)
        .window_size((672.0, 696.0));

    let data: AppState = Default::default();

    druid::AppLauncher::with_window(main_window)
        .delegate(Delegate)
        .use_env_tracing()
        .launch(data)
}
