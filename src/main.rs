use druid::widget::prelude::*;
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

struct Palette {
    colors: Vec<Color>,
}

impl Palette {
    pub fn new() -> Self {
        Self {
            colors: Self::read_palette(),
        }
    }

    pub fn idx_to_point(idx: usize) -> druid::Point {
        let y = (idx / 16) as f64;
        let x = (idx % 16) as f64;
        druid::Point::new(1.0 + (x * (10.0 + 1.0)), 1.0 + (y * (10.0 + 1.0)))
    }

    pub fn idx_to_rect(idx: usize) -> druid::Rect {
        let origin = Self::idx_to_point(idx);
        druid::Rect::from_origin_size(origin, (10.0, 10.0))
    }

    fn read_palette() -> Vec<Color> {
        let bytes = include_bytes!("./assets/vga.pal");

        assert!(bytes.len() % 16 == 0);

        let mut colors: Vec<Color> = Vec::new();

        let mut i = 0;
        while i < bytes.len() {
            let a = bytes[i + 0];
            let r = bytes[i + 1];
            let g = bytes[i + 2];
            let b = bytes[i + 3];
            colors.push(Color::rgba8(r, g, b, a));
            i += 4;
        }

        colors
    }
}

impl Widget<PixWizState> for Palette {
    fn event(&mut self, _ctx: &mut EventCtx, _event: &Event, _data: &mut PixWizState, _env: &Env) {}

    fn lifecycle(
        &mut self,
        _ctx: &mut LifeCycleCtx,
        _event: &LifeCycle,
        _data: &PixWizState,
        _env: &Env,
    ) {
    }

    fn update(
        &mut self,
        _ctx: &mut UpdateCtx,
        _old_data: &PixWizState,
        _data: &PixWizState,
        _env: &Env,
    ) {
    }

    fn layout(
        &mut self,
        _layout_ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        _data: &PixWizState,
        _env: &Env,
    ) -> Size {
        let rect = Self::idx_to_rect(self.colors.len() - 1);
        let size = Size::new(rect.x1 + 1.0, rect.y1 + 1.0);
        bc.constrain(size)
    }

    fn paint(&mut self, ctx: &mut PaintCtx, _data: &PixWizState, _env: &Env) {
        let mut i = 0;
        for color in &self.colors {
            let rect = Self::idx_to_rect(i);
            ctx.fill(rect, color);
            i += 1;
        }
    }
}

fn build_palette() -> impl Widget<PixWizState> {
    Flex::column()
        .with_child(Palette::new())
        .background(Color::BLACK)
}

fn build_left_pane() -> impl Widget<PixWizState> {
    Flex::column()
        .with_child(build_tools())
        .with_default_spacer()
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
                .with_default_spacer()
                .with_child(build_left_pane())
                .with_default_spacer()
                .with_child(build_canvas())
                .with_default_spacer()
                .with_child(build_right_pane()),
        )
        .with_default_spacer()
        .background(Color::WHITE)
}
