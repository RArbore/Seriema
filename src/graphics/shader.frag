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

Texture2D shaderTexture;
SamplerState sample;

struct FragInput {
    float4 pos : VPOS;
    float2 texc : TEXCOORD0;
};

struct FragOutput {
    float4 color : COLOR0;
};

FragOutput fs_main(FragInput input) : SV_TARGET {
    FragOutput output;
    output.color = shaderTexture.Sample(sample, input.texc);
    return output;
}
