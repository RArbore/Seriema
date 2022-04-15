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

use std::collections::HashMap;

pub const NUM_TILES: usize = 1;
pub const NUM_TILE_VERSIONS: usize = 256;
pub const TILE_SIZE: usize = 16;
pub const CHUNK_SIZE: usize = 16;
pub const PIXEL_SIZE: usize = 8;

#[repr(usize)]
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum Tile {
    TestTile1,
    NoTile,
}

pub type Tiles = HashMap<(usize, usize), [[(Tile, usize); CHUNK_SIZE]; CHUNK_SIZE]>;

pub type TileBatch = Vec<Vec<(usize, usize, usize)>>;
