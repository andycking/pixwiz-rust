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
