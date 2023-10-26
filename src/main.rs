
use winit::{
    event::*,
    event_loop::EventLoop,
    window::WindowBuilder,
};
use winit::event::WindowEvent::KeyboardInput;
use winit::keyboard::{KeyCode, PhysicalKey};

fn main()
{
    env_logger::init();

    let str = String::from("SomeS");

    let s2 = str.clone();

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

    log::trace!("Trace Sth");
    log::debug!("Debug Sth");
    log::info!("Info Sth");
    log::warn!("Some Warn");
    log::error!("Some Error");


    let event_loop = EventLoop::new().unwrap();

    let window = WindowBuilder::new()
        .with_title("Test Title")
        .with_inner_size(winit::dpi::LogicalSize::new(1280, 720))
        .build(&event_loop)
        .unwrap();

    event_loop.run(move |event,elwt| {

        match event {
            Event::WindowEvent { event, window_id } if window_id == window.id() => match event {
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
                WindowEvent::RedrawRequested => {
                    // Notify the windowing system that we'll be presenting to the window.
                    window.pre_present_notify();

                }
                _ => (),
            },
            Event::AboutToWait => {
                window.request_redraw();
            }

            _ => (),
        }
    }).unwrap();

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
