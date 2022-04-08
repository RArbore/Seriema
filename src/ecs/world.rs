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

use super::components::*;
use super::resources::*;
use super::systems::*;

use super::super::graphics::sprite::*;

#[derive(Copy, Clone)]
pub struct Entity {
    pub index: usize,
}

pub struct Components {
    pub positions: Vec<Option<Position>>,
    pub velocities: Vec<Option<Velocity>>,
}

pub struct Resources {
    pub timer: Timer,
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
            },
            size: 0,
            systems: Vec::new(),
            resources: Resources {
                timer: Timer::new(),
            },
        }
    }

    pub fn add(&mut self) -> Entity {
        self.components.positions.push(None);
        self.components.velocities.push(None);
        let entity = Entity { index: self.size };
        self.size += 1;
        entity
    }

    pub fn insert<T: Component>(&mut self, entity: Entity, component: T) {
        let vec = T::get_host_vec(&mut self.components);
        vec[entity.index] = Some(component);
    }

    pub fn run(&mut self) -> (RenderBatch, f32, f32) {
        self.resources.timer.update_dt();
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
            vec![
                vec![(0, 0.0, 0.0, 10.0, 10.0), (1, 0.0, 160.0, 10.0, 10.0)],
                vec![
                    (
                        0,
                        self.resources.timer.micros() as f32 / 10000.0,
                        0.0,
                        10.0,
                        10.0,
                    ),
                    (
                        0,
                        self.resources.timer.micros() as f32 / 10000.0,
                        160.0,
                        10.0,
                        10.0,
                    ),
                ],
            ],
            0.0,
            0.0,
        )
    }
}
