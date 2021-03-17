use druid::PlatformError;

mod controller;
mod model;
mod storage;
mod view;

use controller::delegate::Delegate;
use model::state::AppState;

fn main() -> Result<(), PlatformError> {
    let ui = view::build_ui();

    let main_window = druid::WindowDesc::new(ui)
        .title("Pix Wiz")
        .menu(view::build_menu_bar(true))
        .window_size((672.0, 712.0));

    let data = AppState::new();

    druid::AppLauncher::with_window(main_window)
        .delegate(Delegate)
        .use_env_tracing()
        .launch(data)
}
