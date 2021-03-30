use druid::widget::prelude::*;

use crate::model::app_state::AppState;

/// A container one level up from the tool buttons. We use this to force a repaint of the
/// tool buttons when a new tool is selected.
pub struct ToolsController;

impl<W: Widget<AppState>> druid::widget::Controller<AppState, W> for ToolsController {
    fn update(
        &mut self,
        _child: &mut W,
        ctx: &mut UpdateCtx,
        old_data: &AppState,
        data: &AppState,
        _env: &Env,
    ) {
        if old_data.tool_type != data.tool_type {
            ctx.request_paint();
        }
    }
}
