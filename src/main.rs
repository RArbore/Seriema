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

mod ecs;
mod graphics;

fn main() {
    let mut world = ecs::World::new();
    for i in 0..10000 {
        let entity = world.add();
        world.insert(
            entity,
            ecs::Position {
                x: -2000.0 + ((i / 100) as f32) * 64.0,
                y: -2000.0 + ((i % 100) as f32) * 64.0,
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
    }
    world.systems.push(Box::new(
        ecs::update_pos as fn(&mut ecs::Timer, (&mut ecs::Position, &mut ecs::Velocity)),
    ));
    world
        .systems
        .push(Box::new(ecs::print_fps as fn(&mut ecs::Timer)));
    world.systems.push(Box::new(
        ecs::render_sprite as fn(&mut ecs::RenderBatchRes, (&mut ecs::Position, &mut ecs::Sprite)),
    ));
    graphics::Graphics::new().run(move || world.run());
}
