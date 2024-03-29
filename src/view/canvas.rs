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

use crate::common::commands;
use crate::model::app::AppState;
use crate::model::types::*;
use crate::util::shapes;
use crate::view::theme;

/// A canvas that allows for the display and modification of pixels. The size is currently
/// fixed at 32x32.
pub struct Canvas {
    long_dash: [druid::piet::StrokeStyle; 2],
    short_dash: [druid::piet::StrokeStyle; 2],
}

impl Canvas {
    /// Create an empty canvas.
    pub fn new() -> Self {
        Self {
            long_dash: [
                druid::piet::StrokeStyle::new().dash(vec![4.0], 0.0),
                druid::piet::StrokeStyle::new().dash(vec![4.0], 4.0),
            ],
            short_dash: [
                druid::piet::StrokeStyle::new().dash(vec![1.0], 0.0),
                druid::piet::StrokeStyle::new().dash(vec![1.0], 1.0),
            ],
        }
    }

    /// Translate from screen coordinates (typically the mouse position) to canvas coordinates.
    fn screen_coords_to_canvas_coords(pos: druid::Point) -> Option<druid::Point> {
        if pos.x < 1.0 || pos.y < 1.0 {
            return None;
        }

        let x = pos.x as usize / (theme::CANVAS_PIXEL_SIZE as usize) + 1;
        let y = pos.y as usize / (theme::CANVAS_PIXEL_SIZE as usize) + 1;
        if x > theme::CANVAS_DIMS || y > theme::CANVAS_DIMS {
            return None;
        }

        Some(druid::Point::new(x as f64, y as f64))
    }

    /// Translate from canvas coordinates to screen coordinates.
    fn canvas_coords_to_screen_coords(x: usize, y: usize) -> druid::Point {
        Self::canvas_coords_to_screen_coords_f64(x as f64, y as f64)
    }

    fn canvas_coords_to_screen_coords_f64(x: f64, y: f64) -> druid::Point {
        assert!(x > 0.0 && y > 0.0);
        druid::Point::new(
            1.0 + ((x - 1.0) * theme::CANVAS_PIXEL_SIZE),
            1.0 + ((y - 1.0) * theme::CANVAS_PIXEL_SIZE),
        )
    }

    /// Convert an index within the canvas storage to a rectanble in screen coordinates.
    fn screen_rect(x: usize, y: usize) -> druid::Rect {
        let fx = x as f64;
        let fy = y as f64;
        let origin = Self::canvas_coords_to_screen_coords_f64(fx, fy);

        druid::Rect::from_origin_size(origin, (theme::CANVAS_PIXEL_SIZE, theme::CANVAS_PIXEL_SIZE))
    }

    /// Paint border. The canvas does this internally instead of via border() because the
    /// pixels are already inset within the canvas (so that we can detect when the mouse
    /// leaves the area).
    fn paint_border(&self, ctx: &mut PaintCtx, _data: &AppState) {
        let rect = ctx.size().to_rect();
        let color = theme::CANVAS_STROKE;
        ctx.stroke(rect, &color, 1.0);
    }

    fn paint_pixel(ctx: &mut PaintCtx, x: usize, y: usize, color: &druid::Color) {
        let rect = Self::screen_rect(x, y);

        let (_, _, _, a) = color.as_rgba8();
        if a != 255 {
            let fill_color = if (x + y) % 2 == 0 {
                theme::CANVAS_FILL_DARK
            } else {
                theme::CANVAS_FILL_LIGHT
            };
            ctx.fill(rect, &fill_color);
        }

        ctx.fill(rect, color);
    }

    fn paint_pixels_moving(&self, ctx: &mut PaintCtx, data: &AppState) {
        let header = data.doc().pixels().header();
        let height = header.height();
        let width = header.width();

        let move_info = data.doc().move_info().unwrap();
        let selection = shapes::inflate_rect(data.doc().selection().unwrap());

        for y in 1..height + 1 {
            for x in 1..width + 1 {
                let p = druid::Point::new(x as f64, y as f64);

                let color = if selection.contains(p) {
                    let off_x = (x - selection.x0 as usize) + 1;
                    let off_y = (y - selection.y0 as usize) + 1;
                    move_info.pixels().read_xy_unchecked(off_x, off_y)
                } else {
                    data.doc().pixels().read_xy_unchecked(x, y)
                };

                Self::paint_pixel(ctx, x, y, &color);
            }
        }
    }

