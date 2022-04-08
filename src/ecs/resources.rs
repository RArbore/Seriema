/*
 * This file is part of game-testbed.
 * game-testbed is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * any later version.
 * game-testbed is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 * You should have received a copy of the GNU General Public License
 * along with game-testbed. If not, see <https://www.gnu.org/licenses/>.
 */

use std::time::Instant;

use super::super::graphics::sprite::*;

pub struct Timer {
    start: Instant,
    stopwatch: u32,
    dt: f32,
}

impl Timer {
    pub fn new() -> Self {
        Self {
            start: Instant::now(),
            stopwatch: 0,
            dt: 0.0,
        }
    }

    pub fn update_dt(&mut self) {
        self.dt = ((self.micros() - self.stopwatch) as f32) / 1000000.0;
        self.stopwatch = self.micros();
    }

    pub fn dt(&self) -> f32 {
        self.dt
    }

    pub fn micros(&self) -> u32 {
        self.start.elapsed().as_micros() as u32
    }
}

pub struct RenderBatchRes {
    pub render_batch: *mut RenderBatch,
}

impl RenderBatchRes {
    pub fn new(render_batch: *mut RenderBatch) -> Self {
        RenderBatchRes { render_batch }
    }

    pub fn insert(&mut self, sprite: Sprite, frame: usize, x: f32, y: f32, w: f32, h: f32) {
        unsafe {
            (*self.render_batch)[sprite as usize].push((frame, x, y, w, h));
        }
    }
}
