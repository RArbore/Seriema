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

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    position: [f32; 2],
    texcoord: [f32; 2],
}

pub const VERTICES: &[Vertex] = &[
    Vertex {
        position: [0.0, 0.5],
        texcoord: [0.0, 1.0],
    },
    Vertex {
        position: [-0.5, -0.5],
        texcoord: [1.0, 0.0],
    },
    Vertex {
        position: [0.5, -0.5],
        texcoord: [0.0, 0.0],
    },
];
