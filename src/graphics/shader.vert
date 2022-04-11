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

struct Camera {
    float x;
    float y;
};

cbuffer UBO : register(b0) {
    Camera camera : packoffset(c0);
};

struct VertexInput {
    float2 pos : TEXCOORD0;
    float2 texc : TEXCOORD1;
    float texo : TEXCOORD2;
    float texw : TEXCOORD3;
    float x : TEXCOORD4;
    float y : TEXCOORD5;
    float w : TEXCOORD6;
    float h : TEXCOORD7;
    float ww : TEXCOORD8;
    float wh : TEXCOORD9;
};

struct VertexOutput {
    float4 pos : POSITION;
    float2 texc : TEXCOORD0;
};

VertexOutput vs_main(VertexInput input) {
    VertexOutput result;
    result.pos = float4(input.pos.x * input.w / input.ww + input.x / input.ww - camera.x / input.ww, input.pos.y * input.h / input.wh + input.y / input.wh - camera.y / input.wh, 0.0, 1.0);
    result.texc = float2(input.pos.x * input.texw + input.texo, input.texc.y);
    return result;
}
