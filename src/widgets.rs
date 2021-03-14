use std::collections::VecDeque;
use std::sync::Arc;

use druid::widget::prelude::*;

use crate::model::AppState;
use crate::model::ToolType;
use crate::theme;

/// A tool button that displays an icon. Could also have been implemented as a Painter or
/// Image, but without the styling for the selected state.
#[derive(Clone, druid::Data)]
pub struct ToolButton {
    tool_type: ToolType,
    image_buf: Arc<druid::ImageBuf>,
}

impl ToolButton {
    /// Create a new tool button of the given type and with the given raw bytes for the icon.
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
                        data.selection = None;
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

/// A palette that displays available colors. Each value is stored as a u32 representation
/// of RGBA, with the alpha value in the least significant position. This matches what Color
/// does internally.
pub struct Palette {
    current_idx: usize,
    values: [u32; 256],
}

impl Palette {
    /// Create a new palette with the given raw byte values.
    pub fn new(bytes: &[u8]) -> Self {
        Self {
            current_idx: 0,
            values: Self::read_values(bytes),
        }
    }

    /// Read given values into palette storage. The format is RGBA8, so four bytes
    /// per value including the alpha in the least significant position [a, r, g, b].
    /// For now we expect it to be 256 values exactly.
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

    /// Translate from screen coordinates (typically the mouse position) to palette coordinates.
    fn screen_coords_to_palette_coords(pos: druid::Point) -> Option<druid::Point> {
        if pos.x < 1.0 || pos.y < 1.0 {
            return None;
        }

        let x = pos.x as usize / (10 + 1) + 1;
        let y = pos.y as usize / (10 + 1) + 1;
        if x > 8 || y > 32 {
            return None;
        }

        Some(druid::Point::new(x as f64, y as f64))
    }

    /// Convert coordinates to an index within the palette storage.
    fn palette_coords_to_idx(p: druid::Point) -> usize {
        ((p.y - 1.0) * 8.0 + (p.x - 1.0)) as usize
    }

    /// Convert an index within the palette storage to screen coordinates.
    fn idx_to_screen_coords(idx: usize) -> druid::Point {
        let y = (idx / 8) as f64;
        let x = (idx % 8) as f64;
        druid::Point::new(1.0 + (x * (10.0 + 1.0)), 1.0 + (y * (10.0 + 1.0)))
    }

    /// Convert an index within the palette storage to a rectanble in screen coordinates.
    fn idx_to_screen_rect(idx: usize) -> druid::Rect {
        let origin = Self::idx_to_screen_coords(idx);
        druid::Rect::from_origin_size(origin, (10.0, 10.0))
    }

