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

extern crate graphics;

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
    pub sprite_batch_res: SpriteBatchRes,
    pub user_input: graphics::GameInput,
    pub camera: (f32, f32),
    pub control_point: (f32, f32),
    pub tiles: graphics::Tiles,
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
                sprite_batch_res: SpriteBatchRes::new(0 as *mut graphics::SpriteBatch),
                user_input: graphics::GameInput::new(),
                camera: (0.0, 0.0),
                control_point: (0.0, 0.0),
                tiles: HashMap::new(),
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

    pub fn run(
        &mut self,
        input: graphics::GameInput,
    ) -> (
        graphics::SpriteBatch,
        graphics::TileBatch,
        f32,
        f32,
        f32,
        f32,
    ) {
        self.resources.timer.update_dt();
        let mut sprite_batch: graphics::SpriteBatch = vec![vec![]; graphics::NUM_SPRITES];
        self.resources.sprite_batch_res = SpriteBatchRes::new(&mut sprite_batch);
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

        let mut tile_batch: graphics::TileBatch = vec![vec![]; graphics::NUM_TILES];
        for (coords, data) in self.resources.tiles.iter() {
            for r in 0..graphics::CHUNK_SIZE {
                for c in 0..graphics::CHUNK_SIZE {
                    if data[r][c].0 != graphics::Tile::NoTile {
                        tile_batch[data[r][c].0 as usize].push((
                            data[r][c].1,
                            coords.0 * graphics::CHUNK_SIZE + r,
                            coords.1 * graphics::CHUNK_SIZE + c,
                        ));
                    }
                }
            }
        }

        (
            sprite_batch,
            tile_batch,
            self.resources.camera.0,
            self.resources.camera.1,
            self.resources.control_point.0,
            self.resources.control_point.1,
        )
    }
}
