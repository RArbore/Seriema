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

#[derive(PartialEq, Eq, Debug)]
pub enum Correction {
    None,
    Left,
    Up,
    Right,
    Down,
}

fn correct_left(aabb1: &mut AABB, aabb2: &mut AABB) -> Correction {
    if aabb1.x + aabb1.w / 2.0 > aabb2.x - aabb2.w / 2.0
        && (aabb1.y - aabb2.y).abs() < (aabb1.h + aabb2.h) / 2.0
    {
        aabb1.x = aabb2.x - (aabb1.w + aabb2.w) / 2.0;
        Correction::Left
    } else {
        Correction::None
    }
}

fn correct_up(aabb1: &mut AABB, aabb2: &mut AABB) -> Correction {
    if aabb1.y - aabb1.h / 2.0 < aabb2.y + aabb2.h / 2.0
        && (aabb1.x - aabb2.x).abs() < (aabb1.w + aabb2.w) / 2.0
    {
        aabb1.y = aabb2.y + (aabb1.h + aabb2.h) / 2.0;
        Correction::Up
    } else {
        Correction::None
    }
}

fn correct_right(aabb1: &mut AABB, aabb2: &mut AABB) -> Correction {
    if aabb1.x - aabb1.w / 2.0 < aabb2.x + aabb2.w / 2.0
        && (aabb1.y - aabb2.y).abs() < (aabb1.h + aabb2.h) / 2.0
    {
        aabb1.x = aabb2.x + (aabb1.w + aabb2.w) / 2.0;
        Correction::Right
    } else {
        Correction::None
    }
}

fn correct_down(aabb1: &mut AABB, aabb2: &mut AABB) -> Correction {
    if aabb1.y + aabb1.h / 2.0 > aabb2.y - aabb2.h / 2.0
        && (aabb1.x - aabb2.x).abs() < (aabb1.w + aabb2.w) / 2.0
    {
        aabb1.y = aabb2.y - (aabb1.h + aabb2.h) / 2.0;
        Correction::Down
    } else {
        Correction::None
    }
}

pub fn correct_collision(aabb1: &mut AABB, aabb2: &mut AABB) -> (Correction, f32) {
    if aabb1.x < aabb2.x {
        if aabb1.y < aabb2.y {
            if (aabb1.x + aabb1.w / 2.0) - (aabb2.x - aabb2.w / 2.0)
                < (aabb1.y + aabb1.h / 2.0) - (aabb2.y - aabb2.h / 2.0)
            {
                let area = (((aabb1.x + aabb1.w / 2.0) - (aabb2.x - aabb2.w / 2.0))
                    * ((aabb1.y + aabb1.h / 2.0) - (aabb2.y - aabb2.h / 2.0)))
                    .abs();
                (correct_left(aabb1, aabb2), area)
            } else {
                let area = (((aabb1.x + aabb1.w / 2.0) - (aabb2.x - aabb2.w / 2.0))
                    * ((aabb1.y + aabb1.h / 2.0) - (aabb2.y - aabb2.h / 2.0)))
                    .abs();
                (correct_down(aabb1, aabb2), area)
            }
        } else {
            if (aabb1.x + aabb1.w / 2.0) - (aabb2.x - aabb2.w / 2.0)
                < (aabb2.y + aabb2.h / 2.0) - (aabb1.y - aabb1.h / 2.0)
            {
                let area = (((aabb1.x + aabb1.w / 2.0) - (aabb2.x - aabb2.w / 2.0))
                    * ((aabb2.y + aabb2.h / 2.0) - (aabb1.y - aabb1.h / 2.0)))
                    .abs();
                (correct_left(aabb1, aabb2), area)
            } else {
                let area = (((aabb1.x + aabb1.w / 2.0) - (aabb2.x - aabb2.w / 2.0))
                    * ((aabb2.y + aabb2.h / 2.0) - (aabb1.y - aabb1.h / 2.0)))
                    .abs();
                (correct_up(aabb1, aabb2), area)
            }
        }
    } else {
        if aabb1.y < aabb2.y {
            if (aabb2.x + aabb2.w / 2.0) - (aabb1.x - aabb1.w / 2.0)
                < (aabb1.y + aabb1.h / 2.0) - (aabb2.y - aabb2.h / 2.0)
            {
                let area = (((aabb2.x + aabb2.w / 2.0) - (aabb1.x - aabb1.w / 2.0))
                    * ((aabb1.y + aabb1.h / 2.0) - (aabb2.y - aabb2.h / 2.0)))
                    .abs();
                (correct_right(aabb1, aabb2), area)
            } else {
                let area = (((aabb2.x + aabb2.w / 2.0) - (aabb1.x - aabb1.w / 2.0))
                    * ((aabb1.y + aabb1.h / 2.0) - (aabb2.y - aabb2.h / 2.0)))
                    .abs();
                (correct_down(aabb1, aabb2), area)
            }
        } else {
            if (aabb2.x + aabb2.w / 2.0) - (aabb1.x - aabb1.w / 2.0)
                < (aabb2.y + aabb2.h / 2.0) - (aabb1.y - aabb1.h / 2.0)
            {
                let area = (((aabb2.x + aabb2.w / 2.0) - (aabb1.x - aabb1.w / 2.0))
                    * ((aabb2.y + aabb2.h / 2.0) - (aabb1.y - aabb1.h / 2.0)))
                    .abs();
                (correct_right(aabb1, aabb2), area)
            } else {
                let area = (((aabb2.x + aabb2.w / 2.0) - (aabb1.x - aabb1.w / 2.0))
                    * ((aabb2.y + aabb2.h / 2.0) - (aabb1.y - aabb1.h / 2.0)))
                    .abs();
                (correct_up(aabb1, aabb2), area)
            }
        }
    }
}

pub fn get_all_tiles_in_aabb(
    aabb: &AABB,
    tiles: &graphics::Tiles,
) -> Vec<(graphics::Tile, i64, i64)> {
    let mut vec: Vec<(graphics::Tile, i64, i64)> = vec![];
    let min_i: i64 = ((aabb.x - aabb.w / 2.0) as i64 - 1) / graphics::TILE_SIZE as i64;
    let min_j: i64 = ((aabb.y - aabb.h / 2.0) as i64 - 1) / graphics::TILE_SIZE as i64;
    let max_i: i64 = ((aabb.x + aabb.w / 2.0) as i64 + 1) / graphics::TILE_SIZE as i64;
    let max_j: i64 = ((aabb.y + aabb.h / 2.0) as i64 + 1) / graphics::TILE_SIZE as i64;
    for i in min_i..=max_i {
        for j in min_j..=max_j {
            if let Some((tile, _)) = tiles.at(i, j) {
                vec.push((*tile, i, j));
            }
        }
    }
    vec
}
