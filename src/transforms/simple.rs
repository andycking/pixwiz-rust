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

use crate::model::pixel_env::PixelEnv;
use crate::model::pixel_header::PixelHeader;

pub fn clear(header: &PixelHeader, env: &PixelEnv, bytes: &mut Vec<u8>) {
    for y in env.bounds.y0 as usize..env.bounds.y1 as usize {
        for x in env.bounds.x0 as usize..env.bounds.x1 as usize {
            super::write(x, y, header, bytes, &druid::Color::rgba(0.0, 0.0, 0.0, 0.0));
        }
    }
}
