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
use super::world::*;

pub trait Query<'a> {
    fn matches(world: &'a mut World, entity: Entity) -> Option<Self>
    where
        Self: Sized;
}

impl<'a, A: Component> Query<'a> for &'a mut A {
    fn matches(world: &'a mut World, entity: Entity) -> Option<Self> {
        A::get_host_vec(world)[entity.index].as_mut()
    }
}