    /// Paint an index into palette storage into the given render context.
    fn paint_idx(ctx: &mut PaintCtx, idx: usize, value: u32, selected: bool) {
        if value & 0xff != 0 {
            let rect = Self::idx_to_screen_rect(idx);
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

            Event::MouseMove(e) => match Self::screen_coords_to_palette_coords(e.pos) {
                Some(p) => data.pos_color = self.values[Self::palette_coords_to_idx(p)],
                None => data.pos_color = data.brush_color,
            },

            Event::MouseUp(e) if ctx.is_active() => {
                match Self::screen_coords_to_palette_coords(e.pos) {
                    Some(p) => {
                        self.current_idx = Self::palette_coords_to_idx(p);
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
        let rect = Self::idx_to_screen_rect(self.values.len() - 1);
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

/// A canvas that allows for the display and modification of pixels. The size is currently
/// fixed at 32x32.
pub struct Canvas {
    ants_dark: druid::piet::StrokeStyle,
    ants_light: druid::piet::StrokeStyle,
}

impl Canvas {
    /// Create an empty canvas.
    pub fn new() -> Self {
        Self {
            ants_dark: druid::piet::StrokeStyle::new().dash(vec![4.0], 0.0),
            ants_light: druid::piet::StrokeStyle::new().dash(vec![4.0], 4.0),
        }
    }

    /// Translate from screen coordinates (typically the mouse position) to canvas coordinates.
    fn screen_coords_to_canvas_coords(pos: druid::Point) -> Option<druid::Point> {
        if pos.x < 1.0 || pos.y < 1.0 {
            return None;
        }

        let x = pos.x as usize / 16 + 1;
        let y = pos.y as usize / 16 + 1;
        if x > 32 || y > 32 {
            return None;
        }

        Some(druid::Point::new(x as f64, y as f64))
    }

    /// Translate from canvas coordinates to screen coordinates.
    fn canvas_coords_to_screen_coords(p: druid::Point) -> druid::Point {
        assert!(p.x > 0.0 && p.y > 0.0);
        druid::Point::new(1.0 + ((p.x - 1.0) * 16.0), 1.0 + ((p.y - 1.0) * 16.0))
    }

    /// Convert coordinates to an index within the canvas storage.
    fn canvas_coords_to_idx(p: druid::Point) -> usize {
        ((p.y - 1.0) * 32.0 + (p.x - 1.0)) as usize
    }

    /// Convert an index within the canvas storage to screen coordinates.
    fn idx_to_screen_coords(idx: usize) -> druid::Point {
        let y = (idx / 32) as f64;
        let x = (idx % 32) as f64;
        druid::Point::new(1.0 + (x * 16.0), 1.0 + (y * 16.0))
    }

    /// Convert an index within the canvas storage to a rectanble in screen coordinates.
    fn idx_to_screen_rect(idx: usize) -> druid::Rect {
        let origin = Self::idx_to_screen_coords(idx);
        druid::Rect::from_origin_size(origin, (16.0, 16.0))
    }

    /// Paint an index into canvas storage into the given render context.
    fn paint_idx(ctx: &mut PaintCtx, idx: usize, value: u32) {
        if value & 0xff != 0 {
            let rect = Self::idx_to_screen_rect(idx);
            let color = druid::Color::from_rgba32_u32(value);
            ctx.fill(rect, &color);
        }
    }

    /// Paint a checkerboard pattern for the background of the canvas into the given
    /// render context.
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

    /// Paint pixels from storage onto the given render context. This will paint
    /// on top of the checkboard. Pixel transparency is via alpha value.
    fn paint_pixels(&self, ctx: &mut PaintCtx, data: &AppState) {
        for i in 0..data.pixels.len() {
            Self::paint_idx(ctx, i, data.pixels[i]);
        }
    }

    /// Paint the currently selected area onto the given render context.
    fn paint_selection(&self, ctx: &mut PaintCtx, data: &AppState) {
        match data.selection {
            Some(s) => {
                let tl = Self::canvas_coords_to_screen_coords(druid::Point::new(
                    s.x0 as f64,
                    s.y0 as f64,
                ));
                let br = Self::canvas_coords_to_screen_coords(druid::Point::new(
                    s.x1 as f64,
                    s.y1 as f64,
                ));

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

            _ => {}
        }
    }

    /// Fill the canvas starting at the given point. Will pick a fill mode depending on
    /// whether there is a selection (marquee).
    fn fill(data: &mut AppState, p: druid::Point) -> bool {
        match data.selection {
            Some(selection) => Self::selection_fill(data, p, selection),
            _ => Self::flood_fill(data, p),
        }
    }

    /// Fill the canvas starting at the given point out to the edge of the current
    /// selection, while respecting color boundaries.
    fn selection_fill(data: &mut AppState, p: druid::Point, selection: druid::Rect) -> bool {
        if !selection.contains(p) {
            return false;
        }

        Self::flood_fill_work(data, p, selection)
    }

    /// Flood fill the canvas starting at the given point out to the edge of the
    /// canvas, while respecting color boundaries.
    fn flood_fill(data: &mut AppState, p: druid::Point) -> bool {
        Self::flood_fill_work(data, p, druid::Rect::new(1.0, 1.0, 32.0, 32.0))
    }

    /// Flood fill the canvas starting at the given point out to the given boundary,
    /// while respecting color boundaries. We should really change this to a span fill.
    fn flood_fill_work(data: &mut AppState, start_pos: druid::Point, bounds: druid::Rect) -> bool {
        let start_idx = Self::canvas_coords_to_idx(start_pos);
        let start_color = data.pixels[start_idx];
        if start_color == data.brush_color {
            return false;
        }

        let mut dirty = false;

        let mut q: VecDeque<druid::Point> = VecDeque::new();
        q.push_back(start_pos);
        while !q.is_empty() {
            let node = q.pop_front().unwrap();

            let idx = Self::canvas_coords_to_idx(node);
            if data.pixels[idx] == start_color {
                data.pixels[idx] = data.brush_color;

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

                dirty = true;
            }
        }

        dirty
    }

    /// Execute a tool at the given point on the canvas. The point is in
    /// canvas coordinates.
    fn tool(&mut self, data: &mut AppState, p: druid::Point) -> bool {
        let idx = Self::canvas_coords_to_idx(p);

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
                let x0 = data.start_pos.x.min(data.current_pos.x);
                let y0 = data.start_pos.y.min(data.current_pos.y);
                let x1 = data.start_pos.x.max(data.current_pos.x);
                let y1 = data.start_pos.y.max(data.current_pos.y);

                let new_selection = druid::Rect::new(x0, y0, x1, y1);

                let old_selection = data.selection.unwrap_or(druid::Rect::ZERO);

                if old_selection != new_selection {
                    data.selection = Some(new_selection);
                }

                old_selection != new_selection
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
                match Self::screen_coords_to_canvas_coords(e.pos) {
                    Some(p) => {
                        data.start_pos = p;

                        if self.tool(data, p) {
                            ctx.request_paint();
                        }
                    }
                    _ => data.start_pos = druid::Point::ZERO,
                }
                ctx.set_active(true);
            }

            Event::MouseMove(e) => match Self::screen_coords_to_canvas_coords(e.pos) {
                Some(p) => {
                    if ctx.is_active() {
                        self.tool(data, p);
                    }

                    let idx = Self::canvas_coords_to_idx(p);
                    data.current_pos = p;
                    data.pos_color = data.pixels[idx];
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

    fn update(&mut self, _ctx: &mut UpdateCtx, _old_data: &AppState, _data: &AppState, _env: &Env) {
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
        self.paint_checkerboard(ctx, data);
        self.paint_pixels(ctx, data);
        self.paint_selection(ctx, data);
    }
}
