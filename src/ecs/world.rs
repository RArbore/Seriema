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

use std::collections::HashMap;

use super::components::*;
use super::resources::*;
use super::systems::*;
use super::tiles::*;

use super::super::graphics::controls::UserInput;
use super::super::graphics::sprite::{RenderBatch, NUM_TEXTURES};

#[derive(Copy, Clone)]
pub struct Entity {
    pub index: usize,
}

pub struct Components {
    pub positions: Vec<Option<Position>>,
    pub velocities: Vec<Option<Velocity>>,
    pub sprites: Vec<Option<Sprite>>,
    pub players: Vec<Option<Player>>,
}

pub struct Resources {
    pub timer: Timer,
    pub render_batch_res: RenderBatchRes,
    pub user_input: UserInput,
    pub camera: (f32, f32),
    pub control_point: (f32, f32),
    pub tiles: HashMap<(u32, u32), [Tile; CHUNK_SIZE * CHUNK_SIZE]>,
}

pub struct World {
    pub components: Components,
    pub size: usize,
    pub systems: Vec<Box<dyn System>>,
    pub resources: Resources,
}

impl World {
    pub fn new() -> Self {
        Self {
            components: Components {
                positions: Vec::new(),
                velocities: Vec::new(),
                sprites: Vec::new(),
                players: Vec::new(),
            },
            size: 0,
            systems: Vec::new(),
            resources: Resources {
                timer: Timer::new(),
                render_batch_res: RenderBatchRes::new(0 as *mut RenderBatch),
                user_input: UserInput::new(),
                camera: (0.0, 0.0),
                control_point: (0.0, 0.0),
            },
        }
    }

    pub fn add(&mut self) -> Entity {
        self.components.positions.push(None);
        self.components.velocities.push(None);
        self.components.sprites.push(None);
        self.components.players.push(None);
        let entity = Entity { index: self.size };
        self.size += 1;
        entity
    }

    pub fn insert<T: Component>(&mut self, entity: Entity, component: T) {
        let vec = T::get_host_vec(&mut self.components);
        vec[entity.index] = Some(component);
    }

    pub fn run(&mut self, input: UserInput) -> (RenderBatch, f32, f32, f32, f32) {
        self.resources.timer.update_dt();
        let mut render_batch: RenderBatch = vec![vec![]; NUM_TEXTURES];
        self.resources.render_batch_res = RenderBatchRes::new(&mut render_batch);
        self.resources.user_input = input;
        for system in self.systems.iter_mut() {
            for entity in 0..self.size {
                system.run(
                    &mut self.components,
                    Entity { index: entity },
                    &mut self.resources,
                );
            }
        }
        (
            render_batch,
            self.resources.camera.0,
            self.resources.camera.1,
            self.resources.control_point.0,
            self.resources.control_point.1,
        )
    }
}
