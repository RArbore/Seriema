/*
 * This file is part of game-testbed.
 * game-testbed is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * any later version.
 * game-testbed is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURAABBE.  See the
 * GNU General Public License for more details.
 * You should have received a copy of the GNU General Public License
 * along with game-testbed. If not, see <https://www.gnu.org/licenses/>.
 */

use serde::{Deserialize, Serialize};

use super::components::*;
use super::world::*;

extern crate graphics;

#[derive(Clone, Copy, Deserialize, Serialize)]
pub enum EntityDesc {
    Player(PlayerDesc),
}

impl EntityDesc {
    pub fn get_sprite(&self) -> graphics::Sprite {
        match self {
            EntityDesc::Player(x) => x.get_sprite(),
        }
    }

    pub fn construct(&self, world: &mut World) {
        match self {
            EntityDesc::Player(x) => x.construct(world),
        }
    }

    pub fn get_pos(&self) -> (f32, f32) {
        match self {
            EntityDesc::Player(x) => x.get_pos(),
        }
    }

    pub fn adjust_pos(&mut self, dx: f32, dy: f32) {
        match self {
            EntityDesc::Player(x) => x.adjust_pos(dx, dy),
        }
    }
}

#[derive(Clone, Copy, Deserialize, Serialize)]
pub struct PlayerDesc {
    pub x: f32,
    pub y: f32,
}

impl PlayerDesc {
    fn get_sprite(&self) -> graphics::Sprite {
        graphics::sprite::Sprite::TestSprite1
    }

    fn construct(&self, world: &mut World) {
        let entity = world.add();
        world.insert(
            entity,
            AABB {
                x: self.x,
                y: self.y,
                w: 15.0,
                h: 15.0,
                last: 0,
            },
        );
        world.insert(entity, Velocity { x: 0.0, y: 0.0 });
        world.insert(
            entity,
            Sprite {
                sprite: self.get_sprite(),
                frame: 0,
                width: 1.0,
                height: 1.0,
                off_x: 0.0,
                off_y: 0.5,
            },
        );
        world.insert(entity, Player { can_jump: 0.0 });
    }

    pub fn get_pos(&self) -> (f32, f32) {
        (self.x, self.y)
    }

    fn adjust_pos(&mut self, dx: f32, dy: f32) {
        self.x += dx;
        self.y += dy;
    }
}
