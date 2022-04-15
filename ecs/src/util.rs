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

pub enum Correction {
    None,
    Horizontal,
    Vertical,
}

fn correct_left(aabb1: &mut AABB, aabb2: &mut AABB) -> Correction {
    if aabb1.x + aabb1.w / 2.0 > aabb2.x - aabb2.w / 2.0
        && (aabb1.y - aabb2.y).abs() < (aabb1.h + aabb2.h) / 2.0
    {
        aabb1.x = aabb2.x - (aabb1.w + aabb2.w) / 2.0;
        println!("correct_left");
        Correction::Horizontal
    } else {
        Correction::None
    }
}

fn correct_up(aabb1: &mut AABB, aabb2: &mut AABB) -> Correction {
    if aabb1.y + aabb1.h / 2.0 > aabb2.y - aabb2.h / 2.0
        && (aabb1.x - aabb2.x).abs() < (aabb1.w + aabb2.w) / 2.0
    {
        aabb1.y = aabb2.y - (aabb1.h + aabb2.h) / 2.0;
        println!("correct_up");
        Correction::Vertical
    } else {
        Correction::None
    }
}

fn correct_right(aabb1: &mut AABB, aabb2: &mut AABB) -> Correction {
    if aabb1.x - aabb1.w / 2.0 < aabb2.x + aabb2.w / 2.0
        && (aabb1.y - aabb2.y).abs() < (aabb1.h + aabb2.h) / 2.0
    {
        aabb1.x = aabb2.x + (aabb1.w + aabb2.w) / 2.0;
        println!("correct_right");
        Correction::Horizontal
    } else {
        Correction::None
    }
}

fn correct_down(aabb1: &mut AABB, aabb2: &mut AABB) -> Correction {
    if aabb1.y - aabb1.h / 2.0 < aabb2.y + aabb2.h / 2.0
        && (aabb1.x - aabb2.x).abs() < (aabb1.w + aabb2.w) / 2.0
    {
        aabb1.y = aabb2.y + (aabb1.h + aabb2.h) / 2.0;
        println!("correct_down");
        Correction::Vertical
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
                correct_up(aabb1, aabb2)
            }
        } else {
            if (aabb1.x + aabb1.w / 2.0) - (aabb2.x - aabb2.w / 2.0)
                < (aabb2.y + aabb2.h / 2.0) - (aabb1.y - aabb1.h / 2.0)
            {
                correct_left(aabb1, aabb2)
            } else {
                correct_down(aabb1, aabb2)
            }
        }
    } else {
        if aabb1.y < aabb2.y {
            if (aabb2.x + aabb2.w / 2.0) - (aabb1.x - aabb1.w / 2.0)
                < (aabb1.y + aabb1.h / 2.0) - (aabb2.y - aabb2.h / 2.0)
            {
                correct_right(aabb1, aabb2)
            } else {
                correct_up(aabb1, aabb2)
            }
        } else {
            if (aabb2.x + aabb2.w / 2.0) - (aabb1.x - aabb1.w / 2.0)
                < (aabb2.y + aabb2.h / 2.0) - (aabb1.y - aabb1.h / 2.0)
            {
                correct_right(aabb1, aabb2)
            } else {
                correct_down(aabb1, aabb2)
            }
        }
    }
}
