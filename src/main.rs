
use std::iter;

use winit::{
    event::*,
    event_loop::EventLoop,
    window::{Window, WindowBuilder},
};
use winit::raw_window_handle::{HasRawWindowHandle, HasWindowHandle};

struct State  {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    surface_config: wgpu::SurfaceConfiguration,
    window_size: winit::dpi::PhysicalSize<u32>,
    window: Window,
}

impl State
{
    async fn new(window: Window) -> Self
    {

        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
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

        Self {
            surface,
            device,
            queue,
            surface_config,
            window_size,
            window
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
            let _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color{ r: 0.1, g: 0.2, b: 0.3, a: 1.0}),
                        store: true,
                    }
                })],
                depth_stencil_attachment: None,
            });
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
                WindowEvent::CloseRequested //|
                // WindowEvent::KeyboardInput {
                //     event: KeyEvent {
                //         state: ElementState::Pressed,
                //         physical_key: PhysicalKey::Code(KeyCode::Escape),
                //         ..
                //     },
                //     ..
                // }
                => elwt.exit(),
                WindowEvent::Resized(physical_size) => {
                    state.resize(physical_size);
                }
                // WindowEvent::ScaleFactorChanged { inner_size_writer, .. } => {
                //     // new_inner_size is &&mut so we have to dereference it twice
                //     state.resize(inner_size_writer.);
                // }
                WindowEvent::RedrawRequested => {
                    // Notify the windowing system that we'll be presenting to the window.
                    state.window.pre_present_notify();

                    //state.update();
                    match state.render() {
                        Ok(_) => {}
                        // Reconfigure the surface if lost
                        Err(wgpu::SurfaceError::Lost) => state.resize(state.window_size),
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

    let str = String::from("SomeS");

    let s2 = str.clone();

    pollster::block_on(run());

    //
    // let n = 6;
    //
    // let arr = [n; 4];
    //
    // for ele in arr.iter()
    // {
    //     println!("SSS {}", arr.len());
    //
    // }
    //
    // for i in (3..5).rev()
    // {
    //     println!("{}", i)
    // }
    //


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
