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

use std::fs::File;
use std::io::prelude::*;

extern crate graphics;

const OFFSET: i64 = 100000 * graphics::CHUNK_SIZE as i64;

const EDGE_OFFSETS: [(usize, usize); 8] = [
    (0, 1),
    (0, 2),
    (1, 2),
    (2, 2),
    (2, 1),
    (2, 0),
    (1, 0),
    (0, 0),
];

fn calculate_tile_edges(tile_x: usize, tile_y: usize, tiles: &graphics::Tiles) -> u8 {
    let chunk = tiles.get(&(tile_x / graphics::CHUNK_SIZE, tile_y / graphics::CHUNK_SIZE));
    let tile = match chunk {
        Some(arr) => arr[tile_x % graphics::CHUNK_SIZE][tile_y % graphics::CHUNK_SIZE].0,
        None => graphics::Tile::NoTile,
    };

    let mut acc: u8 = 0;
    for i in 0..8 {
        let (o_x, o_y) = EDGE_OFFSETS[i];
        let (o_x, o_y) = (o_x + tile_x - 1, o_y + tile_y - 1);
        let chunk = tiles.get(&(o_x / graphics::CHUNK_SIZE, o_y / graphics::CHUNK_SIZE));
        let o_tile = match chunk {
            Some(arr) => arr[o_x % graphics::CHUNK_SIZE][o_y % graphics::CHUNK_SIZE].0,
            None => graphics::Tile::NoTile,
        };
        acc |= ((o_tile == tile) as u8) << i;
    }
    acc
}

fn save_tiles(tiles: &graphics::Tiles, file_path: &str) -> std::io::Result<()> {
    let serialized = serde_json::to_string(tiles)?;
    let mut file = File::create(file_path)?;
    file.write(serialized.as_bytes())?;
    Ok(())
}

fn main() {
    let mut tiles: graphics::Tiles = Default::default();
    let mut cx = 0.0;
    let mut cy = 0.0;
    let mut dc: Option<(f32, f32)> = None;
    pollster::block_on(graphics::Graphics::new()).run(move |controller, _, _, _, _| {
        if controller.left_click {
            let world_x = (controller.cursor_x as f32 / graphics::PIXEL_SIZE as f32) + cx;
            let world_y = -(controller.cursor_y as f32 / graphics::PIXEL_SIZE as f32) + cy;
            let tile_x = (world_x as i64 / graphics::TILE_SIZE as i64
                - (if world_x < 0.0 { 1 } else { 0 })
                + OFFSET) as usize;
            let tile_y = (world_y as i64 / graphics::TILE_SIZE as i64
                - (if world_y < 0.0 { 1 } else { 0 })
                + OFFSET) as usize;
            let chunk =
                tiles.get_mut(&(tile_x / graphics::CHUNK_SIZE, tile_y / graphics::CHUNK_SIZE));

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

            for x in 0usize..3 {
                for y in 0usize..3 {
                    let (o_x, o_y) = (tile_x + x - 1, tile_y + y - 1);
                    let frame = calculate_tile_edges(o_x, o_y, &tiles) as usize;
                    let chunk =
                        tiles.get_mut(&(o_x / graphics::CHUNK_SIZE, o_y / graphics::CHUNK_SIZE));
                    if let Some(arr) = chunk {
                        arr[o_x % graphics::CHUNK_SIZE][o_y % graphics::CHUNK_SIZE] =
                            (|(t, _)| (t, frame))(
                                arr[o_x % graphics::CHUNK_SIZE][o_y % graphics::CHUNK_SIZE],
                            );
                    }
                }
            }
        }

        if controller.middle_click {
            if let Some((dcx, dcy)) = dc {
                let (ncx, ncy) = (
                    controller.cursor_x as f32 / graphics::PIXEL_SIZE as f32,
                    -controller.cursor_y as f32 / graphics::PIXEL_SIZE as f32,
                );
                cx -= ncx - dcx;
                cy -= ncy - dcy;
                dc = Some((
                    controller.cursor_x as f32 / graphics::PIXEL_SIZE as f32,
                    -controller.cursor_y as f32 / graphics::PIXEL_SIZE as f32,
                ));
            } else {
                dc = Some((
                    controller.cursor_x as f32 / graphics::PIXEL_SIZE as f32,
                    -controller.cursor_y as f32 / graphics::PIXEL_SIZE as f32,
                ));
            }
        } else {
            dc = None;
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
