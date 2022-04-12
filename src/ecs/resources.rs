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
    second_border: bool,
}

impl Timer {
    pub fn new() -> Self {
        Self {
            start: Instant::now(),
            stopwatch: 0,
            dt: 0.0,
            second_border: false,
        }
    }

    pub fn update_dt(&mut self) {
        self.dt = ((self.micros() - self.stopwatch) as f32) / 1000000.0;
        let before = self.stopwatch;
        self.stopwatch = self.micros();
        self.second_border = (self.stopwatch % 1000000) < (before % 1000000);
    }

    pub fn dt(&self) -> f32 {
        self.dt
    }

    pub fn micros(&self) -> u32 {
        self.start.elapsed().as_micros() as u32
    }

    pub fn second_border(&mut self) -> bool {
        let second_border = self.second_border;
        self.second_border = false;
        second_border
    }
}

pub struct SpriteBatchRes {
    pub sprite_batch: *mut SpriteBatch,
}

impl SpriteBatchRes {
    pub fn new(sprite_batch: *mut SpriteBatch) -> Self {
        SpriteBatchRes { sprite_batch }
    }

    pub fn insert(&mut self, sprite: Sprite, frame: usize, x: f32, y: f32, w: f32, h: f32) {
        unsafe {
            (*self.sprite_batch)[sprite as usize].push((frame, x, y, w, h));
        }
    }
}
