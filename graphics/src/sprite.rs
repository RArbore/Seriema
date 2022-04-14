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

use wgpu::{BindGroup, BindGroupLayout, Device};

use image::GenericImageView;
use image::ImageResult;

pub const NUM_TEXTURES: usize = 2;

#[repr(usize)]
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum Sprite {
    TestSprite1,
    TestSprite2,
}

impl Sprite {
    pub fn frames(s: usize) -> usize {
        match unsafe { std::mem::transmute(s) } {
            Sprite::TestSprite1 => 2,
            Sprite::TestSprite2 => 1,
        }
    }
}

pub type SpriteBatch = Vec<Vec<(usize, f32, f32, f32, f32)>>;

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Instance {
    pub texoffset: f32,
    pub texwidth: f32,
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub ww: f32,
    pub wh: f32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    position: [f32; 2],
    texcoord: [f32; 2],
}

pub const VERTICES: &[Vertex] = &[
    Vertex {
        position: [-0.5, -0.5],
        texcoord: [0.0, 1.0],
    },
    Vertex {
        position: [0.5, -0.5],
        texcoord: [1.0, 1.0],
    },
    Vertex {
        position: [-0.5, 0.5],
        texcoord: [0.0, 0.0],
    },
    Vertex {
        position: [0.5, 0.5],
        texcoord: [1.0, 0.0],
    },
];

pub struct Texture {
    pub texture: wgpu::Texture,
    pub view: wgpu::TextureView,
    pub sampler: wgpu::Sampler,
    pub dimensions: (u32, u32),
}

impl Texture {
    pub fn from_bytes(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        bytes: &[u8],
        label: Option<&str>,
    ) -> ImageResult<Self> {
        let img = image::load_from_memory(bytes)?;
        Self::from_image(device, queue, &img, label)
    }

    pub fn from_image(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        img: &image::DynamicImage,
        label: Option<&str>,
    ) -> ImageResult<Self> {
        let rgba = img.to_rgba8();
        let dimensions = img.dimensions();

        let size = wgpu::Extent3d {
            width: dimensions.0,
            height: dimensions.1,
            depth_or_array_layers: 1,
        };
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label,
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
        });

        queue.write_texture(
            wgpu::ImageCopyTexture {
                aspect: wgpu::TextureAspect::All,
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
            },
            &rgba,
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: std::num::NonZeroU32::new(4 * dimensions.0),
                rows_per_image: std::num::NonZeroU32::new(dimensions.1),
            },
            size,
        );

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Nearest,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        Ok(Self {
            texture,
            view,
            sampler,
            dimensions,
        })
    }
}

pub fn create_texture_bind_group(
    textures: &[&Texture],
    device: &Device,
) -> (BindGroup, BindGroupLayout) {
    let mut repeated_pattern = vec![];
    for i in 0..textures.len() {
        repeated_pattern.push(wgpu::BindGroupLayoutEntry {
            binding: (2 * i) as u32,
            visibility: wgpu::ShaderStages::FRAGMENT,
            ty: wgpu::BindingType::Texture {
                multisampled: false,
                view_dimension: wgpu::TextureViewDimension::D2,
                sample_type: wgpu::TextureSampleType::Float { filterable: true },
            },
            count: None,
        });
        repeated_pattern.push(wgpu::BindGroupLayoutEntry {
            binding: (2 * i + 1) as u32,
            visibility: wgpu::ShaderStages::FRAGMENT,
            ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
            count: None,
        });
    }
    let texture_bind_group_layout =
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &repeated_pattern[..],
            label: Some("texture_bind_group_layout"),
        });

    let mut repeated_pattern = vec![];
    for i in 0..textures.len() {
        repeated_pattern.push(wgpu::BindGroupEntry {
            binding: (2 * i) as u32,
            resource: wgpu::BindingResource::TextureView(&textures[i].view),
        });
        repeated_pattern.push(wgpu::BindGroupEntry {
            binding: (2 * i + 1) as u32,
            resource: wgpu::BindingResource::Sampler(&textures[i].sampler),
        });
    }
    let texture_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        layout: &texture_bind_group_layout,
        entries: &repeated_pattern[..],
        label: Some("diffuse_bind_group"),
    });
    (texture_bind_group, texture_bind_group_layout)
}

macro_rules! create_textures {
    ($a:expr, $b:expr, $($x:literal),+) => {
        {
            let mut textures_vec: Vec<super::sprite::Texture> = vec![];
            let mut bind_groups_vec: Vec<BindGroup> = vec![];
            let mut bind_group_layouts_vec: Vec<BindGroupLayout> = vec![];
            $(
                textures_vec.push(super::sprite::Texture::from_bytes($a, $b, include_bytes!($x), None).unwrap());
                let (group, layout) = create_texture_bind_group(&[textures_vec.last().unwrap()], $a);
                bind_groups_vec.push(group);
                bind_group_layouts_vec.push(layout);
            )*
                (textures_vec, bind_groups_vec, bind_group_layouts_vec)
        }
    }
}
