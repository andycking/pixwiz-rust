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
        data.window_pos = ctx.window().get_position();

        if let Event::WindowConnected = event {
            ctx.request_focus();
        }

        child.event(ctx, event, data, env);
    }
}
