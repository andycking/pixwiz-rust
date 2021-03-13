use std::cmp;
use std::collections::VecDeque;
use std::sync::Arc;

use druid::widget::prelude::*;

use crate::model::AppState;
use crate::model::Point;
use crate::model::Rect;
use crate::model::ToolType;
use crate::theme;

#[derive(Clone, druid::Data)]
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

impl druid::Widget<AppState> for ToolButton {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut AppState, _env: &Env) {
        match event {
            Event::MouseDown(_e) => {
                ctx.set_active(true);
            }

            Event::MouseUp(_e) if ctx.is_active() => {
                if ctx.is_hot() {
                    if self.tool_type == ToolType::Marquee {
                        data.selection = ((0, 0), (0, 0));
                        ctx.request_paint();
                    }
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

    fn druid_point_to_p(pos: druid::Point) -> Option<Point<usize>> {
        if pos.x < 1.0 || pos.y < 1.0 {
            return None;
        }

        let x = pos.x as usize / (10 + 1) + 1;
        let y = pos.y as usize / (10 + 1) + 1;
        if x > 8 || y > 32 {
            return None;
        }

        Some(Point::new(x, y))
    }

    fn p_to_idx(p: Point<usize>) -> usize {
        (p.y - 1) * 8 + (p.x - 1)
    }

    fn idx_to_druid_point(idx: usize) -> druid::Point {
        let y = (idx / 8) as f64;
        let x = (idx % 8) as f64;
        druid::Point::new(1.0 + (x * (10.0 + 1.0)), 1.0 + (y * (10.0 + 1.0)))
    }

    fn idx_to_druid_rect(idx: usize) -> druid::Rect {
        let origin = Self::idx_to_druid_point(idx);
        druid::Rect::from_origin_size(origin, (10.0, 10.0))
    }

    fn paint_idx(ctx: &mut PaintCtx, idx: usize, value: u32, selected: bool) {
        if value & 0xff != 0 {
            let rect = Self::idx_to_druid_rect(idx);
            let color = druid::Color::from_rgba32_u32(value);
            ctx.fill(rect, &color);

            if selected {
                ctx.stroke(rect, &theme::PALETTE_STROKE_SELECTED, 2.0);
            }
        }
    }
}

impl druid::Widget<AppState> for Palette {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut AppState, _env: &Env) {
        match event {
            Event::MouseDown(_e) => {
                ctx.set_active(true);
            }

            Event::MouseMove(e) => match Self::druid_point_to_p(e.pos) {
                Some(p) => data.pos_color = self.values[Self::p_to_idx(p)],
                None => data.pos_color = data.brush_color,
            },

            Event::MouseUp(e) if ctx.is_active() => {
                match Self::druid_point_to_p(e.pos) {
                    Some(p) => {
                        self.current_idx = Self::p_to_idx(p);
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
        let rect = Self::idx_to_druid_rect(self.values.len() - 1);
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

pub struct Canvas {
    ants_dark: druid::piet::StrokeStyle,
    ants_light: druid::piet::StrokeStyle,
}

impl Canvas {
    pub fn new() -> Self {
        Self {
            ants_dark: druid::piet::StrokeStyle::new().dash(vec![4.0], 0.0),
            ants_light: druid::piet::StrokeStyle::new().dash(vec![4.0], 4.0),
        }
    }

    fn druid_point_to_p(pos: druid::Point) -> Option<Point<usize>> {
        if pos.x < 1.0 || pos.y < 1.0 {
            return None;
        }

        let x = pos.x as usize / 16 + 1;
        let y = pos.y as usize / 16 + 1;
        if x > 32 || y > 32 {
            return None;
        }

        Some(Point::new(x, y))
    }

    fn p_to_idx(p: Point<usize>) -> usize {
        (p.y - 1) * 32 + (p.x - 1)
    }

    fn p_to_druid_point(p: Point<usize>) -> druid::Point {
        assert!(p.x > 0 && p.y > 0);
        druid::Point::new(1.0 + ((p.x - 1) as f64 * 16.0), 1.0 + ((p.y - 1) as f64 * 16.0))
    }

    fn idx_to_druid_point(idx: usize) -> druid::Point {
        let y = (idx / 32) as f64;
        let x = (idx % 32) as f64;
        druid::Point::new(1.0 + (x * 16.0), 1.0 + (y * 16.0))
    }

    fn idx_to_druid_rect(idx: usize) -> druid::Rect {
        let origin = Self::idx_to_druid_point(idx);
        druid::Rect::from_origin_size(origin, (16.0, 16.0))
    }

    fn paint_idx(ctx: &mut PaintCtx, idx: usize, value: u32) {
        if value & 0xff != 0 {
            let rect = Self::idx_to_druid_rect(idx);
            let color = druid::Color::from_rgba32_u32(value);
            ctx.fill(rect, &color);
        }
    }

    fn paint_checkerboard(&self, ctx: &mut PaintCtx, _data: &AppState) {
        let rect = ctx.size().to_rect();
        ctx.stroke(rect, &theme::CANVAS_STROKE, 1.0);

        let dark = theme::CANVAS_FILL_DARK.as_rgba_u32();
        let light = theme::CANVAS_FILL_LIGHT.as_rgba_u32();

        let mut i = 0;
        for x in 0..32 {
            for y in 0..32 {
                let v = match (x + y) % 2 {
                    0 => dark,
                    _ => light,
                };
                Self::paint_idx(ctx, i, v);
                i += 1;
            }
        }
    }

    fn paint_pixels(&self, ctx: &mut PaintCtx, data: &AppState) {
        for i in 0..data.pixels.len() {
            Self::paint_idx(ctx, i, data.pixels[i]);
        }
    }

    fn paint_selection(&self, ctx: &mut PaintCtx, data: &AppState) {
        if data.has_selection() {
            let s = data.selection;

            let tl = Self::p_to_druid_point(Point::from(s.0));
            let br = Self::p_to_druid_point(Point::from(s.1));

            let rect = druid::Rect::new(tl.x, tl.y, br.x + 16.0, br.y + 16.0);

            ctx.stroke_styled(
                rect,
                &theme::CANVAS_STROKE_SELECTED_DARK,
                2.0,
                &self.ants_dark,
            );
            ctx.stroke_styled(
                rect,
                &theme::CANVAS_STROKE_SELECTED_LIGHT,
                2.0,
                &self.ants_light,
            );
        }
    }

    fn fill(data: &mut AppState, p: Point<usize>) -> bool {
        if data.has_selection() {
            Self::selection_fill(data, p)
        } else {
            Self::flood_fill(data, p)
        }
    }

    fn selection_fill(data: &mut AppState, p: Point<usize>) -> bool {
        if p.x < data.selection.0 .0
            || p.x > data.selection.1 .0
            || p.y < data.selection.0 .1
            || p.y > data.selection.1 .1
        {
            return false;
        }

        Self::flood_fill_work(data, p, Rect::from(data.selection))
    }

    fn flood_fill(data: &mut AppState, p: Point<usize>) -> bool {
        Self::flood_fill_work(data, p, Rect::new(1, 1, 32, 32))
    }

    fn flood_fill_work(data: &mut AppState, start_pos: Point<usize>, bounds: Rect<usize>) -> bool {
        let start_idx = Self::p_to_idx(start_pos);
        let start_color = data.pixels[start_idx];
        if start_color == data.brush_color {
            return false;
        }

        let mut dirty = false;

        let mut q: VecDeque<Point<usize>> = VecDeque::new();
        q.push_back(start_pos);
        while !q.is_empty() {
            let node = q.pop_front().unwrap();

            let idx = Self::p_to_idx(node);
            if data.pixels[idx] == start_color {
                data.pixels[idx] = data.brush_color;

                if node.x > bounds.x0 {
                    q.push_back(Point::new(node.x - 1, node.y));
                }
                if node.x < bounds.x1 {
                    q.push_back(Point::new(node.x + 1, node.y));
                }
                if node.y > bounds.y0 {
                    q.push_back(Point::new(node.x, node.y - 1));
                }
                if node.y < bounds.y1 {
                    q.push_back(Point::new(node.x, node.y + 1));
                }

                dirty = true;
            }
        }

        dirty
    }

    fn tool(&mut self, data: &mut AppState, p: Point<usize>) -> bool {
        let idx = Self::p_to_idx(p);

        match data.tool_type {
            ToolType::Dropper => {
                data.brush_color = data.pixels[idx];
                false
            }

            ToolType::Eraser => {
                data.pixels[idx] = 0;
                true
            }

            ToolType::Fill => Self::fill(data, p),

            ToolType::Marquee => {
                let tl = (
                    cmp::min(data.start_pos.x, data.current_pos.x),
                    cmp::min(data.start_pos.y, data.current_pos.y),
                );

                let br = (
                    cmp::max(data.start_pos.x, data.current_pos.x),
                    cmp::max(data.start_pos.y, data.current_pos.y),
                );

                let s = (tl, br);

                if s != data.selection {
                    data.selection = s;
                }

                s != data.selection
            }

            ToolType::Paint => {
                data.pixels[idx] = data.brush_color;
                true
            }

            _ => false,
        }
    }
}

impl druid::Widget<AppState> for Canvas {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut AppState, _env: &Env) {
        match event {
            Event::MouseDown(e) => {
                match Self::druid_point_to_p(e.pos) {
                    Some(p) => {
                        data.start_pos = p;

                        if self.tool(data, p) {
                            ctx.request_paint();
                        }
                    }
                    _ => data.start_pos = Point::empty(),
                }
                ctx.set_active(true);
            }

            Event::MouseMove(e) => match Self::druid_point_to_p(e.pos) {
                Some(p) => {
                    if ctx.is_active() {
                        self.tool(data, p);
                    }

                    let idx = Self::p_to_idx(p);
                    data.current_pos = p;
                    data.pos_color = data.pixels[idx];
                }
                None => {
                    if !ctx.is_active() {
                        data.current_pos = Point::empty();
                        data.pos_color = data.brush_color;
                    }
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
        let rect = Self::idx_to_druid_rect(data.pixels.len() - 1);
        let size = Size::new(rect.x1 + 1.0, rect.y1 + 1.0);
        bc.constrain(size)
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &AppState, _env: &Env) {
        self.paint_checkerboard(ctx, data);
        self.paint_pixels(ctx, data);
        self.paint_selection(ctx, data);
    }
}
