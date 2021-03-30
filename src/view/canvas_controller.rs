use druid::widget::prelude::*;

use crate::model::app_state::AppState;

pub struct CanvasController;

/// A controller one level up from the canvas. We use this to steal the focus when the
/// app starts, so that key events go to the canvas.
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
