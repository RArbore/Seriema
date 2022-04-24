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

    let scene: (graphics::Tiles, Vec<Box<dyn ecs::EntityDesc>>) =
        bincode::deserialize(&std::fs::read("assets/testscene.bin").unwrap()).unwrap();

    world.resources.tiles = scene.0;
    for entity in scene.1 {
        entity.construct(&mut world);
    }

    pollster::block_on(graphics::Graphics::new()).run(move |controller, p_cx, p_cy, p_ax, p_ay| {
        world.run(controller.get_game_input(p_cx, p_cy, p_ax, p_ay))
    });
}
