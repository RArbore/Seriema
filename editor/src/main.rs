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
use std::process;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread;

use druid::widget::*;
use druid::*;

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
    let mut min: (usize, usize) = (usize::MAX, usize::MAX);
    for (key, _) in tiles {
        if key.0 < min.0 {
            min.0 = key.0;
        }
        if key.1 < min.1 {
            min.1 = key.1;
        }
    }
    let mut adjusted_tiles: graphics::Tiles = Default::default();
    for (key, value) in tiles {
        adjusted_tiles.insert((key.0 - min.0, key.1 - min.1), value.clone());
    }
    let serialized = bincode::serialize(&adjusted_tiles).unwrap();
    let mut file = File::create(file_path)?;
    file.write(&serialized)?;
    Ok(())
}

macro_rules! create_images {
    ($($x:literal),+) => {
        [
            $(
                ImageBuf::from_raw(
                    image::load_from_memory(&include_bytes!($x)[..])
                        .unwrap()
                        .crop_imm(
                            4 * graphics::TILE_SIZE as u32,
                            0,
                            graphics::TILE_SIZE as u32,
                            graphics::TILE_SIZE as u32,
                        )
                        .as_bytes(),
                    piet::ImageFormat::RgbaSeparate,
                    graphics::TILE_SIZE,
                    graphics::TILE_SIZE,
                )
            ),+
        ]
    };
}

const SELECTIONS: [Selection; 3] = [
    Selection::Tile(graphics::Tile::NoTile),
    Selection::Tile(graphics::Tile::TestTile1),
    Selection::Tile(graphics::Tile::TestTile2),
];

fn build_ui() -> impl Widget<()> {
    let png_data = create_images!(
        "../../assets/editor/notile.png",
        "../../assets/test-tileset1.png",
        "../../assets/test-tileset2.png"
    );
    let images = png_data.into_iter().map(|png| {
        SizedBox::new(
            Image::new(png)
                .fill_mode(FillStrat::Contain)
                .interpolation_mode(piet::InterpolationMode::NearestNeighbor),
        )
        .fix_width(32.0)
        .fix_height(32.0)
    });

    let bin_spec = FileSpec::new("BIN file", &["bin"]);
    let save_dialog_options = FileDialogOptions::new()
        .allowed_types(vec![bin_spec])
        .default_type(bin_spec);
    let mut top = Flex::column().with_child(Padding::new(
        10.0,
        Button::new("Save").on_click(move |ctx, _, _| {
            ctx.submit_command(Command::new(
                commands::SHOW_SAVE_PANEL,
                save_dialog_options.clone(),
                Target::Auto,
            ))
        }),
    ));
    let mut i = 0;
    for image in images {
        top.add_default_spacer();
        top.add_child(ControllerHost::new(
            image,
            Click::new(move |ctx, _, _| {
                ctx.submit_command(Command::new(
                    Selector::new("update_sel"),
                    SELECTIONS[i],
                    Target::Auto,
                ))
            }),
        ));
        i += 1;
    }
    top
}

struct Delegate {
    scene: Arc<Mutex<graphics::Tiles>>,
    sel: Arc<Mutex<Selection>>,
}

impl AppDelegate<()> for Delegate {
    fn command(
        &mut self,
        _ctx: &mut DelegateCtx,
        _target: Target,
        cmd: &Command,
        _data: &mut (),
        _env: &Env,
    ) -> Handled {
        if let Some(file_info) = cmd.get(commands::SAVE_FILE_AS) {
            if let Err(e) = save_tiles(
                &self.scene.lock().unwrap(),
                file_info.path().to_str().unwrap(),
            ) {
                println!("Error writing file: {}", e);
            }
            Handled::Yes
        } else if let Some(selection) = cmd.get::<Selection>(Selector::new("update_sel")) {
            let mut sel_ref = self.sel.lock().unwrap();
            *sel_ref = *selection;
            Handled::Yes
        } else {
            Handled::No
        }
    }
}

#[derive(Clone, Copy)]
enum Selection {
    Tile(graphics::Tile),
}

impl Default for Selection {
    fn default() -> Self {
        Selection::Tile(Default::default())
    }
}

fn main() {
    let scene: Arc<Mutex<graphics::Tiles>> = Default::default();
    let cur_selection: Arc<Mutex<Selection>> = Default::default();
    let scene_clone = Arc::clone(&scene);
    let cur_selection_clone = Arc::clone(&cur_selection);

    let (tx, rx): (Sender<()>, Receiver<()>) = mpsc::channel();
    thread::spawn(move || {
        AppLauncher::with_window(
            WindowDesc::new(|| build_ui())
                .window_size((400.0, 400.0))
                .resizable(false)
                .title("Editor Tools"),
        )
        .delegate(Delegate {
            scene: scene_clone,
            sel: cur_selection_clone,
        })
        .launch(())
        .expect("Failed to editor tools window.");
        tx.send(()).unwrap();
    });

    let mut cx = 0.0;
    let mut cy = 0.0;
    let mut dc: Option<(f32, f32)> = None;
    pollster::block_on(graphics::Graphics::new()).run(move |controller, _, _, _, _| {
        if let Ok(()) = rx.try_recv() {
            process::exit(0);
        }

        let tiles: &mut graphics::Tiles = &mut scene.lock().unwrap();

        if controller.left_click {
            let world_x = (controller.cursor_x as f32 / graphics::PIXEL_SIZE as f32) + cx;
            let world_y = -(controller.cursor_y as f32 / graphics::PIXEL_SIZE as f32) + cy;
            let selection: &Selection = &cur_selection.lock().unwrap();
            match selection {
                Selection::Tile(tile) => {
                    let tile_x = (world_x as i64 / graphics::TILE_SIZE as i64
                        - (if world_x < 0.0 { 1 } else { 0 })
                        + OFFSET) as usize;
                    let tile_y = (world_y as i64 / graphics::TILE_SIZE as i64
                        - (if world_y < 0.0 { 1 } else { 0 })
                        + OFFSET) as usize;
                    let chunk = tiles
                        .get_mut(&(tile_x / graphics::CHUNK_SIZE, tile_y / graphics::CHUNK_SIZE));

                    match chunk {
                        Some(arr) => {
                            arr[tile_x % graphics::CHUNK_SIZE][tile_y % graphics::CHUNK_SIZE] =
                                (*tile, 0);
                        }
                        None => {
                            let mut new_chunk: [[(graphics::Tile, usize); graphics::CHUNK_SIZE];
                                graphics::CHUNK_SIZE] = Default::default();
                            new_chunk[tile_x % graphics::CHUNK_SIZE]
                                [tile_y % graphics::CHUNK_SIZE] = (*tile, 0);
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
                            let chunk = tiles
                                .get_mut(&(o_x / graphics::CHUNK_SIZE, o_y / graphics::CHUNK_SIZE));
                            if let Some(arr) = chunk {
                                arr[o_x % graphics::CHUNK_SIZE][o_y % graphics::CHUNK_SIZE] =
                                    (|(t, _)| (t, frame))(
                                        arr[o_x % graphics::CHUNK_SIZE][o_y % graphics::CHUNK_SIZE],
                                    );
                            }
                        }
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
