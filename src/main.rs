use wgpu::util::DeviceExt;
use wgpu::vertex_attr_array;
use winit::{
    event::*,
    event_loop::EventLoop,
    window::{Window, WindowBuilder},
};

struct State  {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    surface_config: wgpu::SurfaceConfiguration,
    window_size: winit::dpi::PhysicalSize<u32>,
    window: Window,
    render_pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    red: f64
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct Vertex {
    position: [f32; 3],
    color: [f32; 3],
}

const VERTICES: &[Vertex] = &[
    Vertex { position: [0.0, 0.5, 0.0], color: [1.0, 0.0, 0.0] },
    Vertex { position: [-0.5, -0.5, 0.0], color: [0.0, 1.0, 0.0] },
    Vertex { position: [0.5, -0.5, 0.0], color: [0.0, 0.0, 1.0] },
];

impl State
{
    async fn new(window: Window) -> Self
    {

        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::VULKAN,
            dx12_shader_compiler: Default::default()
        });

        let surface = unsafe { instance.create_surface(&window) }.unwrap();

        let adapter = instance.request_adapter(
            &wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            }
        ).await.unwrap();

        let (device, queue) = adapter.request_device(
            &wgpu::DeviceDescriptor {
                features: wgpu::Features::empty(),
                limits: wgpu::Limits::default(),
                label: None
            },
            None
        ).await.unwrap();

        let surface_caps = surface.get_capabilities(&adapter);

        let surface_format = surface_caps.formats.iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_caps.formats[0]);

        let window_size = window.inner_size();

        let surface_config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: window_size.width,
            height: window_size.height,
            present_mode: surface_caps.present_modes[0],  // PresentMode::Fifo
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
        };
        surface.configure(&device, &surface_config);

        // wgpu::include_wgsl!()
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
        });

        let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("PLayout"),
            bind_group_layouts: &[],
            push_constant_ranges: &[]
        });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("RP"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: surface_config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
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


        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
           label: Some("VBUF"),
            contents: bytemuck::cast_slice(VERTICES),
            usage: wgpu::BufferUsages::VERTEX,
        });
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &vertex_attr_array![0 => Float32x3, 1 => Float32x3],
            // &[
            //     wgpu::VertexAttribute {
            //         offset: 0,
            //         shader_location: 0,
            //         format: wgpu::VertexFormat::Float32x3,
            //     },
            //     wgpu::VertexAttribute {
            //         offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
            //         shader_location: 1,
            //         format: wgpu::VertexFormat::Float32x3,
            //     }
            // ]
        };


        Self {
            surface,
            device,
            queue,
            surface_config,
            window_size,
            window,
            render_pipeline,
            red: 1.0,
        }
    }

    fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        //if new_size.width > 0 && new_size.height > 0 {
        self.window_size = new_size;
        self.surface_config.width = new_size.width;
        self.surface_config.height = new_size.height;
        self.surface.configure(&self.device, &self.surface_config);
    }

    fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });

        {
            // {} block borrows the encoder.
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color{ r: self.red, g: 0.2, b: 0.3, a: 1.0}),
                        store: true,
                    }
                })],
                depth_stencil_attachment: None,
            });

            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.draw(0..3, 0..1);
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}

async fn run()
{
    let event_loop = EventLoop::new().unwrap();

    let window = WindowBuilder::new()
        .with_title("Test Title")
        .with_inner_size(winit::dpi::LogicalSize::new(1280, 720))
        .build(&event_loop)
        .unwrap();


    let mut state = State::new(window).await;


    event_loop.run(move |event,elwt| {

        match event {
            Event::WindowEvent {
                event,
                window_id
            } if window_id == state.window.id() => match event {
                WindowEvent::CloseRequested => elwt.exit(),
                WindowEvent::CursorMoved { position, .. } => {
                    state.red = position.x / 100.0;
                }
                WindowEvent::Resized(physical_size) => {
                    state.resize(physical_size);
                }
                // WindowEvent::ScaleFactorChanged { inner_size_writer, .. } => {
                //     // new_inner_size is &&mut so we have to dereference it twice
                //     state.resize(inner_size_writer.);
                // }
                WindowEvent::RedrawRequested => {
                    // Notify the windowing system that we'll be presenting to the window.
                    // state.window.pre_present_notify();

                    //state.update();
                    match state.render() {
                        Ok(_) => {}
                        // Reconfigure the surface if lost
                        // Err(wgpu::SurfaceError::Lost) => state.resize(state.window_size),
                        // All other errors (Outdated, Timeout) should be resolved by the next frame
                        Err(e) => eprintln!("{:?}", e),
                    }
                }
                _ => (),
            },
            Event::AboutToWait => {
                state.window.request_redraw();
            }

            _ => (),
        }
    }).unwrap();

}

fn main()
{
    env_logger::init();



    pollster::block_on(run());

}



//
// fn init()
// {
//
//
// }
// fn destroy()
// {
//
// }
//
// fn run_main_loop()
// {
//
// }
