use druid::PlatformError;

mod canvas;
mod delegate;
mod file;
mod model;
mod palette;
mod theme;
mod tool_button;
mod view;

use model::AppState;

fn main() -> Result<(), PlatformError> {
    let ui = view::build_ui();

    let main_window = druid::WindowDesc::new(ui)
        .title("Pix Wiz")
        .menu(view::build_menu_bar())
        .window_size((672.0, 712.0));

    let data = AppState::new();

    druid::AppLauncher::with_window(main_window)
        .delegate(delegate::Delegate)
        .use_env_tracing()
        .launch(data)
}
