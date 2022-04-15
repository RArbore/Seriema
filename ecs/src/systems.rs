/*
 * This file is part of game-testbed.
 * game-testbed is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * any later version.
 * game-testbed is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURAABBE.  See the
 * GNU General Public License for more details.
 * You should have received a copy of the GNU General Public License
 * along with game-testbed. If not, see <https://www.gnu.org/licenses/>.
 */

use super::components::*;
use super::resources::*;
use super::util::*;
use super::world::*;

extern crate graphics;

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
pub fn update_aabb(timer: &mut Timer, aabb: &mut AABB, vel: &mut Velocity) {
    aabb.x += vel.x * timer.dt();
    aabb.y += vel.y * timer.dt();
    match correct_collision(
        aabb,
        &mut AABB {
            x: 8.0,
            y: 8.0,
            w: 16.0,
            h: 16.0,
        },
    ) {
        Correction::None => {}
        Correction::Horizontal => {
            vel.x = 0.0;
        }
        Correction::Vertical => {
            vel.y = 0.0;
        }
    }
}

system_impl!(A, B, (sprite_batch_res, SpriteBatchRes));
pub fn render_sprite(sprite_batch: &mut SpriteBatchRes, aabb: &mut AABB, sprite: &mut Sprite) {
    sprite_batch.insert(
        sprite.sprite,
        sprite.frame,
        aabb.x,
        aabb.y,
        sprite.width,
        sprite.height,
    );
}

system_impl!(
    A,
    B,
    C,
    (timer, Timer),
    (user_input, graphics::GameInput),
    (camera, (f32, f32)),
    (control_point, (f32, f32))
);
pub fn player_system(
    timer: &mut Timer,
    user_input: &mut graphics::GameInput,
    camera: &mut (f32, f32),
    control_point: &mut (f32, f32),
    aabb: &mut AABB,
    vel: &mut Velocity,
    _player: &mut Player,
) {
    vel.x += user_input.n_cursor_x * 100.0 * timer.dt();
    vel.y += user_input.n_cursor_y * 100.0 * timer.dt();
    camera.0 = aabb.x;
    camera.1 = aabb.y;
    control_point.0 = aabb.x;
    control_point.1 = aabb.y;
}
