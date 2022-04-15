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

use std::env::*;

use image::imageops::*;
use image::io::*;
use image::*;

pub const LEFT_MASK: u8 = 0x01;
pub const UP_LEFT_MASK: u8 = 0x02;
pub const UP_MASK: u8 = 0x04;
pub const UP_RIGHT_MASK: u8 = 0x08;
pub const RIGHT_MASK: u8 = 0x10;
pub const DOWN_RIGHT_MASK: u8 = 0x20;
pub const DOWN_MASK: u8 = 0x40;
pub const DOWN_LEFT_MASK: u8 = 0x80;

#[repr(u8)]
pub enum QuadEdgeType {
    Clean,
    Diag,
    Horiz,
    Vert,
    Full,
}

fn num_to_edges(n: u8) -> [QuadEdgeType; 4] {
    let mut ret = [
        if n | UP_LEFT_MASK != 0 {
            QuadEdgeType::Diag
        } else {
            QuadEdgeType::Clean
        },
        if n | UP_RIGHT_MASK != 0 {
            QuadEdgeType::Diag
        } else {
            QuadEdgeType::Clean
        },
        if n | DOWN_LEFT_MASK != 0 {
            QuadEdgeType::Diag
        } else {
            QuadEdgeType::Clean
        },
        if n | DOWN_RIGHT_MASK != 0 {
            QuadEdgeType::Diag
        } else {
            QuadEdgeType::Clean
        },
    ];

    ret
}

fn concat4(quads: [DynamicImage; 4]) -> DynamicImage {
    let mut full = quads[0].clone().resize(16, 16, FilterType::Nearest);
    _ = full.copy_from(&quads[0], 0, 0);
    _ = full.copy_from(&quads[1], 8, 0);
    _ = full.copy_from(&quads[2], 0, 8);
    _ = full.copy_from(&quads[3], 8, 8);
    full
}

fn main() {
    let args: Vec<String> = args().collect();
    for arg in args[1..].iter() {
        let img = Reader::open(arg)
            .expect(&format!("Couldn't open file: {}.", arg)[..])
            .decode()
            .expect(&format!("File couldn't be coded: {}.", arg)[..]);
        println!("{:?}", img);
        let mut tileset = RgbaImage::new(256 * 16, 16);
        for i in 0u8..=255u8 {}
    }
}
