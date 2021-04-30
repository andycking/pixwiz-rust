// Copyright 2018 The Druid Authors.
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

use super::theme;
use crate::model::app::AppState;

const LABEL_INSETS: druid::Insets = druid::Insets::uniform_xy(8., 2.);

/// A button with a text label.
pub struct Button {
    label: druid::widget::Label<AppState>,
    label_size: Size,
    is_default: bool,
}

impl Button {
    pub fn new(text: impl Into<druid::widget::LabelText<AppState>>, is_default: bool) -> Self {
        let label_color = if is_default {
            druid::Color::WHITE
        } else {
            druid::Color::BLACK
        };

        let label = druid::widget::Label::new(text)
            .with_font(druid::FontDescriptor::new(druid::FontFamily::SYSTEM_UI))
            .with_text_color(label_color);

        Self {
            label,
            label_size: Size::ZERO,
            is_default,
        }
    }

    pub fn on_click(
        self,
        f: impl Fn(&mut EventCtx, &mut AppState, &Env) + 'static,
    ) -> druid::widget::ControllerHost<Self, druid::widget::Click<AppState>> {
        druid::widget::ControllerHost::new(self, druid::widget::Click::new(f))
    }
}

impl druid::Widget<AppState> for Button {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, _data: &mut AppState, _env: &Env) {
        match event {
            Event::MouseDown(_) => {
                ctx.set_active(true);
                ctx.request_paint();
            }
            Event::MouseUp(_) => {
                if ctx.is_active() {
                    ctx.set_active(false);
                    ctx.request_paint();
                }
            }
            _ => (),
        }
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &AppState, env: &Env) {
        if let LifeCycle::HotChanged(_) = event {
            ctx.request_paint();
        }
        self.label.lifecycle(ctx, event, data, env)
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &AppState, data: &AppState, env: &Env) {
        self.label.update(ctx, old_data, data, env)
    }

    fn layout(
        &mut self,
        ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        data: &AppState,
        env: &Env,
    ) -> Size {
        let padding = Size::new(LABEL_INSETS.x_value(), LABEL_INSETS.y_value());
        let label_bc = bc.shrink(padding).loosen();
        self.label_size = self.label.layout(ctx, &label_bc, data, env);
        // HACK: to make sure we look okay at default sizes when beside a textbox,
        // we make sure we will have at least the same height as the default textbox.
        let min_height = env.get(druid::theme::BORDERED_WIDGET_HEIGHT);
        let baseline = self.label.baseline_offset();
        ctx.set_baseline_offset(baseline + LABEL_INSETS.y1);

        bc.constrain(Size::new(
            self.label_size.width + padding.width,
            (self.label_size.height + padding.height).max(min_height),
        ))
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &AppState, env: &Env) {
        let is_default = self.is_default;
        let is_active = ctx.is_active() && ctx.is_hot();
        let size = ctx.size();

        let rounded_rect = size
            .to_rect()
            .to_rounded_rect(env.get(druid::theme::BUTTON_BORDER_RADIUS));

        let bg_color = if is_active {
            if is_default {
                theme::BUTTON_DEFAULT_DARK
            } else {
                theme::BUTTON_DARK
            }
        } else if is_default {
            theme::BUTTON_DEFAULT_LIGHT
        } else {
            theme::BUTTON_LIGHT
        };

        ctx.fill(rounded_rect, &bg_color);

        let label_offset = (size.to_vec2() - self.label_size.to_vec2()) / 2.0;

        ctx.with_save(|ctx| {
            ctx.transform(druid::Affine::translate(label_offset));
            self.label.paint(ctx, data, env);
        });
    }
}
