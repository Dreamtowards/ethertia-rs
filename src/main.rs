

use winit::{
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::WindowBuilder,
};

fn main()
{
    

    env_logger::builder()
        .format(|buf, record| {

            let mut dim = buf.style();
            dim.set_dimmed(true);

            use std::io::Write;
            writeln!(buf,
                     "{}{}{}{}: {}",
                     dim.value(chrono::Local::now().format("[%Y-%m-%d %H:%M:%S%.6f]")),
                     dim.value(format!("[main/")),
                     buf.default_level_style(record.level()).value(record.level()),
                     dim.value(format!("][{} {}:{}]", record.target(), record.file().unwrap(), record.line().unwrap())),
                     record.args())
        })
        .init();

    log::trace!("Trace Sth");
    log::debug!("Debug Sth");
    log::info!("Info Sth");
    log::warn!("Some Warn");
    log::error!("Some Error");

    println!("SSS");

    return;

    let event_loop = EventLoop::new().unwrap();

    let window = WindowBuilder::new()
        .with_title("Test Title")
        .with_inner_size(winit::dpi::LogicalSize::new(1280, 720))
        .build(&event_loop)
        .unwrap();

    event_loop.run(move |event, elwt| {
        println!("{event:?}");

        match event {
            Event::WindowEvent { event, window_id } if window_id == window.id() => match event {
                WindowEvent::CloseRequested => elwt.exit(),
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
