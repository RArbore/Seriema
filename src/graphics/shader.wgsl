// 
// This file is part of game-testbed.
// game-testbed is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// any later version.
// game-testbed is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
// You should have received a copy of the GNU General Public License
// along with game-testbed. If not, see <https://www.gnu.org/licenses/>.
// 

struct Camera {
    x: f32;
    y: f32;
};
[[group(1), binding(0)]]
var<uniform> camera: Camera;

struct VertexInput {
    [[location(0)]] position: vec2<f32>;
    [[location(1)]] texcoord: vec2<f32>;
};

struct InstanceInput {
    [[location(2)]] texoffset: f32;
    [[location(3)]] texwidth: f32;
    [[location(4)]] x: f32;
    [[location(5)]] y: f32;
    [[location(6)]] w: f32;
    [[location(7)]] h: f32;
};

struct VertexOutput {
    [[builtin(position)]] clip_position: vec4<f32>;
    [[location(0)]] texcoord: vec2<f32>;
};

[[stage(vertex)]]
fn vs_main(
    model: VertexInput,
    instance: InstanceInput,
) -> VertexOutput {
    var out: VertexOutput;
    out.texcoord = vec2<f32>(model.texcoord.x * instance.texwidth + instance.texoffset, model.texcoord.y);
    out.clip_position = vec4<f32>(model.position.x * instance.w + instance.x - camera.x, model.position.y * instance.h + instance.y - camera.y, 0.0, 1.0);
    return out;
}

[[group(0), binding(0)]]
var frag_texture: texture_2d<f32>;
[[group(0), binding(1)]]
var frag_sampler: sampler;

[[stage(fragment)]]
fn fs_main(in: VertexOutput) -> [[location(0)]] vec4<f32> {
    return textureSample(frag_texture, frag_sampler, in.texcoord);
}
