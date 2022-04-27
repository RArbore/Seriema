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

extern crate graphics;

use super::components::*;

#[repr(u8)]
#[derive(PartialEq, Eq, Debug)]
pub enum Correction {
    None = 0,
    Left = 1,
    Up = 2,
    Right = 4,
    Down = 8,
}

pub fn collides(aabb1: &AABB, aabb2: &AABB) -> bool {
    2.0 * (aabb1.x - aabb2.x).abs() <= aabb1.w + aabb2.w
        && 2.0 * (aabb1.y - aabb2.y).abs() <= aabb1.h + aabb2.h
}

pub fn correct_collision(aabb1_after: &mut AABB, aabb2: &mut AABB, vel1: (f32, f32), dt: f32) {
    let aabb1 = AABB {
        x: aabb1_after.x - vel1.0 * dt,
        y: aabb1_after.y - vel1.1 * dt,
        w: aabb1_after.w,
        h: aabb1_after.h,
        last: 0,
    };
    let time_to_left_coll = (aabb2.x - aabb1.x + (aabb1.w + aabb2.w) / 2.0) / vel1.0;
    let time_to_right_coll = (aabb2.x - aabb1.x - (aabb1.w + aabb2.w) / 2.0) / vel1.0;
    let time_to_up_coll = (aabb2.y - aabb1.y + (aabb1.h + aabb2.h) / 2.0) / vel1.1;
    let time_to_down_coll = (aabb2.y - aabb1.y - (aabb1.h + aabb2.h) / 2.0) / vel1.1;
    let left_collides = vel1.0 != 0.0
        && time_to_left_coll >= 0.0
        && time_to_left_coll <= dt
        && collides(
            &AABB {
                x: aabb1_after.x.max(aabb1.x + vel1.0 * time_to_left_coll),
                y: aabb1_after.y,
                w: aabb1.w,
                h: aabb1.h,
                last: 0,
            },
            aabb2,
        );
    let right_collides = vel1.0 != 0.0
        && time_to_right_coll >= 0.0
        && time_to_right_coll <= dt
        && collides(
            &AABB {
                x: aabb1_after.x.min(aabb1.x + vel1.0 * time_to_right_coll),
                y: aabb1_after.y,
                w: aabb1.w,
                h: aabb1.h,
                last: 0,
            },
            aabb2,
        );
    let up_collides = vel1.1 != 0.0
        && time_to_up_coll >= 0.0
        && time_to_up_coll <= dt
        && collides(
            &AABB {
                x: aabb1_after.x,
                y: aabb1_after.y.max(aabb1.y + vel1.1 * time_to_up_coll),
                w: aabb1.w,
                h: aabb1.h,
                last: 0,
            },
            aabb2,
        );
    let down_collides = vel1.1 != 0.0
        && time_to_down_coll >= 0.0
        && time_to_down_coll <= dt
        && collides(
            &AABB {
                x: aabb1_after.x,
                y: aabb1_after.y.min(aabb1.y + vel1.1 * time_to_down_coll),
                w: aabb1.w,
                h: aabb1.h,
                last: 0,
            },
            aabb2,
        );
    if left_collides
        && (!right_collides || time_to_left_coll < time_to_right_coll)
        && (!up_collides || time_to_left_coll < time_to_up_coll)
        && (!down_collides || time_to_left_coll < time_to_down_coll)
    {
        *aabb1_after = AABB {
            x: aabb1_after.x.max(aabb1.x + vel1.0 * time_to_left_coll),
            y: aabb1_after.y,
            w: aabb1.w,
            h: aabb1.h,
            last: aabb1_after.last | Correction::Left as u8,
        };
    } else if right_collides
        && (!left_collides || time_to_right_coll < time_to_left_coll)
        && (!up_collides || time_to_right_coll < time_to_up_coll)
        && (!down_collides || time_to_right_coll < time_to_down_coll)
    {
        *aabb1_after = AABB {
            x: aabb1_after.x.min(aabb1.x + vel1.0 * time_to_right_coll),
            y: aabb1_after.y,
            w: aabb1.w,
            h: aabb1.h,
            last: aabb1_after.last | Correction::Right as u8,
        };
    } else if up_collides
        && (!left_collides || time_to_up_coll < time_to_left_coll)
        && (!right_collides || time_to_up_coll < time_to_right_coll)
        && (!down_collides || time_to_up_coll < time_to_down_coll)
    {
        *aabb1_after = AABB {
            x: aabb1_after.x,
            y: aabb1_after.y.max(aabb1.y + vel1.1 * time_to_up_coll),
            w: aabb1.w,
            h: aabb1.h,
            last: aabb1_after.last | Correction::Up as u8,
        };
    } else if down_collides
        && (!left_collides || time_to_down_coll < time_to_left_coll)
        && (!right_collides || time_to_down_coll < time_to_right_coll)
        && (!up_collides || time_to_down_coll < time_to_up_coll)
    {
        *aabb1_after = AABB {
            x: aabb1_after.x,
            y: aabb1_after.y.min(aabb1.y + vel1.1 * time_to_down_coll),
            w: aabb1.w,
            h: aabb1.h,
            last: aabb1_after.last | Correction::Down as u8,
        };
    }
}

pub fn get_all_tiles_in_aabb(
    aabb: &AABB,
    tiles: &graphics::Tiles,
) -> Vec<(graphics::Tile, i64, i64)> {
    let mut vec: Vec<(graphics::Tile, i64, i64)> = vec![];
    let min_i: i64 =
        ((aabb.x - aabb.w / 2.0) as i64 - graphics::TILE_SIZE as i64) / graphics::TILE_SIZE as i64;
    let min_j: i64 =
        ((aabb.y - aabb.h / 2.0) as i64 - graphics::TILE_SIZE as i64) / graphics::TILE_SIZE as i64;
    let max_i: i64 =
        ((aabb.x + aabb.w / 2.0) as i64 + graphics::TILE_SIZE as i64) / graphics::TILE_SIZE as i64;
    let max_j: i64 =
        ((aabb.y + aabb.h / 2.0) as i64 + graphics::TILE_SIZE as i64) / graphics::TILE_SIZE as i64;
    for j in min_j..=max_j {
        for i in min_i..=max_i {
            if let Some((tile, _)) = tiles.at(i, j) {
                vec.push((*tile, i, j));
            }
        }
    }
    vec
}
