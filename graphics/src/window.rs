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

use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::*,
};

use wgpu::util::DeviceExt;
use wgpu::*;

use super::controls::*;
use super::sprite::*;
use super::tiles::*;

pub struct Graphics {
    event_loop: EventLoop<()>,
    controller: Controller,
    window: Window,
    context: Context,
}

struct Context {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,
    render_pipeline: wgpu::RenderPipeline,
    vertex_buffers: wgpu::Buffer,
    textures: Vec<super::sprite::Texture>,
    texture_bind_groups: Vec<wgpu::BindGroup>,
    camera_buffer: wgpu::Buffer,
    camera_bind_group: wgpu::BindGroup,
    instance_buffer: wgpu::Buffer,
}

impl Graphics {
    pub async fn new() -> Self {
        env_logger::init();
        let event_loop = EventLoop::new();
        let controller = Controller::new(ControllerScheme::KeyboardMouse {
            jump_key: VirtualKeyCode::W,
            crouch_key: VirtualKeyCode::S,
            left_key: VirtualKeyCode::A,
            right_key: VirtualKeyCode::D,
        });
        let window = WindowBuilder::new()
            .build(&event_loop)
            .expect("Could not create a window.");
        let context = Context::new(&window).await;
        Graphics {
            event_loop,
            controller,
            window,
            context,
        }
    }

    pub fn run<
        F: FnMut(&Controller, f32, f32, f32, f32) -> (SpriteBatch, TileBatch, f32, f32, f32, f32)
            + 'static,
    >(
        mut self,
        mut tick: F,
    ) {
        let mut p_cx = 0.0;
        let mut p_cy = 0.0;
        let mut p_ax = 0.0;
        let mut p_ay = 0.0;
        self.event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Poll;
            match event {
                Event::MainEventsCleared => {
                    self.window.request_redraw();
                }
                Event::WindowEvent {
                    ref event,
                    window_id,
                } if window_id == self.window.id() => {
                    if !self
                        .controller
                        .process_window_event(event, &self.context.size)
                    {
                        match event {
                            WindowEvent::Resized(physical_size) => {
                                self.context.resize(*physical_size);
                            }
                            WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                                self.context.resize(**new_inner_size);
                            }
                            WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                            _ => {}
                        }
                    }
                }
                Event::RedrawRequested(window_id) if window_id == self.window.id() => {
                    let (sprites, tiles, cx, cy, ax, ay) =
                        tick(&self.controller, p_cx, p_cy, p_ax, p_ay);
                    match self.context.render(sprites, tiles, cx, cy) {
                        Ok(_) => {}
                        Err(wgpu::SurfaceError::Lost) => self.context.resize(self.context.size),
                        Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                        Err(e) => eprintln!("{:?}", e),
                    };
                    p_cx = cx;
                    p_cy = cy;
                    p_ax = ax;
                    p_ay = ay;
                }
                _ => {}
            };
        });
    }
}

