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

system_impl!(A, B, (timer, Timer), (tiles, graphics::Tiles));
pub fn update_aabb(
    timer: &mut Timer,
    tiles: &mut graphics::Tiles,
    aabb: &mut AABB,
    vel: &mut Velocity,
) {
    aabb.x += vel.x * timer.dt();
    aabb.y += vel.y * timer.dt();
    aabb.last = Correction::None;
    let tiles_to_check = get_all_tiles_in_aabb(aabb, tiles);
    for (tile_id, ux, uy) in tiles_to_check {
        if tile_id == graphics::Tile::NoTile {
            continue;
        }
        let correction = correct_collision(
            aabb,
            &mut AABB {
                x: (ux * graphics::TILE_SIZE + graphics::TILE_SIZE / 2) as f32,
                y: (uy * graphics::TILE_SIZE + graphics::TILE_SIZE / 2) as f32,
                w: graphics::TILE_SIZE as f32,
                h: graphics::TILE_SIZE as f32,
                last: Correction::None,
            },
        );
        match correction {
            Correction::None => {}
            Correction::Left | Correction::Right => {
                vel.x = 0.0;
                aabb.last = correction;
            }
            Correction::Up | Correction::Down => {
                vel.y = 0.0;
                aabb.last = correction;
            }
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
    (game_input, graphics::GameInput),
    (camera, (f32, f32)),
    (control_point, (f32, f32))
);
pub fn player_system(
    timer: &mut Timer,
    game_input: &mut graphics::GameInput,
    camera: &mut (f32, f32),
    control_point: &mut (f32, f32),
    aabb: &mut AABB,
    vel: &mut Velocity,
    player: &mut Player,
) {
    if aabb.last == Correction::Up {
        player.can_jump = 0.1;
    } else if player.can_jump > 0.0 {
        player.can_jump -= timer.dt();
    } else {
        player.can_jump = 0.0;
    }
    println!("{:?}", player.can_jump);
    vel.y -= (if game_input.crouch { 400.0 } else { 200.0 }) * timer.dt();
    vel.x = 0.0;
    if game_input.left {
        vel.x += -100.0;
    }
    if game_input.right {
        vel.x += 100.0;
    }
    if game_input.jump && player.can_jump > 0.0 {
        vel.y = 100.0;
        player.can_jump = 0.0;
    }
    camera.0 = aabb.x;
    camera.1 = aabb.y;
    control_point.0 = aabb.x;
    control_point.1 = aabb.y;
}