    fn paint_pixels_static(&self, ctx: &mut PaintCtx, data: &AppState) {
        let header = data.doc().pixels().header();
        let height = header.height();
        let width = header.width();

        for y in 1..height + 1 {
            for x in 1..width + 1 {
                let color = data.doc().pixels().read_xy_unchecked(x, y);
                Self::paint_pixel(ctx, x, y, &color);
            }
        }
    }

    /// Paint pixels from storage onto the given render context. This will paint
    /// on top of the checkboard. Pixel transparency is via alpha value.
    fn paint_pixels(&self, ctx: &mut PaintCtx, data: &AppState) {
        if data.doc().is_moving() {
            self.paint_pixels_moving(ctx, data);
        } else {
            self.paint_pixels_static(ctx, data);
        }
    }

    /// Paint a grid line onto the given render context.
    fn paint_grid_line(&self, ctx: &mut PaintCtx, x0: usize, y0: usize, x1: usize, y1: usize) {
        let a = Self::canvas_coords_to_screen_coords(x0, y0);
        let b = Self::canvas_coords_to_screen_coords(x1, y1);
        let line = druid::kurbo::Line::new(a, b);
        ctx.stroke_styled(
            line,
            &theme::CANVAS_STROKE_GRID_DARK,
            1.0,
            &self.short_dash[0],
        );
        ctx.stroke_styled(
            line,
            &theme::CANVAS_STROKE_GRID_LIGHT,
            1.0,
            &self.short_dash[1],
        );
    }

    /// Paint the grid onto the given render context.
    fn paint_grid(&self, ctx: &mut PaintCtx, data: &AppState) {
        if data.show_grid() {
            let header = data.doc().pixels().header();
            let height = header.height();
            let width = header.width();

            let num_lines = width / 8 + 1;
            for i in 1..num_lines {
                let offset = 1 + i * 8;
                self.paint_grid_line(ctx, offset, 1, offset, width + 1);
                self.paint_grid_line(ctx, 1, offset, height + 1, offset);
            }
        }
    }

    /// Paint the currently selected area onto the given render context.
    fn paint_selection(&self, ctx: &mut PaintCtx, data: &AppState) {
        if let Some(s) = data.doc().selection() {
            let tl = Self::canvas_coords_to_screen_coords_f64(s.x0, s.y0);
            let br = Self::canvas_coords_to_screen_coords_f64(s.x1, s.y1);

            let rect = druid::Rect::new(
                tl.x,
                tl.y,
                br.x + theme::CANVAS_PIXEL_SIZE,
                br.y + theme::CANVAS_PIXEL_SIZE,
            );

            ctx.stroke_styled(
                rect,
                &theme::CANVAS_STROKE_SELECTED_DARK,
                2.0,
                &self.long_dash[0],
            );
            ctx.stroke_styled(
                rect,
                &theme::CANVAS_STROKE_SELECTED_LIGHT,
                2.0,
                &self.long_dash[1],
            );
        }
    }

    /// Execute a tool at the given point on the canvas. The point is in
    /// canvas coordinates.
    fn tool(&mut self, ctx: &mut EventCtx, data: &mut AppState, state: ToolState) {
        match data.tool_type() {
            ToolType::Dropper => {
                let current_pos = data.current_pos();
                let color = data.doc().pixels().read(current_pos);

                data.set_brush_color(color);
            }

            ToolType::Eraser => {
                let current_pos = data.current_pos();
                let bounds = data.doc().bounds();

                if bounds.contains(current_pos) {
                    ctx.submit_command(commands::IMAGE_ERASER.with(state));
                }
            }

            ToolType::Fill => {
                let current_pos = data.current_pos();
                let bounds = data.doc().bounds();

                if bounds.contains(current_pos) {
                    ctx.submit_command(commands::IMAGE_FILL.with(true));
                }
            }

            ToolType::Marquee => {
                ctx.submit_command(commands::IMAGE_MARQUEE.with(state));
            }

            ToolType::Move => {
                ctx.submit_command(commands::IMAGE_MOVE.with(state));
            }

            ToolType::Paint => {
                let current_pos = data.current_pos();
                let bounds = data.doc().bounds();

                if bounds.contains(current_pos) {
                    ctx.submit_command(commands::IMAGE_PAINT.with(state));
                }
            }
        }
    }
}

