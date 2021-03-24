use druid::PlatformError;

mod controller;
mod model;
mod storage;
mod transforms;
mod view;

use controller::delegate::Delegate;
use model::state::AppState;
use view::MenuOpts;

fn main() -> Result<(), PlatformError> {
    let ui = view::build_ui();

    let menu_opts: MenuOpts = Default::default();
    let menu_bar = view::build_menu_bar(&menu_opts);

    let main_window = druid::WindowDesc::new(ui)
        .title("Pix Wiz")
        .menu(menu_bar)
        .window_size((672.0, 712.0));

    let data = AppState::new();

    druid::AppLauncher::with_window(main_window)
        .delegate(Delegate)
        .use_env_tracing()
        .launch(data)
}
