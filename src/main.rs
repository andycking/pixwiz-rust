use druid::PlatformError;

mod model;
mod theme;
mod view;
mod widgets;

use model::AppState;

fn main() -> Result<(), PlatformError> {
    let ui = view::build_ui();

    let main_window = druid::WindowDesc::new(ui)
        .title("Pix Wiz")
        .window_size((680.0, 620.0))
        .resizable(false);

    let data = AppState::new();

    druid::AppLauncher::with_window(main_window)
        .use_env_tracing()
        .launch(data)
}
