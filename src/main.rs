use druid::widget::{CrossAxisAlignment, FillStrat, Flex, Image};
use druid::{AppLauncher, Color, Data, ImageBuf, PlatformError, Widget, WidgetExt, WindowDesc};

fn main() -> Result<(), PlatformError> {
    let main_window = WindowDesc::new(ui_builder());
    let data = PixWizState {};
    AppLauncher::with_window(main_window)
        .use_env_tracing()
        .launch(data)
}

#[derive(Clone, Data)]
struct PixWizState {}

fn tool_pane_button(bytes: &[u8]) -> impl Widget<PixWizState> {
    let png_data = ImageBuf::from_data(bytes).unwrap();

    Image::new(png_data)
        .fill_mode(FillStrat::Cover)
        .border(Color::BLACK, 1.0)
}

fn tool_pane_flex_row<T: Data>(
    a: impl Widget<T> + 'static,
    b: impl Widget<T> + 'static,
) -> impl Widget<T> {
    Flex::row().with_child(a).with_child(b)
}

fn build_tool_pane() -> impl Widget<PixWizState> {
    Flex::column()
        .cross_axis_alignment(CrossAxisAlignment::End)
        .with_child(tool_pane_flex_row(
            tool_pane_button(include_bytes!("./assets/marquee.png")),
            tool_pane_button(include_bytes!("./assets/lasso.png")),
        ))
        .with_child(tool_pane_flex_row(
            tool_pane_button(include_bytes!("./assets/move.png")),
            tool_pane_button(include_bytes!("./assets/zoom.png")),
        ))
        .with_child(tool_pane_flex_row(
            tool_pane_button(include_bytes!("./assets/cropper.png")),
            tool_pane_button(include_bytes!("./assets/type.png")),
        ))
        .with_child(tool_pane_flex_row(
            tool_pane_button(include_bytes!("./assets/paint.png")),
            tool_pane_button(include_bytes!("./assets/eraser.png")),
        ))
        .with_child(tool_pane_flex_row(
            tool_pane_button(include_bytes!("./assets/fill.png")),
            tool_pane_button(include_bytes!("./assets/dropper.png")),
        ))
}

fn ui_builder() -> impl Widget<PixWizState> {
    Flex::column()
        .with_child(build_tool_pane())
        .background(Color::WHITE)
}
