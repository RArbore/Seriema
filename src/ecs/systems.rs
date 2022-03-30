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

pub trait Query {
    fn matches(world: &mut World, entity: Entity) -> Option<Self>
    where
        Self: Sized;
}

impl<A: Component> Query for &mut A {
    fn matches(world: &mut World, entity: Entity) -> Option<Self> {
        let a = A::get_host_vec(world)[entity.index].as_mut()? as *mut A;
        unsafe { Some(&mut *a) }
    }
}

impl<A: Component, B: Component> Query for (&mut A, &mut B) {
    fn matches(world: &mut World, entity: Entity) -> Option<Self> {
        let a = A::get_host_vec(world)[entity.index].as_mut()? as *mut A;
        let b = B::get_host_vec(world)[entity.index].as_mut()? as *mut B;
        debug_assert_ne!(a as *mut (), b as *mut (), "2 components being queried came back as the same component. This is likely due to a query of 2 components of the same type; this is not allowed!");
        unsafe { Some((&mut *a, &mut *b)) }
    }
}

impl<A: Component, B: Component, C: Component> Query for (&mut A, &mut B, &mut C) {
    fn matches(world: &mut World, entity: Entity) -> Option<Self> {
        let a = A::get_host_vec(world)[entity.index].as_mut()? as *mut A;
        let b = B::get_host_vec(world)[entity.index].as_mut()? as *mut B;
        let c = C::get_host_vec(world)[entity.index].as_mut()? as *mut C;
        debug_assert_ne!(a as *mut (), b as *mut (), "2 components being queried came back as the same component. This is likely due to a query of 2 components of the same type; this is not allowed!");
        debug_assert_ne!(b as *mut (), c as *mut (), "2 components being queried came back as the same component. This is likely due to a query of 2 components of the same type; this is not allowed!");
        debug_assert_ne!(a as *mut (), c as *mut (), "2 components being queried came back as the same component. This is likely due to a query of 2 components of the same type; this is not allowed!");
        unsafe { Some((&mut *a, &mut *b, &mut *c)) }
    }
}

pub trait System {
    fn run(&'static self, world: &mut World, entity: Entity);
}

impl<Q: Query> System for fn(Q) {
    fn run(&'static self, world: &mut World, entity: Entity) {
        let matches_option = Q::matches(world, entity);
        if let Some(matches) = matches_option {
            self(matches);
        }
    }
}

pub fn print_position(position: &mut Position) {
    println!("print_position: {} {}", position.x, position.y);
}
