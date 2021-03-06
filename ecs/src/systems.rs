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
    aabb.last = 0;
    let tiles_to_check = get_all_tiles_in_aabb(aabb, tiles);

    let mut run_info: Option<(i64, i64, i64, i64)> = None;

    for (tile_id, ux, uy) in tiles_to_check {
        match run_info {
            None => {
                if tile_id != graphics::Tile::NoTile {
                    run_info = Some((ux, uy, ux, uy));
                } else {
                    run_info = None;
                }
            }
            Some((sx, sy, ex, ey)) => {
                if tile_id != graphics::Tile::NoTile && uy == sy {
                    run_info = Some((sx, sy, ux, uy));
                } else {
                    correct_collision(
                        aabb,
                        &mut AABB {
                            x: ((ex + sx) as f32 * graphics::TILE_SIZE as f32
                                + graphics::TILE_SIZE as f32)
                                / 2.0,
                            y: ((ey + sy) as f32 * graphics::TILE_SIZE as f32
                                + graphics::TILE_SIZE as f32)
                                / 2.0,
                            w: ((ex - sx + 1) * graphics::TILE_SIZE as i64) as f32,
                            h: graphics::TILE_SIZE as f32,
                            last: 0,
                        },
                        (vel.x, vel.y),
                        timer.dt(),
                    );
                    if tile_id != graphics::Tile::NoTile {
                        run_info = Some((ux, uy, ux, uy));
                    } else {
                        run_info = None;
                    }
                }
            }
        }
    }
    if aabb.last & Correction::Left as u8 != 0 && vel.x < 0.0 {
        vel.x = 0.0;
    }
    if aabb.last & Correction::Right as u8 != 0 && vel.x > 0.0 {
        vel.x = 0.0;
    }
    if aabb.last & Correction::Up as u8 != 0 && vel.y < 0.0 {
        vel.y = 0.0;
    }
    if aabb.last & Correction::Down as u8 != 0 && vel.y > 0.0 {
        vel.y = 0.0;
    }
}

system_impl!(A, B, (sprite_batch_res, SpriteBatchRes));
pub fn render_sprite(sprite_batch: &mut SpriteBatchRes, aabb: &mut AABB, sprite: &mut Sprite) {
    sprite_batch.insert(
        sprite.sprite,
        sprite.frame,
        aabb.x + sprite.off_x,
        aabb.y + sprite.off_y,
        sprite.width,
        sprite.height,
    );
}

system_impl!(
    A,
    B,
    C,
    D,
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
    sprite: &mut Sprite,
    player: &mut Player,
) {
    if aabb.last & Correction::Up as u8 != 0 {
        player.can_jump = 0.1;
    } else if player.can_jump > 0.0 {
        player.can_jump -= timer.dt();
    } else {
        player.can_jump = 0.0;
    }
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
    if player.can_jump > 0.0 {
        sprite.frame = 0;
    } else {
        sprite.frame = 1;
    }
    camera.0 = aabb.x;
    camera.1 = aabb.y;
    control_point.0 = aabb.x;
    control_point.1 = aabb.y;
}