impl Context {
    async fn new(window: &Window) -> Self {
        let size = window.inner_size();
        let instance = wgpu::Instance::new(wgpu::Backends::all());
        let surface = unsafe { instance.create_surface(window) };
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    features: wgpu::Features::empty(),
                    limits: wgpu::Limits::default(),
                    label: None,
                },
                None,
            )
            .await
            .unwrap();

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_preferred_format(&adapter).unwrap(),
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Immediate,
        };
        surface.configure(&device, &config);

        let (textures, texture_bind_groups, texture_bind_group_layouts) = create_textures!(
            &device,
            &queue,
            "../../assets/gen/test-tileset1.png",
            "../../assets/gen/test-sprite1.png",
            "../../assets/gen/test-sprite2.png"
        );

        let vertex_buffers = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffers"),
            contents: bytemuck::cast_slice(VERTICES),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let camera_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Camera buffer"),
            contents: bytemuck::cast_slice(&[0.0, 0.0]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let camera_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
                label: Some("camera_bind_group_layout"),
            });

        let camera_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &camera_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: camera_buffer.as_entire_binding(),
            }],
            label: Some("camera_bind_group"),
        });

        let shader = device.create_shader_module(&include_wgsl!("shader.wgsl"));
        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[&texture_bind_group_layouts[0], &camera_bind_group_layout],
                push_constant_ranges: &[],
            });
        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[
                    wgpu::VertexBufferLayout {
                        array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
                        step_mode: wgpu::VertexStepMode::Vertex,
                        attributes: &[
                            wgpu::VertexAttribute {
                                offset: 0,
                                shader_location: 0,
                                format: wgpu::VertexFormat::Float32x2,
                            },
                            wgpu::VertexAttribute {
                                offset: std::mem::size_of::<[f32; 2]>() as wgpu::BufferAddress,
                                shader_location: 1,
                                format: wgpu::VertexFormat::Float32x2,
                            },
                        ],
                    },
                    wgpu::VertexBufferLayout {
                        array_stride: std::mem::size_of::<super::sprite::Instance>()
                            as wgpu::BufferAddress,
                        step_mode: wgpu::VertexStepMode::Instance,
                        attributes: &[
                            wgpu::VertexAttribute {
                                offset: 0,
                                shader_location: 2,
                                format: wgpu::VertexFormat::Float32,
                            },
                            wgpu::VertexAttribute {
                                offset: std::mem::size_of::<f32>() as wgpu::BufferAddress,
                                shader_location: 3,
                                format: wgpu::VertexFormat::Float32,
                            },
                            wgpu::VertexAttribute {
                                offset: std::mem::size_of::<[f32; 2]>() as wgpu::BufferAddress,
                                shader_location: 4,
                                format: wgpu::VertexFormat::Float32,
                            },
                            wgpu::VertexAttribute {
                                offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                                shader_location: 5,
                                format: wgpu::VertexFormat::Float32,
                            },
                            wgpu::VertexAttribute {
                                offset: std::mem::size_of::<[f32; 4]>() as wgpu::BufferAddress,
                                shader_location: 6,
                                format: wgpu::VertexFormat::Float32,
                            },
                            wgpu::VertexAttribute {
                                offset: std::mem::size_of::<[f32; 5]>() as wgpu::BufferAddress,
                                shader_location: 7,
                                format: wgpu::VertexFormat::Float32,
                            },
                            wgpu::VertexAttribute {
                                offset: std::mem::size_of::<[f32; 6]>() as wgpu::BufferAddress,
                                shader_location: 8,
                                format: wgpu::VertexFormat::Float32,
                            },
                            wgpu::VertexAttribute {
                                offset: std::mem::size_of::<[f32; 7]>() as wgpu::BufferAddress,
                                shader_location: 9,
                                format: wgpu::VertexFormat::Float32,
                            },
                        ],
                    },
                ],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                }],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleStrip,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: None,
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        });

        let instance_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Instance Buffer"),
            contents: bytemuck::cast_slice(vec![0.0; 65536].as_ref()),
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
        });

        Self {
            surface,
            device,
            queue,
            config,
            size,
            render_pipeline,
            vertex_buffers,
            textures,
            texture_bind_groups,
            camera_buffer,
            camera_bind_group,
            instance_buffer,
        }
    }

    fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
        }
    }

    fn render(
        &mut self,
        sprites: SpriteBatch,
        tiles: TileBatch,
        cx: f32,
        cy: f32,
    ) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.0,
                            g: 0.0,
                            b: 0.0,
                            a: 1.0,
                        }),
                        store: true,
                    },
                }],
                depth_stencil_attachment: None,
            });
            self.queue.write_buffer(
                &self.camera_buffer,
                0,
                bytemuck::cast_slice(&[cx * PIXEL_SIZE as f32, cy * PIXEL_SIZE as f32]),
            );
            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_vertex_buffer(0, self.vertex_buffers.slice(..));
            render_pass.set_vertex_buffer(1, self.instance_buffer.slice(..));
            render_pass.set_bind_group(1, &self.camera_bind_group, &[]);

            let mut count: usize = 0;

            for i in 0..sprites.len() {
                if sprites[i].len() == 0 {
                    continue;
                }
                let instances: Vec<_> = sprites[i]
                    .iter()
                    .map(|(frame, x, y, w, h)| super::sprite::Instance {
                        texoffset: *frame as f32 / Sprite::frames(i) as f32,
                        texwidth: 1.0 / Sprite::frames(i) as f32,
                        x: *x * PIXEL_SIZE as f32,
                        y: *y * PIXEL_SIZE as f32,
                        w: (self.textures[i + 1].dimensions.0 as usize * PIXEL_SIZE) as f32
                            / Sprite::frames(i) as f32
                            * *w,
                        h: (self.textures[i + 1].dimensions.1 as usize * PIXEL_SIZE) as f32 * *h,
                        ww: self.size.width as f32 / 2.0,
                        wh: self.size.height as f32 / 2.0,
                    })
                    .collect();
                self.queue.write_buffer(
                    &self.instance_buffer,
                    (count * std::mem::size_of::<super::sprite::Instance>()) as u64,
                    bytemuck::cast_slice(instances.as_ref()),
                );
                render_pass.set_bind_group(0, &self.texture_bind_groups[i + NUM_TILES], &[]);
                render_pass.draw(0..4, (count as u32)..(count + instances.len()) as u32);
                count += instances.len();
            }

            for i in 0..tiles.len() {
                if tiles[i].len() == 0 {
                    continue;
                }
                let instances: Vec<_> = tiles[i]
                    .iter()
                    .map(|(version, x, y)| super::sprite::Instance {
                        texoffset: *version as f32 / NUM_TILE_VERSIONS as f32,
                        texwidth: 1.0 / NUM_TILE_VERSIONS as f32,
                        x: ((*x * PIXEL_SIZE + PIXEL_SIZE / 2) * TILE_SIZE) as f32,
                        y: ((*y * PIXEL_SIZE + PIXEL_SIZE / 2) * TILE_SIZE) as f32,
                        w: (self.textures[0].dimensions.0 as usize * PIXEL_SIZE) as f32
                            / NUM_TILE_VERSIONS as f32,
                        h: (self.textures[0].dimensions.1 as usize * PIXEL_SIZE) as f32,
                        ww: self.size.width as f32 / 2.0,
                        wh: self.size.height as f32 / 2.0,
                    })
                    .collect();
                self.queue.write_buffer(
                    &self.instance_buffer,
                    (count * std::mem::size_of::<super::sprite::Instance>()) as u64,
                    bytemuck::cast_slice(instances.as_ref()),
                );
                render_pass.set_bind_group(0, &self.texture_bind_groups[i], &[]);
                render_pass.draw(0..4, (count as u32)..(count + instances.len()) as u32);
                count += instances.len();
            }
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}
