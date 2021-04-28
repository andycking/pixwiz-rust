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

pub fn enclosing_rect(a: druid::Point, b: druid::Point) -> druid::Rect {
    let x0 = a.x.min(b.x);
    let y0 = a.y.min(b.y);
    let x1 = a.x.max(b.x);
    let y1 = a.y.max(b.y);

    druid::Rect::new(x0, y0, x1, y1)
}

pub fn inflate_rect(rect: druid::Rect) -> druid::Rect {
    rect.with_size((rect.width() + 1.0, rect.height() + 1.0))
}

pub fn offset_rect(rect: druid::Rect, by: druid::Point) -> druid::Rect {
    druid::Rect::new(
        rect.x0 - by.x,
        rect.y0 - by.y,
        rect.x1 - by.x,
        rect.y1 - by.y,
    )
}

pub fn constrain_rect(rect: druid::Rect, bounds: druid::Rect) -> druid::Rect {
    let width = rect.width();
    let height = rect.height();

    let mut tl = (rect.x0, rect.y0);
    let mut br = (rect.x1, rect.y1);

    if tl.0 < bounds.x0 {
        tl.0 = bounds.x0;
        br.0 = 1.0 + width;
    }
    if tl.1 < bounds.y0 {
        tl.1 = bounds.y0;
        br.1 = 1.0 + height;
    }
    if br.0 > bounds.x1 {
        tl.0 = bounds.x1 - width;
        br.0 = bounds.x1;
    }
    if br.1 > bounds.y1 {
        tl.1 = bounds.y1 - height;
        br.1 = bounds.y1;
    }

    druid::Rect::from_points(tl, br)
}
