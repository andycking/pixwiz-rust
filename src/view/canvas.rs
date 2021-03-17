use std::collections::VecDeque;

use druid::widget::prelude::*;

use crate::model::state::AppState;
use crate::model::types::ToolType;
use crate::view::theme;

/// A canvas that allows for the display and modification of pixels. The size is currently
/// fixed at 32x32.
pub struct Canvas {
    long_dash: [druid::piet::StrokeStyle; 2],
    short_dash: [druid::piet::StrokeStyle; 2],
}

impl Canvas {
    const COLS: usize = 32;
    const ROWS: usize = 32;
    const PIXELS: f64 = 16.0;

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

        let x = pos.x as usize / (Self::PIXELS as usize) + 1;
        let y = pos.y as usize / (Self::PIXELS as usize) + 1;
        if x > Self::COLS || y > Self::ROWS {
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
            1.0 + ((x - 1.0) * Self::PIXELS),
            1.0 + ((y - 1.0) * Self::PIXELS),
        )
    }

    /// Convert coordinates to an index within the canvas storage.
    fn canvas_coords_to_idx(p: druid::Point) -> usize {
        ((p.y - 1.0) * (Self::COLS as f64) + (p.x - 1.0)) as usize
    }

    /// Convert an index within the canvas storage to screen coordinates.
    fn idx_to_screen_coords(idx: usize) -> druid::Point {
        let y = (idx / Self::COLS) as f64;
        let x = (idx % Self::COLS) as f64;
        druid::Point::new(1.0 + (x * Self::PIXELS), 1.0 + (y * Self::PIXELS))
    }

    /// Convert an index within the canvas storage to a rectanble in screen coordinates.
    fn idx_to_screen_rect(idx: usize) -> druid::Rect {
        let origin = Self::idx_to_screen_coords(idx);
        druid::Rect::from_origin_size(origin, (Self::PIXELS, Self::PIXELS))
    }

    /// Paint an index into canvas storage into the given render context.
    fn paint_idx(ctx: &mut PaintCtx, idx: usize, value: u32) {
        let rect = Self::idx_to_screen_rect(idx);
        if value & 0xff != 0 {
            let color = druid::Color::from_rgba32_u32(value);
            ctx.fill(rect, &color);
        } else {
            let y = idx / Self::COLS;
            let x = idx % Self::ROWS;
            let color = match (x + y) % 2 {
                0 => theme::CANVAS_FILL_DARK,
                _ => theme::CANVAS_FILL_LIGHT,
            };
            ctx.fill(rect, &color);
        }
    }

    /// Paint border. The canvas does this internally instead of via border() because the
    /// pixels are already inset within the canvas (so that we can detect when the mouse
    /// leaves the area).
    fn paint_border(&self, ctx: &mut PaintCtx, _data: &AppState) {
        let rect = ctx.size().to_rect();
        let color = theme::CANVAS_STROKE;
        ctx.stroke(rect, &color, 1.0);
    }

    /// Paint pixels from storage onto the given render context. This will paint
    /// on top of the checkboard. Pixel transparency is via alpha value.
    fn paint_pixels(&self, ctx: &mut PaintCtx, data: &AppState) {
        for i in 0..data.pixels.len() {
            Self::paint_idx(ctx, i, data.pixels[i]);
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
    fn paint_grid(&self, ctx: &mut PaintCtx, _data: &AppState) {
        for i in 1..4 {
            let offset = 1 + i * 8;
            self.paint_grid_line(ctx, offset, 1, offset, Self::ROWS + 1);
            self.paint_grid_line(ctx, 1, offset, Self::COLS + 1, offset);
        }
    }

    /// Paint the currently selected area onto the given render context.
    fn paint_selection(&self, ctx: &mut PaintCtx, data: &AppState) {
        match data.selection {
            Some(s) => {
                let tl = Self::canvas_coords_to_screen_coords_f64(s.x0, s.y0);
                let br = Self::canvas_coords_to_screen_coords_f64(s.x1, s.y1);

                let rect = druid::Rect::new(tl.x, tl.y, br.x + 16.0, br.y + 16.0);

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

            _ => {}
        }
    }

    /// Fill the canvas starting at the given point. Will pick a fill mode depending on
    /// whether there is a selection (marquee).
    fn fill(data: &mut AppState, p: druid::Point) {
        match data.selection {
            Some(selection) => Self::selection_fill(data, p, selection),
            _ => Self::flood_fill(data, p),
        }
    }

    /// Fill the canvas starting at the given point out to the edge of the current
    /// selection, while respecting color boundaries.
    fn selection_fill(data: &mut AppState, p: druid::Point, selection: druid::Rect) {
        if selection.contains(p) {
            Self::flood_fill_work(data, p, selection)
        }
    }

    /// Flood fill the canvas starting at the given point out to the edge of the
    /// canvas, while respecting color boundaries.
    fn flood_fill(data: &mut AppState, p: druid::Point) {
        Self::flood_fill_work(data, p, druid::Rect::new(1.0, 1.0, 32.0, 32.0))
    }

    /// Flood fill the canvas starting at the given point out to the given boundary,
    /// while respecting color boundaries. We should really change this to a span fill.
    fn flood_fill_work(data: &mut AppState, start_pos: druid::Point, bounds: druid::Rect) {
        let start_idx = Self::canvas_coords_to_idx(start_pos);
        let start_color = data.pixels[start_idx];
        if start_color == data.brush_color {
            return;
        }

        let mut q: VecDeque<druid::Point> = VecDeque::new();
        q.push_back(start_pos);
        while !q.is_empty() {
            let node = q.pop_front().unwrap();

            let idx = Self::canvas_coords_to_idx(node);
            if data.pixels[idx] == start_color {
                data.pixels.write(idx, data.brush_color);

                if node.x > bounds.x0 as f64 {
                    q.push_back(druid::Point::new(node.x - 1.0, node.y));
                }
                if node.x < bounds.x1 as f64 {
                    q.push_back(druid::Point::new(node.x + 1.0, node.y));
                }
                if node.y > bounds.y0 as f64 {
                    q.push_back(druid::Point::new(node.x, node.y - 1.0));
                }
                if node.y < bounds.y1 as f64 {
                    q.push_back(druid::Point::new(node.x, node.y + 1.0));
                }
            }
        }
    }

    /// Execute a tool at the given point on the canvas. The point is in
    /// canvas coordinates.
    fn tool(&mut self, data: &mut AppState, p: druid::Point) {
        let idx = Self::canvas_coords_to_idx(p);

        match data.tool_type {
            ToolType::Dropper => {
                data.brush_color = data.pixels[idx];
            }

            ToolType::Eraser => {
                data.pixels.write(idx, 0);
            }

            ToolType::Fill => Self::fill(data, p),

            ToolType::Marquee => {
                let x0 = data.start_pos.x.min(data.current_pos.x);
                let y0 = data.start_pos.y.min(data.current_pos.y);
                let x1 = data.start_pos.x.max(data.current_pos.x);
                let y1 = data.start_pos.y.max(data.current_pos.y);

                let new_selection = druid::Rect::new(x0, y0, x1, y1);

                let old_selection = data.selection.unwrap_or(druid::Rect::ZERO);

                if old_selection != new_selection {
                    data.selection = Some(new_selection);
                }
            }

            ToolType::Paint => {
                data.pixels.write(idx, data.brush_color);
            }

            _ => {}
        }
    }
}

impl druid::Widget<AppState> for Canvas {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut AppState, _env: &Env) {
        match event {
            Event::MouseDown(e) => {
                if !e.focus {
                    match Self::screen_coords_to_canvas_coords(e.pos) {
                        Some(p) => {
                            data.start_pos = p;
                            self.tool(data, p);
                        }
                        _ => data.start_pos = druid::Point::ZERO,
                    }
                    ctx.set_active(true);
                }
            }

            Event::MouseMove(e) => match Self::screen_coords_to_canvas_coords(e.pos) {
                Some(p) => {
                    let idx = Self::canvas_coords_to_idx(p);
                    data.current_pos = p;
                    data.pos_color = data.pixels[idx];

                    if ctx.is_active() {
                        self.tool(data, p);
                    }
                }
                None => {
                    if !ctx.is_active() {
                        data.current_pos = druid::Point::ZERO;
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

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &AppState, data: &AppState, _env: &Env) {
        if !old_data.same(data) {
            ctx.request_paint();
        }
    }

    fn layout(
        &mut self,
        _layout_ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        data: &AppState,
        _env: &Env,
    ) -> Size {
        let rect = Self::idx_to_screen_rect(data.pixels.len() - 1);
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
