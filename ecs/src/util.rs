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

fn correct_left(aabb1: &mut AABB, aabb2: &mut AABB) {
    println!("correct_left");
}

fn correct_up(aabb1: &mut AABB, aabb2: &mut AABB) {
    println!("correct_up");
}

fn correct_right(aabb1: &mut AABB, aabb2: &mut AABB) {
    println!("correct_right");
}

fn correct_down(aabb1: &mut AABB, aabb2: &mut AABB) {
    println!("correct_down");
}

pub fn correct_collision(aabb1: &mut AABB, aabb2: &mut AABB) {
    if aabb1.x < aabb2.x {
        if aabb1.y < aabb2.y {
            if (aabb1.x + aabb1.w / 2.0) - (aabb2.x - aabb2.w / 2.0)
                < (aabb1.y + aabb1.h / 2.0) - (aabb2.y - aabb2.h / 2.0)
            {
                correct_left(aabb1, aabb2);
            } else {
                correct_down(aabb1, aabb2);
            }
        } else {
            if (aabb1.x + aabb1.w / 2.0) - (aabb2.x - aabb2.w / 2.0)
                < (aabb2.y + aabb2.h / 2.0) - (aabb1.y - aabb1.h / 2.0)
            {
                correct_left(aabb1, aabb2);
            } else {
                correct_up(aabb1, aabb2);
            }
        }
    } else {
        if aabb1.y < aabb2.y {
            if (aabb2.x + aabb2.w / 2.0) - (aabb1.x - aabb1.w / 2.0)
                < (aabb1.y + aabb1.h / 2.0) - (aabb2.y - aabb2.h / 2.0)
            {
                correct_right(aabb1, aabb2);
            } else {
                correct_down(aabb1, aabb2);
            }
        } else {
            if (aabb2.x + aabb2.w / 2.0) - (aabb1.x - aabb1.w / 2.0)
                < (aabb2.y + aabb2.h / 2.0) - (aabb1.y - aabb1.h / 2.0)
            {
                correct_right(aabb1, aabb2);
            } else {
                correct_up(aabb1, aabb2);
            }
        }
    }
}
