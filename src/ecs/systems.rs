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
use super::world::*;

pub trait Query {
    fn matches(components: &mut Components, entity: Entity) -> Option<Self>
    where
        Self: Sized;
}

macro_rules! query_tuple_impl {
    ($($x:ident),+) => {
        #[allow(unused_parens, non_snake_case)]
        impl<$($x: Component),*> Query for ($(&mut $x),*) {
            fn matches(components: &mut Components, entity: Entity) -> Option<Self> {
                $(
                    let $x = $x::get_host_vec(components)[entity.index].as_mut()? as *mut $x;
                )*
                    unsafe { Some(($(&mut *$x),*)) }
            }
        }
    };
}

query_tuple_impl!(A);
query_tuple_impl!(A, B);
query_tuple_impl!(A, B, C);

pub trait System {
    fn run(&self, components: &mut Components, entity: Entity, resources: &mut Resources);
}

macro_rules! system_impl {
    ($($x:ident),*) => {
        #[allow(unused_parens, non_snake_case)]
        impl<$($x: Component),*> System for fn(($(&mut $x),*)) {
            fn run(&self, components: &mut Components, entity: Entity, _resources: &mut Resources) {
                let matches_option = <($(&mut $x),*)>::matches(components, entity);
                if let Some(matches) = matches_option {
                    self(matches);
                }
            }
        }
    };
    ($($x:ident),*, $(($y:ident, $z: ty)),*) => {
        #[allow(unused_parens, non_snake_case)]
        impl<$($x: Component),*> System for fn(($(&mut $z),*), ($(&mut $x),*)) {
            fn run(&self, components: &mut Components, entity: Entity, resources: &mut Resources) {
                let matches_option = <($(&mut $x),*)>::matches(components, entity);
                if let Some(matches) = matches_option {
                    self($(&mut resources.$y),*, matches);
                }
            }
        }
    };
    ($(($y:ident, $z: ty)),*) => {
        #[allow(unused_parens, non_snake_case)]
        impl System for fn(($(&mut $z),*)) {
            fn run(&self, _components: &mut Components, _entity: Entity, resources: &mut Resources) {
                self($(&mut resources.$y),*);
            }
        }
    };
}

system_impl!((timer, Timer));
pub fn print_fps(timer: &mut Timer) {
    println!("FPS: {}", 1.0 / timer.dt());
}

system_impl!(A, B, (timer, Timer));
pub fn print_position_and_velocity(timer: &mut Timer, query: (&mut Position, &mut Velocity)) {
    println!(
        "print_position_and_velocity: {} {} {} {}",
        query.0.x, query.0.y, query.1.x, query.1.y,
    );
    query.0.x += timer.dt();
}
