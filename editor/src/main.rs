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

const OFFSET: i64 = 10000 * graphics::CHUNK_SIZE as i64;

fn main() {
    let mut tiles: graphics::Tiles = Default::default();
    let cx = 0.0;
    let cy = 0.0;
    pollster::block_on(graphics::Graphics::new()).run(move |controller, _, _, _, _| {
        let world_x = (controller.cursor_x as f32 / graphics::PIXEL_SIZE as f32) + cx;
        let world_y = -(controller.cursor_y as f32 / graphics::PIXEL_SIZE as f32) + cy;
        let tile_x = (world_x as i64 / graphics::TILE_SIZE as i64 - (world_x < 0.0) as i64 + OFFSET)
            as usize;
        let tile_y = (world_y as i64 / graphics::TILE_SIZE as i64 - (world_y < 0.0) as i64 + OFFSET)
            as usize;
        let chunk = tiles.get_mut(&(tile_x / graphics::CHUNK_SIZE, tile_y / graphics::CHUNK_SIZE));
        match chunk {
            Some(arr) => {
                arr[tile_x % graphics::CHUNK_SIZE][tile_y % graphics::CHUNK_SIZE] =
                    (graphics::Tile::TestTile1, 0);
            }
            None => {
                let mut new_chunk: [[(graphics::Tile, usize); graphics::CHUNK_SIZE];
                    graphics::CHUNK_SIZE] = Default::default();
                new_chunk[tile_x % graphics::CHUNK_SIZE][tile_y % graphics::CHUNK_SIZE] =
                    (graphics::Tile::TestTile1, 0);
                tiles.insert(
                    (tile_x / graphics::CHUNK_SIZE, tile_y / graphics::CHUNK_SIZE),
                    new_chunk,
                );
            }
        }

        let mut tile_batch: graphics::TileBatch = Default::default();
        for (coords, data) in tiles.iter() {
            for r in 0..graphics::CHUNK_SIZE {
                for c in 0..graphics::CHUNK_SIZE {
                    if data[r][c].0 != graphics::Tile::NoTile {
                        tile_batch[data[r][c].0 as usize].push((
                            data[r][c].1,
                            coords.0 * graphics::CHUNK_SIZE + r,
                            coords.1 * graphics::CHUNK_SIZE + c,
                        ));
                    }
                }
            }
        }

        (
            Default::default(),
            tile_batch,
            cx + (OFFSET * graphics::TILE_SIZE as i64) as f32,
            cy + (OFFSET * graphics::TILE_SIZE as i64) as f32,
            0.0,
            0.0,
        )
    });
}
