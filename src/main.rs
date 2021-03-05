use std::ops::{Index, IndexMut};
use std::sync::Arc;

use druid::widget::prelude::*;
use druid::widget::{Flex};
use druid::{Color, Data, PlatformError, Widget, WidgetExt};

fn main() -> Result<(), PlatformError> {
    let main_window = druid::WindowDesc::new(ui_builder())
        .title("PixWiz")
        .window_size((800.0, 556.0))
        .resizable(false);
    let data = PixWizState::new();
    druid::AppLauncher::with_window(main_window)
        .use_env_tracing()
        .launch(data)
}

#[derive(Clone, Data)]
struct PixelState {
    storage: Arc<[u32; 1024]>,
}

impl PixelState {
    pub fn new() -> Self {
        Self {
            storage: Arc::new(Self::build_pixels()),
        }
    }

    pub fn len(&self) -> usize {
        self.storage.len()
    }

    fn build_pixels() -> [u32; 1024] {
        let mut pixels: [u32; 1024] = [0; 1024];

        let mut i = 0;
        for x in 0..32 {
            for y in 0..32 {
                pixels[i] = match (x + y) % 2 {
                    0 => 0x505050ff,
                    _ => 0x606060ff,
                };
                i += 1;
            }
        }

        pixels
    }
}

impl Index<usize> for PixelState {
    type Output = u32;
    fn index(&self, idx: usize) -> &Self::Output {
        &self.storage[idx]
    }
}

impl IndexMut<usize> for PixelState {
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        Arc::make_mut(&mut self.storage).index_mut(idx)
    }
}

#[derive(Clone, Data)]
struct PaletteState {
    storage: Arc<[u32; 256]>,
}

impl PaletteState {
    pub fn new() -> Self {
        Self {
            storage: Arc::new(Self::read_palette()),
        }
    }

    pub fn len(&self) -> usize {
        self.storage.len()
    }

    fn read_palette() -> [u32; 256] {
        let bytes = include_bytes!("./assets/vga.pal");

        assert!(bytes.len() == 1024);

        let mut palette: [u32; 256] = [0; 256];

        for i in 0..palette.len() {
            let j = i * 4;
            let argb = [bytes[j + 0], bytes[j + 1], bytes[j + 2], bytes[j + 3]];
            palette[i] = u32::from_le_bytes(argb);
        }

        palette
    }
}

impl Index<usize> for PaletteState {
    type Output = u32;
    fn index(&self, idx: usize) -> &Self::Output {
        &self.storage[idx]
    }
}

impl IndexMut<usize> for PaletteState {
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        Arc::make_mut(&mut self.storage).index_mut(idx)
    }
}

#[derive(Clone, Data)]
struct PixWizState {
    fg: u32,
    bg: u32,
    pos: (usize, usize),
    pixels: PixelState,
    palette: PaletteState,
}

impl PixWizState {
    pub fn new() -> Self {
        Self {
            fg: Color::BLACK.as_rgba_u32(),
            bg: Color::WHITE.as_rgba_u32(),
            pos: (0, 0),
            pixels: PixelState::new(),
            palette: PaletteState::new(),
        }
    }
}

