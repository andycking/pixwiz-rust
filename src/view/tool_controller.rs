use druid::widget::prelude::*;

use crate::model::app_state::AppState;

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
