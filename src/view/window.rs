// Copyright 2021 Andy King
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use druid::widget::prelude::*;
use druid::widget::Flex;
use druid::WidgetExt;

use super::canvas::Canvas;
use super::canvas::CanvasController;
use super::menu;
use super::palette::Palette;
use super::theme;
use super::tool::ToolButton;
use super::tool::ToolsController;
use crate::model::app::AppState;
use crate::model::types::*;

pub fn window() -> druid::WindowDesc<AppState> {
    let ui = build_ui();

    let menu_opts: menu::MenuOpts = Default::default();
    let menu_bar = menu::menu_bar(&menu_opts);

    druid::WindowDesc::new(ui)
        .title("Pix Wiz")
        .menu(menu_bar)
        .window_size(theme::WINDOW_SIZE)
}

fn build_ui() -> impl druid::Widget<AppState> {
    Flex::column()
        .cross_axis_alignment(druid::widget::CrossAxisAlignment::End)
        .with_default_spacer()
        .with_child(build_main_pane())
        .with_default_spacer()
        .background(theme::MAIN_FILL)
        .controller(WindowController {})
}

fn build_tools_row<T: druid::Data>(
    a: impl Widget<T> + 'static,
    b: impl Widget<T> + 'static,
) -> impl Widget<T> {
    Flex::row()
        .with_spacer(theme::TOOLS_PADDING)
        .with_child(a)
        .with_spacer(theme::TOOLS_PADDING)
        .with_child(b)
        .with_spacer(theme::TOOLS_PADDING)
}

fn build_tools() -> impl druid::Widget<AppState> {
    let marquee_bytes = include_bytes!("../assets/marquee.png");
    let move_bytes = include_bytes!("../assets/move.png");
    let paint_bytes = include_bytes!("../assets/paint.png");
    let eraser_bytes = include_bytes!("../assets/eraser.png");
    let fill_bytes = include_bytes!("../assets/fill.png");
    let dropper_bytes = include_bytes!("../assets/dropper.png");

    Flex::column()
        .with_spacer(1.0)
        .with_child(build_tools_row(
            ToolButton::new(ToolType::Marquee, marquee_bytes),
            ToolButton::new(ToolType::Move, move_bytes),
        ))
        .with_spacer(theme::TOOLS_PADDING)
        .with_child(build_tools_row(
            ToolButton::new(ToolType::Paint, paint_bytes),
            ToolButton::new(ToolType::Eraser, eraser_bytes),
        ))
        .with_spacer(theme::TOOLS_PADDING)
        .with_child(build_tools_row(
            ToolButton::new(ToolType::Fill, fill_bytes),
            ToolButton::new(ToolType::Dropper, dropper_bytes),
        ))
        .with_spacer(theme::TOOLS_PADDING)
        .controller(ToolsController)
}

fn build_color_well() -> impl druid::Widget<AppState> {
    druid::widget::Painter::new(|ctx, data: &AppState, _env| {
        let rect = ctx.size().to_rect();
        let color = match data.tool_type() {
            ToolType::Dropper => data.pos_color(),
            _ => data.brush_color(),
        };
        ctx.fill(rect, color);
    })
    .fix_size(theme::COLOR_WELL_SIZE.width, theme::COLOR_WELL_SIZE.height)
    .border(theme::COLOR_WELL_STROKE, 1.0)
}

fn build_left_pane() -> impl druid::Widget<AppState> {
    Flex::column()
        .cross_axis_alignment(druid::widget::CrossAxisAlignment::End)
        .with_child(build_tools())
        .with_default_spacer()
        .with_child(build_color_well())
}

fn build_canvas() -> impl druid::Widget<AppState> {
    Canvas::new().controller(CanvasController)
}

fn build_palette() -> impl druid::Widget<AppState> {
    Palette::new(include_bytes!("../assets/vga.pal")).background(theme::PALETTE_FILL)
}

fn build_preview() -> impl druid::Widget<AppState> {
    druid::widget::Painter::new(|ctx, data: &AppState, _env| {
        let pixels = data.doc().pixels();
        let header = pixels.header();
        for y in 0..header.height() {
            for x in 0..header.width() {
                let rect = druid::Rect::new(x as f64, y as f64, (x as f64) + 1.0, (y as f64) + 1.0);

                let color = pixels.read_xy(x, y);
                let (_, _, _, a) = color.as_rgba8();
                if a != 255 {
                    ctx.fill(rect, &theme::PREVIEW_FILL);
                };

                ctx.fill(rect, &color);
            }
        }
    })
    .fix_size(theme::PREVIEW_SIZE.width, theme::PREVIEW_SIZE.height)
    .border(theme::PREVIEW_STROKE, 1.0)
}

fn build_right_pane() -> impl druid::Widget<AppState> {
    Flex::column()
        .with_child(build_preview())
        .with_default_spacer()
}

fn build_center_pane() -> impl druid::Widget<AppState> {
    Flex::column()
        .cross_axis_alignment(druid::widget::CrossAxisAlignment::End)
        .with_child(build_canvas())
        .with_default_spacer()
        .with_child(build_palette())
        .with_default_spacer()
        .with_child(build_status_bar())
        .with_default_spacer()
}

fn build_main_pane() -> impl druid::Widget<AppState> {
    Flex::row()
        .cross_axis_alignment(druid::widget::CrossAxisAlignment::Start)
        .with_default_spacer()
        .with_child(build_left_pane())
        .with_default_spacer()
        .with_child(build_center_pane())
        .with_default_spacer()
        .with_child(build_right_pane())
        .with_default_spacer()
}

fn build_status_label() -> impl druid::Widget<AppState> {
    druid::widget::Label::new(|data: &AppState, _env: &_| {
        let (r, g, b, a) = data.pos_color().as_rgba8();
        let current_pos = data.current_pos();
        let selection = data.doc().selection().unwrap_or(druid::Rect::ZERO);

        format!(
            "r:{:3} g:{:3} b:{:3} a:{:3}  {:02}:{:02}-{:02}:{:02}  {:02}:{:02}",
            r,
            g,
            b,
            a,
            selection.x0,
            selection.y0,
            selection.x1,
            selection.y1,
            current_pos.x,
            current_pos.y
        )
    })
    .with_font(druid::FontDescriptor::new(druid::FontFamily::MONOSPACE))
    .with_text_color(theme::STATUS_BAR_STROKE)
}

fn build_status_bar() -> impl druid::Widget<AppState> {
    Flex::row()
        .main_axis_alignment(druid::widget::MainAxisAlignment::End)
        .must_fill_main_axis(true)
        .with_child(build_status_label())
        .background(theme::STATUS_BAR_FILL)
}

pub struct WindowController {}

impl<W: Widget<AppState>> druid::widget::Controller<AppState, W> for WindowController {
    fn event(
        &mut self,
        child: &mut W,
        ctx: &mut EventCtx<'_, '_>,
        event: &Event,
        data: &mut AppState,
        env: &Env,
    ) {
        // Remember where this window is, just in case we need to center an alert.
        data.set_window_pos(ctx.window().get_position());

        let block = matches!(
            event,
            Event::MouseUp(_)
                | Event::MouseDown(_)
                | Event::MouseMove(_)
                | Event::KeyUp(_)
                | Event::KeyDown(_)
                | Event::Paste(_)
                | Event::Wheel(_)
                | Event::Zoom(_)
        );

        if data.window_state() == WindowState::Normal || !block {
            child.event(ctx, event, data, env);
        }
    }
}
