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

use std::sync::Arc;

use druid::widget::prelude::*;

use crate::model::app::AppState;
use crate::model::types::ToolType;
use crate::view::theme;

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
            tool_type,
            image_buf: Arc::new(image_buf),
        }
    }
}

impl druid::Widget<AppState> for ToolButton {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut AppState, _env: &Env) {
        match event {
            Event::MouseDown(e) => {
                if !e.focus {
                    ctx.set_active(true);
                }
            }

            Event::MouseUp(_e) if ctx.is_active() => {
                if ctx.is_hot() {
                    data.tool_type = self.tool_type;

                    // Don't forget to clear out the move bytes. There has to be a better
                    // place to put this.
                    if data.tool_type != ToolType::Move && data.doc.move_bytes.is_some() {
                        data.doc.move_bytes = None;
                    }
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

/// A container one level up from the tool buttons. We use this to force a repaint of the
/// tool buttons when a new tool is selected.
pub struct ToolsController;

impl<W: Widget<AppState>> druid::widget::Controller<AppState, W> for ToolsController {
    fn update(
        &mut self,
        child: &mut W,
        ctx: &mut UpdateCtx,
        old_data: &AppState,
        data: &AppState,
        env: &Env,
    ) {
        if old_data.tool_type != data.tool_type {
            ctx.request_paint();
        }

        child.update(ctx, old_data, data, env);
    }
}
