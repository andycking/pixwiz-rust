use druid::widget::{CrossAxisAlignment, FillStrat, Flex, Image, SizedBox};
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

fn tools_button(bytes: &[u8]) -> impl Widget<PixWizState> {
    let png_data = ImageBuf::from_data(bytes).unwrap();

    Image::new(png_data).fill_mode(FillStrat::Cover)
}

fn tools_row<T: Data>(a: impl Widget<T> + 'static, b: impl Widget<T> + 'static) -> impl Widget<T> {
    Flex::row()
        .with_spacer(1.0)
        .with_child(a)
        .with_spacer(1.0)
        .with_child(b)
        .with_spacer(1.0)
}

fn build_tools() -> impl Widget<PixWizState> {
    Flex::column()
        .with_spacer(1.0)
        .with_child(tools_row(
            tools_button(include_bytes!("./assets/marquee.png")),
            tools_button(include_bytes!("./assets/lasso.png")),
        ))
        .with_spacer(1.0)
        .with_child(tools_row(
            tools_button(include_bytes!("./assets/move.png")),
            tools_button(include_bytes!("./assets/zoom.png")),
        ))
        .with_spacer(1.0)
        .with_child(tools_row(
            tools_button(include_bytes!("./assets/cropper.png")),
            tools_button(include_bytes!("./assets/type.png")),
        ))
        .with_spacer(1.0)
        .with_child(tools_row(
            tools_button(include_bytes!("./assets/paint.png")),
            tools_button(include_bytes!("./assets/eraser.png")),
        ))
        .with_spacer(1.0)
        .with_child(tools_row(
            tools_button(include_bytes!("./assets/fill.png")),
            tools_button(include_bytes!("./assets/dropper.png")),
        ))
        .with_spacer(1.0)
        .background(Color::BLACK)
}

fn build_palette() -> impl Widget<PixWizState> {
    Flex::column()
}

fn build_left_pane() -> impl Widget<PixWizState> {
    Flex::column()
        .with_child(build_tools())
        .with_child(build_palette())
}

fn build_canvas() -> impl Widget<PixWizState> {
    SizedBox::empty()
        .width(320.0)
        .height(320.0)
        .border(Color::BLACK, 1.0)
}

fn build_right_pane() -> impl Widget<PixWizState> {
    SizedBox::empty()
}

fn ui_builder() -> impl Widget<PixWizState> {
    Flex::column()
        .with_default_spacer()
        .with_child(
            Flex::row()
                .cross_axis_alignment(CrossAxisAlignment::Start)
                .with_child(build_left_pane())
                .with_default_spacer()
                .with_child(build_canvas())
                .with_default_spacer()
                .with_child(build_right_pane()),
        )
        .with_default_spacer()
        .background(Color::WHITE)
}
