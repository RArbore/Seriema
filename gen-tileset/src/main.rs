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

const LEFT_MASK: u8 = 0x01;
const UP_LEFT_MASK: u8 = 0x02;
const UP_MASK: u8 = 0x04;
const UP_RIGHT_MASK: u8 = 0x08;
const RIGHT_MASK: u8 = 0x10;
const DOWN_RIGHT_MASK: u8 = 0x20;
const DOWN_MASK: u8 = 0x40;
const DOWN_LEFT_MASK: u8 = 0x80;

#[repr(u8)]
#[derive(Clone, Copy, Debug)]
enum QuadEdgeType {
    Clean,
    Diag,
    Horiz,
    Vert,
    Full,
}

fn num_to_edges(n: u8) -> [QuadEdgeType; 4] {
    // n & MASK == 0 means there is not a similar neighbor in the MASK direction
    let mut ret = [
        if n & UP_LEFT_MASK == 0 {
            QuadEdgeType::Diag
        } else {
            QuadEdgeType::Clean
        },
        if n & UP_RIGHT_MASK == 0 {
            QuadEdgeType::Diag
        } else {
            QuadEdgeType::Clean
        },
        if n & DOWN_LEFT_MASK == 0 {
            QuadEdgeType::Diag
        } else {
            QuadEdgeType::Clean
        },
        if n & DOWN_RIGHT_MASK == 0 {
            QuadEdgeType::Diag
        } else {
            QuadEdgeType::Clean
        },
    ];

    if n & LEFT_MASK == 0 {
        ret[0] = QuadEdgeType::Horiz;
        ret[2] = QuadEdgeType::Horiz;
    }
    if n & RIGHT_MASK == 0 {
        ret[1] = QuadEdgeType::Horiz;
        ret[3] = QuadEdgeType::Horiz;
    }
    if n & UP_MASK == 0 {
        ret[0] = QuadEdgeType::Vert;
        ret[1] = QuadEdgeType::Vert;
    }
    if n & DOWN_MASK == 0 {
        ret[2] = QuadEdgeType::Vert;
        ret[3] = QuadEdgeType::Vert;
    }

    if n & LEFT_MASK == 0 && n & UP_MASK == 0 {
        ret[0] = QuadEdgeType::Full;
    }
    if n & RIGHT_MASK == 0 && n & UP_MASK == 0 {
        ret[1] = QuadEdgeType::Full;
    }
    if n & LEFT_MASK == 0 && n & DOWN_MASK == 0 {
        ret[2] = QuadEdgeType::Full;
    }
    if n & RIGHT_MASK == 0 && n & DOWN_MASK == 0 {
        ret[3] = QuadEdgeType::Full;
    }

    ret
}

fn quad_to_coord(quad_num: u32, quad_type: QuadEdgeType) -> (u32, u32) {
    match quad_type {
        QuadEdgeType::Clean => (16, 16),
        QuadEdgeType::Diag => (8 * (quad_num % 2), 8 * (quad_num / 2)),
        QuadEdgeType::Horiz => (8 * (quad_num % 2) + 16, 0),
        QuadEdgeType::Vert => (8 * (quad_num / 2) + 16, 8),
        QuadEdgeType::Full => (8 * (quad_num % 2), 8 * (quad_num / 2) + 16),
    }
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
        assert_ne!(arg, &arg.replace("assets/", "assets/gen/"));
        let img = Reader::open(arg)
            .expect(&format!("Couldn't open file: {}.", arg)[..])
            .decode()
            .expect(&format!("File couldn't be coded: {}.", arg)[..]);
        let mut tileset = RgbaImage::new(256 * 16, 16);
        for i in 0u8..=255u8 {
            let edges = num_to_edges(i);
            let mut coords: [(u32, u32); 4] = [(0, 0), (0, 0), (0, 0), (0, 0)];
            for j in 0..4 {
                coords[j] = quad_to_coord(j as u32, edges[j]);
            }
            let constructed = concat4([
                img.crop_imm(coords[0].0, coords[0].1, 8, 8),
                img.crop_imm(coords[1].0, coords[1].1, 8, 8),
                img.crop_imm(coords[2].0, coords[2].1, 8, 8),
                img.crop_imm(coords[3].0, coords[3].1, 8, 8),
            ]);
            _ = tileset.copy_from(&constructed, i as u32 * 16, 0);
        }
        _ = tileset.save(arg.replace("assets/", "assets/gen/"));
    }
}