impl druid::Widget<AppState> for Canvas {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut AppState, _env: &Env) {
        match event {
            Event::KeyUp(e) => match e.code {
                druid::Code::Delete | druid::Code::Backspace => {
                    ctx.submit_command(commands::IMAGE_CLEAR);
                }

                druid::Code::Escape => {
                    ctx.submit_command(commands::EDIT_DESELECT);
                }
                _ => {}
            },

            Event::MouseDown(e) => {
                if !e.focus {
                    match Self::screen_coords_to_canvas_coords(e.pos) {
                        Some(p) => {
                            data.set_start_pos(p);
                            data.set_current_pos(p);
                        }
                        _ => {
                            data.set_start_pos(druid::Point::ZERO);
                            data.set_current_pos(druid::Point::ZERO);
                        }
                    }
                    ctx.set_active(true);
                    self.tool(ctx, data, ToolState::Start);
                }
            }

            Event::MouseMove(e) => {
                let cursor = match data.tool_type() {
                    ToolType::Marquee => druid::Cursor::Crosshair,
                    _ => druid::Cursor::Arrow,
                };
                ctx.set_cursor(&cursor);

                let mut moved = false;

                match Self::screen_coords_to_canvas_coords(e.pos) {
                    Some(p) => {
                        // The screen coords might have changed, but that doesn't mean the
                        // canvas coords have changed (because of how big our pixels are).
                        // Avoid doing any work if we're still in the same place.
                        if p != data.current_pos() {
                            moved = true;

                            let color = data.doc().pixels().read(p);

                            data.set_pos_color(color);
                            data.set_current_pos(p);
                        }
                    }
                    None => {
                        if !ctx.is_active() && data.current_pos() != druid::Point::ZERO {
                            moved = true;

                            data.set_pos_color(data.brush_color().clone());
                            data.set_current_pos(druid::Point::ZERO);
                        }
                    }
                }

                if ctx.is_active() && moved {
                    self.tool(ctx, data, ToolState::Move);
                }
            }

            Event::MouseUp(_e) if ctx.is_active() => {
                ctx.set_active(false);
                self.tool(ctx, data, ToolState::End);
            }

            _ => {}
        }
    }

    fn lifecycle(
        &mut self,
        _ctx: &mut LifeCycleCtx,
        _event: &LifeCycle,
        _data: &AppState,
        _env: &Env,
    ) {
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &AppState, data: &AppState, _env: &Env) {
        if !old_data.same(data) {
            ctx.request_paint();
        }
    }

    fn layout(
        &mut self,
        _layout_ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        _data: &AppState,
        _env: &Env,
    ) -> Size {
        let rect = Self::screen_rect(theme::CANVAS_DIMS, theme::CANVAS_DIMS);
        let size = Size::new(rect.x1 + 1.0, rect.y1 + 1.0);
        bc.constrain(size)
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &AppState, _env: &Env) {
        self.paint_border(ctx, data);
        self.paint_pixels(ctx, data);
        self.paint_grid(ctx, data);
        self.paint_selection(ctx, data);
    }
}

/// A controller one level up from the canvas. We use this to steal the focus when the
/// app starts, so that key events go to the canvas.
pub struct CanvasController;

impl<W: Widget<AppState>> druid::widget::Controller<AppState, W> for CanvasController {
    fn event(
        &mut self,
        child: &mut W,
        ctx: &mut EventCtx<'_, '_>,
        event: &Event,
        data: &mut AppState,
        env: &Env,
    ) {
        if let Event::WindowConnected = event {
            ctx.request_focus();
        }

        child.event(ctx, event, data, env);
    }
}
