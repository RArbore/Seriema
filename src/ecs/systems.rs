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
    fn matches(components: &mut Components, entity: Entity) -> Option<Self>
    where
        Self: Sized;
}

impl<A: Component> Query for &mut A {
    fn matches(components: &mut Components, entity: Entity) -> Option<Self> {
        let a = A::get_host_vec(components)[entity.index].as_mut()? as *mut A;
        unsafe { Some(&mut *a) }
    }
}

impl<A: Component, B: Component> Query for (&mut A, &mut B) {
    fn matches(components: &mut Components, entity: Entity) -> Option<Self> {
        let a = A::get_host_vec(components)[entity.index].as_mut()? as *mut A;
        let b = B::get_host_vec(components)[entity.index].as_mut()? as *mut B;
        debug_assert_ne!(a as *mut (), b as *mut (), "2 components being queried came back as the same component. This is likely due to a query of 2 components of the same type; this is not allowed!");
        unsafe { Some((&mut *a, &mut *b)) }
    }
}

impl<A: Component, B: Component, C: Component> Query for (&mut A, &mut B, &mut C) {
    fn matches(components: &mut Components, entity: Entity) -> Option<Self> {
        let a = A::get_host_vec(components)[entity.index].as_mut()? as *mut A;
        let b = B::get_host_vec(components)[entity.index].as_mut()? as *mut B;
        let c = C::get_host_vec(components)[entity.index].as_mut()? as *mut C;
        debug_assert_ne!(a as *mut (), b as *mut (), "2 components being queried came back as the same component. This is likely due to a query of 2 components of the same type; this is not allowed!");
        debug_assert_ne!(b as *mut (), c as *mut (), "2 components being queried came back as the same component. This is likely due to a query of 2 components of the same type; this is not allowed!");
        debug_assert_ne!(a as *mut (), c as *mut (), "2 components being queried came back as the same component. This is likely due to a query of 2 components of the same type; this is not allowed!");
        unsafe { Some((&mut *a, &mut *b, &mut *c)) }
    }
}

pub trait System {
    fn run(&self, components: &mut Components, entity: Entity);
}

impl<A: Component> System for for<'a> fn(&'a mut A) {
    fn run(&self, components: &mut Components, entity: Entity) {
        let matches_option = <&mut A>::matches(components, entity);
        if let Some(matches) = matches_option {
            self(matches);
        }
    }
}

impl<A: Component, B: Component> System for for<'a, 'b> fn((&'a mut A, &'b mut B)) {
    fn run(&self, components: &mut Components, entity: Entity) {
        let matches_option = <(&mut A, &mut B)>::matches(components, entity);
        if let Some(matches) = matches_option {
            self(matches);
        }
    }
}

impl<A: Component, B: Component, C: Component> System
    for for<'a, 'b, 'c> fn((&'a mut A, &'b mut B, &'c mut C))
{
    fn run(&self, components: &mut Components, entity: Entity) {
        let matches_option = <(&mut A, &mut B, &mut C)>::matches(components, entity);
        if let Some(matches) = matches_option {
            self(matches);
        }
    }
}

pub fn print_position_and_velocity(query: (&mut Position, &mut Velocity)) {
    println!(
        "print_position_and_velocity: {} {} {} {}",
        query.0.x, query.0.y, query.1.x, query.1.y
    );
}
