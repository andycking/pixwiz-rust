use druid::widget::{Button, FillStrat, Flex, Image, Label};
use druid::{AppLauncher, ImageBuf, LocalizedString, PlatformError, Widget, WidgetExt, WindowDesc};

fn main() -> Result<(), PlatformError> {
    let main_window = WindowDesc::new(ui_builder());
    let data = 0_u32;
    AppLauncher::with_window(main_window)
        .use_env_tracing()
        .launch(data)
}

fn ui_builder() -> impl Widget<u32> {
    let png_data = ImageBuf::from_data(include_bytes!("./assets/move.png")).unwrap();
    let mut img = Image::new(png_data).fill_mode(FillStrat::Cover);

    // The label text will be computed dynamically based on the current locale and count
    let text =
        LocalizedString::new("hello-counter").with_arg("count", |data: &u32, _env| (*data).into());
    let label = Label::new(text).padding(5.0).center();
    let button = Button::new("increment")
        .on_click(|_ctx, data, _env| *data += 1)
        .padding(5.0);

    Flex::column().with_child(label).with_child(button).with_child(img)
}
