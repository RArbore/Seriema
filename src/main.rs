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

extern crate ecs;
extern crate graphics;

fn main() {
    let mut world = ecs::World::new();
    /*
        for i in 0..10201 {
        let entity = world.add();
        world.insert(
        entity,
        ecs::AABB {
        x: -2000.0 + ((i / 101) as f32) * 64.0,
        y: -2000.0 + ((i % 101) as f32) * 64.0,
    },
    );
        world.insert(entity, ecs::Velocity { x: 100.0, y: 0.0 });
        world.insert(
        entity,
        ecs::Sprite {
        sprite: graphics::sprite::Sprite::TestSprite1,
        frame: i % 2,
        width: 4.0,
        height: 4.0,
    },
    );
        world.insert(entity, ecs::Player {});
    }
         */

    let entity = world.add();
    world.insert(
        entity,
        ecs::AABB {
            x: 32.0,
            y: 32.0,
            w: 16.0,
            h: 16.0,
            last: ecs::Correction::None,
        },
    );
    world.insert(entity, ecs::Velocity { x: 0.0, y: 0.0 });
    world.insert(
        entity,
        ecs::Sprite {
            sprite: graphics::sprite::Sprite::TestSprite1,
            frame: 0,
            width: 1.0,
            height: 1.0,
        },
    );
    world.insert(entity, ecs::Player { can_jump: 0.0 });

    world.systems.push(Box::new(
        ecs::update_aabb
            as fn(&mut ecs::Timer, &mut graphics::Tiles, &mut ecs::AABB, &mut ecs::Velocity),
    ));
    world
        .systems
        .push(Box::new(ecs::print_fps as fn(&mut ecs::Timer)));
    world.systems.push(Box::new(
        ecs::render_sprite as fn(&mut ecs::SpriteBatchRes, &mut ecs::AABB, &mut ecs::Sprite),
    ));
    world.systems.push(Box::new(
        ecs::player_system
            as fn(
                &mut ecs::Timer,
                &mut graphics::GameInput,
                &mut (f32, f32),
                &mut (f32, f32),
                &mut ecs::AABB,
                &mut ecs::Velocity,
                &mut ecs::Sprite,
                &mut ecs::Player,
            ),
    ));

    world.resources.tiles.insert(
        (0, 0),
        [[(graphics::tiles::Tile::NoTile, 0); graphics::tiles::CHUNK_SIZE];
            graphics::tiles::CHUNK_SIZE],
    );
    for i in 0..16 {
        world.resources.tiles.get_mut(&(0, 0)).unwrap()[i][0] =
            (graphics::tiles::Tile::TestTile1, 0xFF);
        world.resources.tiles.get_mut(&(0, 0)).unwrap()[0][i] =
            (graphics::tiles::Tile::TestTile1, 0xFF);
        world.resources.tiles.get_mut(&(0, 0)).unwrap()[15][i] =
            (graphics::tiles::Tile::TestTile1, 0xFF);
    }

    pollster::block_on(graphics::Graphics::new()).run(move |controller, p_cx, p_cy, p_ax, p_ay| {
        world.run(controller.get_game_input(p_cx, p_cy, p_ax, p_ay))
    });
}