fn tools_button(bytes: &[u8]) -> impl Widget<PixWizState> {
    let png_data = druid::ImageBuf::from_data(bytes).unwrap();

    druid::widget::Image::new(png_data).fill_mode(druid::widget::FillStrat::Cover)
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

struct Palette {}

impl Palette {
    pub fn new() -> Self {
        Self {}
    }

    fn idx_to_point(idx: usize) -> druid::Point {
        let y = (idx / 16) as f64;
        let x = (idx % 16) as f64;
        druid::Point::new(1.0 + (x * (10.0 + 1.0)), 1.0 + (y * (10.0 + 1.0)))
    }

    fn idx_to_rect(idx: usize) -> druid::Rect {
        let origin = Self::idx_to_point(idx);
        druid::Rect::from_origin_size(origin, (10.0, 10.0))
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
        data: &PixWizState,
        _env: &Env,
    ) -> Size {
        let rect = Self::idx_to_rect(data.palette.len() - 1);
        let size = Size::new(rect.x1 + 1.0, rect.y1 + 1.0);
        bc.constrain(size)
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &PixWizState, _env: &Env) {
        for i in 0..data.palette.len() {
            let rect = Self::idx_to_rect(i);
            let rgba = Color::from_rgba32_u32(data.palette[i]);
            ctx.fill(rect, &rgba);
        }
    }
}

struct Canvas {}

impl Canvas {
    pub fn new() -> Self {
        Self {}
    }

    fn idx_to_point(idx: usize) -> druid::Point {
        let y = (idx / 32) as f64;
        let x = (idx % 32) as f64;
        druid::Point::new(x * 16.0, y * 16.0)
    }

    fn idx_to_rect(idx: usize) -> druid::Rect {
        let origin = Self::idx_to_point(idx);
        druid::Rect::from_origin_size(origin, (16.0, 16.0))
    }
}

impl Widget<PixWizState> for Canvas {
    fn event(&mut self, _ctx: &mut EventCtx, event: &Event, data: &mut PixWizState, _env: &Env) {
        match event {
            Event::MouseMove(e) => {
                let x = std::cmp::min(e.pos.x as usize / 16 + 1, 32);
                let y = std::cmp::min(e.pos.y as usize / 16 + 1, 32);
                data.pos = (x, y);
            }

            _ => {}
        }
    }

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
        data: &PixWizState,
        _env: &Env,
    ) -> Size {
        let rect = Self::idx_to_rect(data.pixels.len() - 1);
        let size = Size::new(rect.x1, rect.y1);
        bc.constrain(size)
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &PixWizState, _env: &Env) {
        for i in 0..data.pixels.len() {
            let rect = Self::idx_to_rect(i);
            let rgba = Color::from_rgba32_u32(data.pixels[i]);
            ctx.fill(rect, &rgba);
        }
    }
}

fn build_color_well() -> impl Widget<PixWizState> {
    Flex::column().with_child(
        druid::widget::Painter::new(|ctx, data: &PixWizState, _env| {
            let rect = ctx.size().to_rect();
            let rgba = Color::from_rgba32_u32(data.fg);
            ctx.fill(rect, &rgba);
        })
        .fix_size(65.0, 30.0)
        .border(Color::BLACK, 1.0)
    )
}

fn build_left_pane() -> impl Widget<PixWizState> {
    Flex::column()
        .cross_axis_alignment(druid::widget::CrossAxisAlignment::End)
        .with_child(build_tools())
        .with_default_spacer()
        .with_child(build_color_well())
        .with_default_spacer()
}

fn build_canvas() -> impl Widget<PixWizState> {
    Flex::column()
        .with_child(Canvas::new())
        .border(Color::BLACK, 1.0)
}

fn build_palette() -> impl Widget<PixWizState> {
    Flex::column()
        .with_child(Palette::new())
        .background(Color::BLACK)
}

fn build_right_pane() -> impl Widget<PixWizState> {
    build_palette()
}

fn build_status_bar() -> impl Widget<PixWizState> {
    Flex::row()
        .with_child(
            druid::widget::Label::new(|data: &PixWizState, _env: &_| {
                format!("{:08x} {:2}:{:2}", data.fg, data.pos.0, data.pos.1)
            })
            .with_font(druid::FontDescriptor::new(druid::FontFamily::MONOSPACE))
            .with_text_color(Color::BLACK)
            .padding(3.0),
        )
        .main_axis_alignment(druid::widget::MainAxisAlignment::End)
        .must_fill_main_axis(true)
        .background(Color::rgb8(101, 123, 131))
}

fn ui_builder() -> impl Widget<PixWizState> {
    Flex::column()
        .cross_axis_alignment(druid::widget::CrossAxisAlignment::End)
        .with_default_spacer()
        .with_child(
            Flex::row()
                .cross_axis_alignment(druid::widget::CrossAxisAlignment::Start)
                .with_default_spacer()
                .with_child(build_left_pane())
                .with_default_spacer()
                .with_child(build_canvas())
                .with_default_spacer()
                .with_child(build_right_pane())
                .with_default_spacer(),
        )
        .with_default_spacer()
        .with_child(build_status_bar())
        .with_default_spacer()
        .background(Color::rgb8(0, 43, 54))
}
