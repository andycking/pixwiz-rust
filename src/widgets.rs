use std::sync::Arc;

use druid::widget::prelude::*;
use druid::{Color, Data, Widget};

use crate::model::AppState;
use crate::model::ToolType;
use crate::theme;

#[derive(Clone, Data)]
pub struct ToolButton {
    tool_type: ToolType,
    image_buf: Arc<druid::ImageBuf>,
}

impl ToolButton {
    pub fn new(tool_type: ToolType, bytes: &[u8]) -> Self {
        let image_buf = druid::ImageBuf::from_data(bytes).unwrap();

        Self {
            tool_type: tool_type,
            image_buf: Arc::new(image_buf),
        }
    }
}

impl Widget<AppState> for ToolButton {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut AppState, _env: &Env) {
        match event {
            Event::MouseDown(_e) => {
                ctx.set_active(true);
            }

            Event::MouseUp(_e) if ctx.is_active() => {
                if ctx.is_hot() {
                    data.tool_type = self.tool_type;
                }
                ctx.set_active(false);
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

    fn update(&mut self, _ctx: &mut UpdateCtx, _old_data: &AppState, _data: &AppState, _env: &Env) {
    }

    fn layout(
        &mut self,
        _layout_ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        _data: &AppState,
        _env: &Env,
    ) -> Size {
        let image_buf = Arc::as_ref(&self.image_buf);
        let size = druid::Size::new(image_buf.width() as f64, image_buf.height() as f64);
        bc.constrain(size)
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &AppState, _env: &Env) {
        let image_buf = Arc::as_ref(&self.image_buf);
        let rect = druid::Rect::new(
            0.0,
            0.0,
            image_buf.width() as f64,
            image_buf.height() as f64,
        );
        let image = image_buf.to_image(ctx.render_ctx);
        ctx.draw_image(&image, rect, druid::piet::InterpolationMode::Bilinear);

        let selected = data.tool_type == self.tool_type;
        if selected {
            ctx.stroke(rect, &theme::TOOLS_STROKE_SELECTED, 2.0);
        } else {
            ctx.stroke(rect, &theme::TOOLS_STROKE, 1.0);
        }
    }
}

pub struct Palette {
    current_idx: usize,
    values: [u32; 256],
}

impl Palette {
    pub fn new(bytes: &[u8]) -> Self {
        Self {
            current_idx: 0,
            values: Self::read_values(bytes),
        }
    }

    fn read_values(bytes: &[u8]) -> [u32; 256] {
        assert!(bytes.len() == 1024);

        let mut values: [u32; 256] = [0; 256];

        for i in 0..values.len() {
            let j = i * 4;
            let argb = [bytes[j + 0], bytes[j + 1], bytes[j + 2], bytes[j + 3]];
            values[i] = u32::from_le_bytes(argb);
        }

        values
    }

    fn point_to_xy(pos: druid::Point) -> Option<(usize, usize)> {
        if pos.x < 1.0 || pos.y < 1.0 {
            return None;
        }

        let x = pos.x as usize / (10 + 1) + 1;
        let y = pos.y as usize / (10 + 1) + 1;
        if x > 8 || y > 32 {
            return None;
        }

        Some((x, y))
    }

    fn xy_to_idx(x: usize, y: usize) -> usize {
        (y - 1) * 8 + (x - 1)
    }

    fn idx_to_point(idx: usize) -> druid::Point {
        let y = (idx / 8) as f64;
        let x = (idx % 8) as f64;
        druid::Point::new(1.0 + (x * (10.0 + 1.0)), 1.0 + (y * (10.0 + 1.0)))
    }

    fn idx_to_rect(idx: usize) -> druid::Rect {
        let origin = Self::idx_to_point(idx);
        druid::Rect::from_origin_size(origin, (10.0, 10.0))
    }

    fn paint_idx(ctx: &mut PaintCtx, idx: usize, value: u32, selected: bool) {
        if value & 0xff != 0 {
            let rect = Self::idx_to_rect(idx);
            let color = Color::from_rgba32_u32(value);
            ctx.fill(rect, &color);

            if selected {
                ctx.stroke(rect, &theme::PALETTE_STROKE_SELECTED, 2.0);
            }
        }
    }
}

impl Widget<AppState> for Palette {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut AppState, _env: &Env) {
        match event {
            Event::MouseDown(_e) => {
                ctx.set_active(true);
            }

            Event::MouseMove(e) => match Self::point_to_xy(e.pos) {
                Some(xy) => data.pos_color = self.values[Self::xy_to_idx(xy.0, xy.1)],
                None => data.pos_color = data.brush_color,
            },

            Event::MouseUp(e) if ctx.is_active() => {
                match Self::point_to_xy(e.pos) {
                    Some(xy) => {
                        self.current_idx = Self::xy_to_idx(xy.0, xy.1);
                        data.brush_color = self.values[self.current_idx];
                        ctx.request_paint();
                    }
                    None => {}
                }
                ctx.set_active(false);
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

    fn update(&mut self, _ctx: &mut UpdateCtx, _old_data: &AppState, _data: &AppState, _env: &Env) {
    }

    fn layout(
        &mut self,
        _layout_ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        _data: &AppState,
        _env: &Env,
    ) -> Size {
        let rect = Self::idx_to_rect(self.values.len() - 1);
        let size = Size::new(rect.x1 + 1.0, rect.y1 + 1.0);
        bc.constrain(size)
    }

    fn paint(&mut self, ctx: &mut PaintCtx, _data: &AppState, _env: &Env) {
        for i in 0..self.values.len() {
            let color = self.values[i];
            let selected = self.current_idx == i;
            Self::paint_idx(ctx, i, color, selected);
        }
    }
}

pub struct Canvas {}

impl Canvas {
    pub fn new() -> Self {
        Self {}
    }

    fn point_to_xy(pos: druid::Point) -> Option<(usize, usize)> {
        if pos.x < 1.0 || pos.y < 1.0 {
            return None;
        }

        let x = pos.x as usize / 16 + 1;
        let y = pos.y as usize / 16 + 1;
        if x > 32 || y > 32 {
            return None;
        }

        Some((x, y))
    }

    fn xy_to_idx(x: usize, y: usize) -> usize {
        (y - 1) * 32 + (x - 1)
    }

    fn idx_to_point(idx: usize) -> druid::Point {
        let y = (idx / 32) as f64;
        let x = (idx % 32) as f64;
        druid::Point::new(1.0 + (x * 16.0), 1.0 + (y * 16.0))
    }

    fn idx_to_rect(idx: usize) -> druid::Rect {
        let origin = Self::idx_to_point(idx);
        druid::Rect::from_origin_size(origin, (16.0, 16.0))
    }

    fn paint_idx(ctx: &mut PaintCtx, idx: usize, value: u32) {
        if value & 0xff != 0 {
            let rect = Self::idx_to_rect(idx);
            let color = Color::from_rgba32_u32(value);
            ctx.fill(rect, &color);
        }
    }

    fn paint_checkerboard(&mut self, ctx: &mut PaintCtx, _data: &AppState, _env: &Env) {
        let rect = ctx.size().to_rect();
        ctx.stroke(rect, &theme::CHECKERBOARD_STROKE, 1.0);

        let mut i = 0;
        for x in 0..32 {
            for y in 0..32 {
                let v = match (x + y) % 2 {
                    0 => theme::CHECKERBOARD_FILL_DARK,
                    _ => theme::CHECKERBOARD_FILL_LIGHT,
                };
                Self::paint_idx(ctx, i, v);
                i += 1;
            }
        }
    }

    fn tool(&mut self, data: &mut AppState, x: usize, y: usize) -> bool {
        let idx = Self::xy_to_idx(x, y);

        match data.tool_type {
            ToolType::Dropper => {
                data.brush_color = data.pixels[idx];
                false
            }

            ToolType::Eraser => {
                data.pixels[idx] = 0;
                true
            }

            ToolType::Paint => {
                data.pixels[idx] = data.brush_color;
                true
            }

            _ => false,
        }
    }
}

impl Widget<AppState> for Canvas {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut AppState, _env: &Env) {
        match event {
            Event::MouseDown(e) => {
                match Self::point_to_xy(e.pos) {
                    Some(xy) => {
                        if self.tool(data, xy.0, xy.1) {
                            ctx.request_paint();
                        }
                    }
                    _ => {}
                }
                ctx.set_active(true);
            }

            Event::MouseMove(e) => match Self::point_to_xy(e.pos) {
                Some(xy) => {
                    if ctx.is_active() {
                        self.tool(data, xy.0, xy.1);
                    }

                    let idx = Self::xy_to_idx(xy.0, xy.1);
                    data.pos = (xy.0, xy.1);
                    data.pos_color = data.pixels[idx];
                }
                None => {
                    data.pos = (0, 0);
                    data.pos_color = data.brush_color;
                }
            },

            Event::MouseUp(_e) if ctx.is_active() => {
                ctx.set_active(false);
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

    fn update(&mut self, _ctx: &mut UpdateCtx, _old_data: &AppState, _data: &AppState, _env: &Env) {
    }

    fn layout(
        &mut self,
        _layout_ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        data: &AppState,
        _env: &Env,
    ) -> Size {
        let rect = Self::idx_to_rect(data.pixels.len() - 1);
        let size = Size::new(rect.x1 + 1.0, rect.y1 + 1.0);
        bc.constrain(size)
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &AppState, env: &Env) {
        self.paint_checkerboard(ctx, data, env);

        for i in 0..data.pixels.len() {
            Self::paint_idx(ctx, i, data.pixels[i]);
        }
    }
}
