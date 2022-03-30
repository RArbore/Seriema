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
use super::systems::*;

#[derive(Copy, Clone)]
pub struct Entity {
    pub index: usize,
}

pub struct World {
    pub positions: Vec<Option<Position>>,
    pub velocities: Vec<Option<Velocity>>,
    pub size: usize,
}

impl World {
    pub fn new() -> Self {
        Self {
            positions: Vec::new(),
            velocities: Vec::new(),
            size: 0,
        }
    }

    pub fn add(&mut self) -> Entity {
        self.positions.push(None);
        self.velocities.push(None);
        let entity = Entity { index: self.size };
        self.size += 1;
        entity
    }

    pub fn insert<T: Component>(&mut self, entity: Entity, component: T) {
        let vec = T::get_host_vec(self);
        vec[entity.index] = Some(component);
    }

    pub fn at(&self, entity: Entity) -> Vec<&dyn Component> {
        let mut vec: Vec<&dyn Component> = vec![];
        add_if(self.positions[entity.index].as_ref(), &mut vec);
        vec
    }
}

fn add_if<'a, T: Component>(option: Option<&'a T>, vec: &mut Vec<&'a dyn Component>) {
    if let Some(r) = option {
        vec.push(r);
    }
}
