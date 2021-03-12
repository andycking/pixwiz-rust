use druid::widget::prelude::*;
use druid::widget::Flex;
use druid::{Data, Widget, WidgetExt};

use crate::model::AppState;
use crate::model::ToolType;
use crate::theme;
use crate::widgets::Canvas;
use crate::widgets::Palette;
use crate::widgets::ToolButton;

pub fn build_ui() -> impl Widget<AppState> {
    Flex::column()
        .cross_axis_alignment(druid::widget::CrossAxisAlignment::End)
        .with_default_spacer()
        .with_child(build_main_pane())
        .with_default_spacer()
        .with_child(build_status_bar())
        .with_default_spacer()
        .background(theme::MAIN_FILL)
}

fn build_tools_row<T: Data>(
    a: impl Widget<T> + 'static,
    b: impl Widget<T> + 'static,
) -> impl Widget<T> {
    Flex::row()
        .with_spacer(8.0)
        .with_child(a)
        .with_spacer(8.0)
        .with_child(b)
        .with_spacer(8.0)
}

fn build_tools() -> impl Widget<AppState> {
    let marquee_bytes = include_bytes!("./assets/marquee.png");
    let lasso_bytes = include_bytes!("./assets/lasso.png");
    let move_bytes = include_bytes!("./assets/move.png");
    let cropper_bytes = include_bytes!("./assets/cropper.png");
    let paint_bytes = include_bytes!("./assets/paint.png");
    let eraser_bytes = include_bytes!("./assets/eraser.png");
    let fill_bytes = include_bytes!("./assets/fill.png");
    let dropper_bytes = include_bytes!("./assets/dropper.png");

    Flex::column()
        .with_spacer(8.0)
        .with_child(build_tools_row(
            ToolButton::new(ToolType::Marquee, marquee_bytes),
            ToolButton::new(ToolType::Lasso, lasso_bytes),
        ))
        .with_spacer(8.0)
        .with_child(build_tools_row(
            ToolButton::new(ToolType::Move, move_bytes),
            ToolButton::new(ToolType::Cropper, cropper_bytes),
        ))
        .with_spacer(8.0)
        .with_child(build_tools_row(
            ToolButton::new(ToolType::Paint, paint_bytes),
            ToolButton::new(ToolType::Eraser, eraser_bytes),
        ))
        .with_spacer(8.0)
        .with_child(build_tools_row(
            ToolButton::new(ToolType::Fill, fill_bytes),
            ToolButton::new(ToolType::Dropper, dropper_bytes),
        ))
        .with_spacer(8.0)
}

fn build_color_well() -> impl Widget<AppState> {
    druid::widget::Painter::new(|ctx, data: &AppState, _env| {
        let rect = ctx.size().to_rect();
        let color = druid::Color::from_rgba32_u32(data.brush_color);
        ctx.fill(rect, &color);
    })
    .fix_size(88.0, 30.0)
    .border(theme::COLOR_WELL_STROKE, 1.0)
}

fn build_left_pane() -> impl Widget<AppState> {
    Flex::column()
        .cross_axis_alignment(druid::widget::CrossAxisAlignment::End)
        .with_child(build_tools())
        .with_default_spacer()
        .with_child(build_color_well())
        .with_default_spacer()
        .with_child(build_palette())
}

fn build_canvas() -> impl Widget<AppState> {
    Canvas::new()
}

fn build_palette() -> impl Widget<AppState> {
    Palette::new(include_bytes!("./assets/vga.pal")).background(theme::PALETTE_FILL)
}

fn build_preview() -> impl Widget<AppState> {
    druid::widget::Painter::new(|ctx, data: &AppState, _env| {
        let mut i = 0;
        for y in 0..32 {
            for x in 0..32 {
                let rect = druid::Rect::new(x as f64, y as f64, (x as f64) + 1.0, (y as f64) + 1.0);

                let mut value = data.pixels[i];
                value = match value & 0xff {
                    0 => theme::CHECKERBOARD_FILL_LIGHT,
                    _ => value,
                };

                let color = druid::Color::from_rgba32_u32(value);
                ctx.fill(rect, &color);

                i += 1;
            }
        }
    })
    .fix_size(32.0, 32.0)
    .border(theme::PREVIEW_STROKE, 1.0)
}

fn build_right_pane() -> impl Widget<AppState> {
    Flex::column()
        .with_default_spacer()
        .with_child(build_preview())
        .with_default_spacer()
}

fn build_main_pane() -> impl Widget<AppState> {
    Flex::row()
        .cross_axis_alignment(druid::widget::CrossAxisAlignment::Start)
        .with_default_spacer()
        .with_child(build_left_pane())
        .with_default_spacer()
        .with_child(build_canvas())
        .with_default_spacer()
        .with_child(build_right_pane())
        .with_default_spacer()
}

fn build_status_label() -> impl Widget<AppState> {
    druid::widget::Label::new(|data: &AppState, _env: &_| {
        let color = druid::Color::from_rgba32_u32(data.pos_color);
        let (r, g, b, a) = color.as_rgba8();
        format!(
            "{:>10}  r:{:3} g:{:3} b:{:3} a:{:3}  {:02}:{:02}-{:02}:{:02}  {:02}:{:02}",
            data.tool_type.to_string().to_lowercase(),
            r,
            g,
            b,
            a,
            data.selection.0 .0,
            data.selection.0 .1,
            data.selection.1 .0,
            data.selection.1 .1,
            data.current_pos.0,
            data.current_pos.1
        )
    })
    .with_font(druid::FontDescriptor::new(druid::FontFamily::MONOSPACE))
    .with_text_color(theme::STATUS_BAR_STROKE)
    .padding(3.0)
}

fn build_status_bar() -> impl Widget<AppState> {
    Flex::row()
        .main_axis_alignment(druid::widget::MainAxisAlignment::End)
        .must_fill_main_axis(true)
        .with_child(build_status_label())
        .background(theme::STATUS_BAR_FILL)
}
