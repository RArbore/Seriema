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

use serde::{Deserialize, Serialize};

pub const NUM_TILES: usize = 2;
pub const NUM_TILE_VERSIONS: usize = 256;
pub const TILE_SIZE: usize = 16;
pub const CHUNK_SIZE: usize = 16;
pub const PIXEL_SIZE: usize = 4;

#[repr(usize)]
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug, Deserialize, Serialize)]
pub enum Tile {
    TestTile1,
    TestTile2,
    NoTile,
}

impl Default for Tile {
    fn default() -> Self {
        Tile::NoTile
    }
}

#[derive(Default, Deserialize, Serialize)]
pub struct Tiles(pub HashMap<(i64, i64), [[(Tile, usize); CHUNK_SIZE]; CHUNK_SIZE]>);

impl Tiles {
    pub fn get(&mut self, x: i64, y: i64) -> &(Tile, usize) {
        let c_x = x.div_euclid(CHUNK_SIZE as i64);
        let c_y = y.div_euclid(CHUNK_SIZE as i64);
        let t_x = x.rem_euclid(CHUNK_SIZE as i64);
        let t_y = y.rem_euclid(CHUNK_SIZE as i64);
        &self.0.entry((c_x, c_y)).or_insert(Default::default())[t_x as usize][t_y as usize]
    }

    pub fn get_mut(&mut self, x: i64, y: i64) -> &mut (Tile, usize) {
        let c_x = x.div_euclid(CHUNK_SIZE as i64);
        let c_y = y.div_euclid(CHUNK_SIZE as i64);
        let t_x = x.rem_euclid(CHUNK_SIZE as i64);
        let t_y = y.rem_euclid(CHUNK_SIZE as i64);
        &mut self.0.entry((c_x, c_y)).or_insert(Default::default())[t_x as usize][t_y as usize]
    }

    pub fn at(&self, x: i64, y: i64) -> Option<&(Tile, usize)> {
        let c_x = x.div_euclid(CHUNK_SIZE as i64);
        let c_y = y.div_euclid(CHUNK_SIZE as i64);
        let t_x = x.rem_euclid(CHUNK_SIZE as i64);
        let t_y = y.rem_euclid(CHUNK_SIZE as i64);
        Some(&self.0.get(&(c_x, c_y))?[t_x as usize][t_y as usize])
    }

    pub fn at_mut(&mut self, x: i64, y: i64) -> Option<&mut (Tile, usize)> {
        let c_x = x.div_euclid(CHUNK_SIZE as i64);
        let c_y = y.div_euclid(CHUNK_SIZE as i64);
        let t_x = x.rem_euclid(CHUNK_SIZE as i64);
        let t_y = y.rem_euclid(CHUNK_SIZE as i64);
        Some(&mut self.0.get_mut(&(c_x, c_y))?[t_x as usize][t_y as usize])
    }
}

pub type TileBatch = [Vec<(usize, i64, i64)>; NUM_TILES];
