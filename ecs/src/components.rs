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

use super::world::*;

extern crate graphics;

pub trait Component {
    fn get_host_vec(components: &mut Components) -> &mut Vec<Option<Self>>
    where
        Self: Sized;
}

#[derive(Debug)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

impl Component for Position {
    fn get_host_vec(components: &mut Components) -> &mut Vec<Option<Position>> {
        &mut components.positions
    }
}

#[derive(Debug)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

impl Component for Velocity {
    fn get_host_vec(components: &mut Components) -> &mut Vec<Option<Velocity>> {
        &mut components.velocities
    }
}

#[derive(Debug)]
pub struct Sprite {
    pub sprite: graphics::sprite::Sprite,
    pub frame: usize,
    pub width: f32,
    pub height: f32,
}

impl Component for Sprite {
    fn get_host_vec(components: &mut Components) -> &mut Vec<Option<Sprite>> {
        &mut components.sprites
    }
}

#[derive(Debug)]
pub struct Player {}

impl Component for Player {
    fn get_host_vec(components: &mut Components) -> &mut Vec<Option<Player>> {
        &mut components.players
    }
}
