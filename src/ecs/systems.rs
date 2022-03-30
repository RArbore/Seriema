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

pub trait Query<'z> {
    fn matches(world: &'z mut World, entity: Entity) -> Option<Self>
    where
        Self: Sized;
}

impl<'z, A: Component> Query<'z> for &'z mut A {
    fn matches(world: &'z mut World, entity: Entity) -> Option<Self> {
        A::get_host_vec(world)[entity.index].as_mut()
    }
}

impl<'z, A: Component, B: Component> Query<'z> for (&'z mut A, &'z mut B) {
    fn matches(world: &'z mut World, entity: Entity) -> Option<Self> {
        let a = A::get_host_vec(world)[entity.index].as_mut()? as *mut A;
        let b = B::get_host_vec(world)[entity.index].as_mut()? as *mut B;
        debug_assert_ne!(a as *mut (), b as *mut (), "2 components being queried came back as the same component. This is likely due to a query of 2 components of the same type; this is not allowed!");
        unsafe { Some((&mut *a, &mut *b)) }
    }
}

impl<'z, A: Component, B: Component, C: Component> Query<'z> for (&'z mut A, &'z mut B, &'z mut C) {
    fn matches(world: &'z mut World, entity: Entity) -> Option<Self> {
        let a = A::get_host_vec(world)[entity.index].as_mut()? as *mut A;
        let b = B::get_host_vec(world)[entity.index].as_mut()? as *mut B;
        let c = C::get_host_vec(world)[entity.index].as_mut()? as *mut C;
        debug_assert_ne!(a as *mut (), b as *mut (), "2 components being queried came back as the same component. This is likely due to a query of 2 components of the same type; this is not allowed!");
        debug_assert_ne!(b as *mut (), c as *mut (), "2 components being queried came back as the same component. This is likely due to a query of 2 components of the same type; this is not allowed!");
        debug_assert_ne!(a as *mut (), c as *mut (), "2 components being queried came back as the same component. This is likely due to a query of 2 components of the same type; this is not allowed!");
        unsafe { Some((&mut *a, &mut *b, &mut *c)) }
    }
}
