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

pub fn correct_collision(aabb1: &mut AABB, aabb2: &mut AABB) -> Correction {
    if aabb1.x < aabb2.x {
        if aabb1.y < aabb2.y {
            if (aabb1.x + aabb1.w / 2.0) - (aabb2.x - aabb2.w / 2.0)
                < (aabb1.y + aabb1.h / 2.0) - (aabb2.y - aabb2.h / 2.0)
            {
                correct_left(aabb1, aabb2)
            } else {
                correct_down(aabb1, aabb2)
            }
        } else {
            if (aabb1.x + aabb1.w / 2.0) - (aabb2.x - aabb2.w / 2.0)
                < (aabb2.y + aabb2.h / 2.0) - (aabb1.y - aabb1.h / 2.0)
            {
                correct_left(aabb1, aabb2)
            } else {
                correct_up(aabb1, aabb2)
            }
        }
    } else {
        if aabb1.y < aabb2.y {
            if (aabb2.x + aabb2.w / 2.0) - (aabb1.x - aabb1.w / 2.0)
                < (aabb1.y + aabb1.h / 2.0) - (aabb2.y - aabb2.h / 2.0)
            {
                correct_right(aabb1, aabb2)
            } else {
                correct_down(aabb1, aabb2)
            }
        } else {
            if (aabb2.x + aabb2.w / 2.0) - (aabb1.x - aabb1.w / 2.0)
                < (aabb2.y + aabb2.h / 2.0) - (aabb1.y - aabb1.h / 2.0)
            {
                correct_right(aabb1, aabb2)
            } else {
                correct_up(aabb1, aabb2)
            }
        }
    }
}

pub fn get_all_tiles_in_aabb(
    aabb: &AABB,
    tiles: &graphics::Tiles,
) -> Vec<(graphics::Tile, usize, usize)> {
    let mut vec: Vec<(graphics::Tile, usize, usize)> = vec![];
    let min_i: usize =
        ((aabb.x - aabb.w / 2.0) / (graphics::TILE_SIZE * graphics::CHUNK_SIZE) as f32) as usize;
    let min_j: usize =
        ((aabb.y - aabb.h / 2.0) / (graphics::TILE_SIZE * graphics::CHUNK_SIZE) as f32) as usize;
    let max_i: usize =
        ((aabb.x + aabb.w / 2.0) / (graphics::TILE_SIZE * graphics::CHUNK_SIZE) as f32) as usize;
    let max_j: usize =
        ((aabb.y + aabb.h / 2.0) / (graphics::TILE_SIZE * graphics::CHUNK_SIZE) as f32) as usize;
    for i in min_i..=max_i {
        for j in min_j..=max_j {
            if let Some(chunk) = tiles.get(&(i, j)) {
                for ii in 0..graphics::CHUNK_SIZE {
                    for jj in 0..graphics::CHUNK_SIZE {
                        if (i > min_i
                            || ((ii + i * graphics::CHUNK_SIZE + 1) * graphics::TILE_SIZE) as f32
                                > aabb.x - aabb.w / 2.0)
                            && (j > min_j
                                || ((jj + j * graphics::CHUNK_SIZE + 1) * graphics::TILE_SIZE)
                                    as f32
                                    > aabb.y - aabb.h / 2.0)
                            && (i < min_i
                                || (((ii + i * graphics::CHUNK_SIZE) * graphics::TILE_SIZE) as f32)
                                    < aabb.x + aabb.w / 2.0)
                            && (j < min_j
                                || (((jj + j * graphics::CHUNK_SIZE) * graphics::TILE_SIZE) as f32)
                                    < aabb.y + aabb.h / 2.0)
                        {
                            vec.push((
                                chunk[ii][jj].0,
                                ii + i * graphics::CHUNK_SIZE,
                                jj + j * graphics::CHUNK_SIZE,
                            ));
                        }
                    }
                }
            }
        }
    }
    vec
}
