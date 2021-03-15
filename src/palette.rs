use druid::widget::prelude::*;

use crate::model::AppState;
use crate::theme;

/// A palette that displays available colors. Each value is stored as a u32 representation
/// of RGBA, with the alpha value in the least significant position. This matches what Color
/// does internally.
pub struct Palette {
    current_idx: usize,
    values: [u32; 256],
}

impl Palette {
    const COLS: usize = 32;
    const ROWS: usize = 8;
    const PIXELS: f64 = 15.0;

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

        let x = pos.x as usize / ((Self::PIXELS as usize) + 1) + 1;
        let y = pos.y as usize / ((Self::PIXELS as usize) + 1) + 1;
        if x > Self::COLS || y > Self::ROWS {
            return None;
        }

        Some(druid::Point::new(x as f64, y as f64))
    }

    /// Convert coordinates to an index within the palette storage.
    fn palette_coords_to_idx(p: druid::Point) -> usize {
        ((p.y - 1.0) * (Self::COLS as f64) + (p.x - 1.0)) as usize
    }

    /// Convert an index within the palette storage to screen coordinates.
    fn idx_to_screen_coords(idx: usize) -> druid::Point {
        let y = (idx / Self::COLS) as f64;
        let x = (idx % Self::COLS) as f64;
        druid::Point::new(
            1.0 + (x * (Self::PIXELS + 1.0)),
            1.0 + (y * (Self::PIXELS + 1.0)),
        )
    }

    /// Convert an index within the palette storage to a rectanble in screen coordinates.
    fn idx_to_screen_rect(idx: usize) -> druid::Rect {
        let origin = Self::idx_to_screen_coords(idx);
        druid::Rect::from_origin_size(origin, (Self::PIXELS, Self::PIXELS))
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