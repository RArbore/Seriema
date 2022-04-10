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

use super::super::graphics::controls::UserInput;

pub trait System {
    fn run(
        &self,
        components: &mut Components,
        entity: Entity,
        resources: &mut Resources,
    ) -> Option<()>;
}

macro_rules! system_impl {
    ($($x:ident),*) => {
        #[allow(unused_parens, non_snake_case)]
        impl<$($x: Component),*> System for fn($(&mut $x),*) {
            fn run(&self, components: &mut Components, entity: Entity, _resources: &mut Resources) -> Option<()> {
                $(
                    let $x = <$x>::matches(components, entity)?;
                )*
                    self($(&mut $x),*);
            }
        }
    };
    ($($x:ident),*, $(($y:ident, $z: ty)),*) => {
        #[allow(unused_parens, non_snake_case)]
        impl<$($x: Component),*> System for fn($(&mut $z),*, $(&mut $x),*) {
            fn run(&self, components: &mut Components, entity: Entity, resources: &mut Resources) -> Option<()> {
                $(
                    let $x = <($x)>::get_host_vec(components)[entity.index].as_mut()? as *mut $x;
                )*
                    self($(&mut resources.$y),*,$(unsafe { &mut *$x }),*);
                Some(())
            }
        }
    };
    ($(($y:ident, $z: ty)),*) => {
        #[allow(unused_parens, non_snake_case)]
        impl System for fn(($(&mut $z),*)) {
            fn run(&self, _components: &mut Components, _entity: Entity, resources: &mut Resources) -> Option<()> {
                self($(&mut resources.$y),*);
                Some(())
            }
        }
    };
}

system_impl!((timer, Timer));
pub fn print_fps(timer: &mut Timer) {
    if timer.second_border() {
        println!("FPS: {}", 1.0 / timer.dt());
    }
}

system_impl!(A, B, (timer, Timer));
pub fn update_pos(timer: &mut Timer, pos: &mut Position, vel: &mut Velocity) {
    pos.x += vel.x * timer.dt();
    pos.y += vel.y * timer.dt();
}

system_impl!(A, B, (render_batch_res, RenderBatchRes));
pub fn render_sprite(render_batch: &mut RenderBatchRes, pos: &mut Position, sprite: &mut Sprite) {
    render_batch.insert(
        sprite.sprite,
        sprite.frame,
        pos.x,
        pos.y,
        sprite.width,
        sprite.height,
    );
}

system_impl!(A, B, (timer, Timer), (user_input, UserInput));
pub fn player_system(
    timer: &mut Timer,
    user_input: &mut UserInput,
    vel: &mut Velocity,
    _player: &mut Player,
) {
    vel.x += user_input.n_cursor_x * 100.0 * timer.dt();
    vel.y += user_input.n_cursor_y * 100.0 * timer.dt();
}
