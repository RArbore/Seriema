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

extern crate ecs;
extern crate graphics;

const EDGE_OFFSETS: [(i64, i64); 8] = [
    (-1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
    (1, 0),
    (1, -1),
    (0, -1),
    (-1, -1),
];

fn calculate_tile_edges(tile_x: i64, tile_y: i64, tiles: &graphics::Tiles) -> u8 {
    let tile = match tiles.at(tile_x, tile_y) {
        Some(x) => x.0,
        None => graphics::Tile::NoTile,
    };

    let mut acc: u8 = 0;
    for i in 0..8 {
        let (o_x, o_y) = EDGE_OFFSETS[i];
        let o_tile = match tiles.at(o_x + tile_x, o_y + tile_y) {
            Some(x) => x.0,
            None => graphics::Tile::NoTile,
        };
        acc |= ((o_tile == tile) as u8) << i;
    }
    acc
}

fn save_scene(
    tiles: &graphics::Tiles,
    entities: &Vec<ecs::EntityDesc>,
    file_path: &str,
) -> std::io::Result<()> {
    let serialized = bincode::serialize(&(tiles, entities)).unwrap();
    let mut file = File::create(file_path)?;
    file.write(&serialized)?;
    Ok(())
}

macro_rules! create_tile {
    ($x:literal) => {
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
    };
}

macro_rules! create_sprite {
    ($x:literal, $y:expr, $z:expr) => {
        ImageBuf::from_raw(
            image::load_from_memory(&include_bytes!($x)[..])
                .unwrap()
                .crop_imm($y * $z as u32, 0, $z as u32, $z as u32)
                .as_bytes(),
            piet::ImageFormat::RgbaSeparate,
            $z,
            $z,
        )
    };
}

#[derive(Clone, Copy)]
enum Selection {
    Tile(graphics::Tile),
    Entity(&'static (dyn Fn(f32, f32) -> ecs::EntityDesc + Sync)),
}

impl Default for Selection {
    fn default() -> Self {
        Selection::Tile(Default::default())
    }
}

const SELECTIONS: [Selection; 4] = [
    Selection::Tile(graphics::Tile::NoTile),
    Selection::Tile(graphics::Tile::TestTile1),
    Selection::Tile(graphics::Tile::TestTile2),
    Selection::Entity(&|x: f32, y: f32| ecs::EntityDesc::Player(ecs::PlayerDesc { x, y })),
];

fn build_ui() -> impl Widget<()> {
    let png_data = [
        create_tile!("../../assets/editor/notile.png"),
        create_tile!("../../assets/test-tileset1.png"),
        create_tile!("../../assets/test-tileset2.png"),
        create_sprite!("../../assets/test-sprite1.png", 0, 16),
    ];
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
    let mut top = Flex::column().with_child(Button::new("Save").on_click(move |ctx, _, _| {
        ctx.submit_command(Command::new(
            commands::SHOW_SAVE_PANEL,
            save_dialog_options.clone(),
            Target::Auto,
        ))
    }));
    let mut i = 0;
    let mut cur = Flex::row();
    for image in images {
        cur.add_child(ControllerHost::new(
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
        if i % 5 == 0 {
            top.add_child(cur);
            cur = Flex::row();
        }
    }
    if i % 5 != 0 {
        top.add_child(cur);
    }
    top
}

struct Delegate {
    scene: Arc<Mutex<(graphics::Tiles, Vec<ecs::EntityDesc>)>>,
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
            let scene = &self.scene.lock().unwrap();
            if let Err(e) = save_scene(&scene.0, &scene.1, file_info.path().to_str().unwrap()) {
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

fn main() {
    let scene: Arc<Mutex<(graphics::Tiles, Vec<ecs::EntityDesc>)>> = Default::default();
    let cur_selection: Arc<Mutex<Selection>> = Default::default();
    let scene_clone = Arc::clone(&scene);
    let cur_selection_clone = Arc::clone(&cur_selection);

    let (tx, rx): (Sender<()>, Receiver<()>) = mpsc::channel();
    thread::spawn(move || {
        AppLauncher::with_window(
            WindowDesc::new(|| build_ui())
                .window_size((200.0, 200.0))
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

    let mut last_click = (false, false, false);
    pollster::block_on(graphics::Graphics::new()).run(move |controller, _, _, _, _| {
        if let Ok(()) = rx.try_recv() {
            process::exit(0);
        }

        let scene = &mut scene.lock().unwrap();

        if controller.left_click {
            let world_x = (controller.cursor_x as f32 / graphics::PIXEL_SIZE as f32) + cx;
            let world_y = -(controller.cursor_y as f32 / graphics::PIXEL_SIZE as f32) + cy;
            let selection: &Selection = &cur_selection.lock().unwrap();
            match selection {
                Selection::Tile(tile) => {
                    let tile_x = (world_x as i64).div_euclid(graphics::TILE_SIZE as i64);
                    let tile_y = (world_y as i64).div_euclid(graphics::TILE_SIZE as i64);
                    *scene.0.get_mut(tile_x, tile_y) = (*tile, 0);
                    for x in -1..=1 {
                        for y in -1..=1 {
                            let (o_x, o_y) = (tile_x + x, tile_y + y);
                            let frame = calculate_tile_edges(o_x, o_y, &scene.0) as usize;
                            scene.0.get_mut(o_x, o_y).1 = frame;
                        }
                    }
                }
                Selection::Entity(construct) => {
                    if !last_click.0 {
                        scene.1.push(construct(world_x, world_y));
                    }
                }
            }
            last_click.0 = true;
        } else {
            last_click.0 = false;
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
            last_click.1 = true;
        } else {
            dc = None;
            last_click.1 = false;
        }

        let mut tile_batch: graphics::TileBatch = Default::default();
        for (coords, data) in scene.0 .0.iter() {
            for r in 0..graphics::CHUNK_SIZE {
                for c in 0..graphics::CHUNK_SIZE {
                    if data[r][c].0 != graphics::Tile::NoTile {
                        tile_batch[data[r][c].0 as usize].push((
                            data[r][c].1,
                            coords.0 * graphics::CHUNK_SIZE as i64 + r as i64,
                            coords.1 * graphics::CHUNK_SIZE as i64 + c as i64,
                        ));
                    }
                }
            }
        }

        let mut sprite_batch: graphics::SpriteBatch = Default::default();
        for entity_desc in scene.1.iter() {
            sprite_batch[entity_desc.get_sprite() as usize].push((
                0,
                entity_desc.get_pos().0,
                entity_desc.get_pos().1,
                1.0,
                1.0,
            ));
        }

        (sprite_batch, tile_batch, cx, cy, 0.0, 0.0)
    });
}
