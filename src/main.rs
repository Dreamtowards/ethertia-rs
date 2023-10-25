


fn main()
{
    init();

    // while true
    // {
    //     run_main_loop();
    // }

    destroy();
}

use byte_unit::Byte;
use sysinfo::{CpuExt, SystemExt, UserExt};
use winit::{
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::WindowBuilder,
};

fn init()
{
    use sysinfo::{NetworkExt, NetworksExt, ProcessExt, System, SystemExt};

// Please note that we use "new_all" to ensure that all list of
// components, network interfaces, disks and users are already
// filled!
    let mut sys = System::new_all();

// First we update all information of our `System` struct.
    sys.refresh_all();

// We display all disks' information:
    println!("=> disks:");
    for disk in sys.disks() {
        println!("{:?}", disk);
    }

// Network interfaces name, data received and data transmitted:
    println!("=> networks:");
    for (interface_name, data) in sys.networks() {
        println!("{}: {}/{} B", interface_name, data.received(), data.transmitted());
    }

// Components temperature:
    println!("=> components:");
    for component in sys.components() {
        println!("{:?}", component);
    }

    println!("=> system:");
// RAM and swap information:
    println!("total memory: {} bytes", sys.total_memory());
    println!("used memory : {} bytes", sys.used_memory());
    println!("total swap  : {} bytes", sys.total_swap());
    println!("used swap   : {} bytes", sys.used_swap());

// Display system information:
    println!("System name:             {:?}", sys.name());
    println!("System kernel version:   {:?}", sys.kernel_version());
    println!("System OS version:       {:?}", sys.os_version());
    println!("System host name:        {:?}", sys.host_name());

// Number of CPUs:
    println!("NB CPUs: {}", sys.cpus().len());

// Display processes ID, name na disk usage:
    for (pid, process) in sys.processes() {
        println!("[{}] {} {:?}", pid, process.name(), process.disk_usage());
    }


    sys.refresh_cpu(); // Refreshing CPU information.
    for cpu in sys.cpus() {
        println!("{}% {}, {} {} {}", cpu.cpu_usage(), cpu.name(), cpu.frequency(), cpu.brand(), cpu.vendor_id());
    }

    for user in sys.users()
    {
        println!("User {} {}, group: {:?}", user.id().to_string(), user.name(), user.groups());
    }




    use memory_stats::memory_stats;

    if let Some(usage) = memory_stats() {
        println!("Current physical memory usage: {}", byte_unit::Byte::from_bytes(usage.physical_mem as u128).get_appropriate_unit(false).to_string());
        println!("Current virtual memory usage: {}", byte_unit::Byte::from_bytes(usage.virtual_mem as u128).get_appropriate_unit(false).to_string());
    } else {
        println!("Couldn't get the current memory usage :(");
    }

    let n = std::thread::available_parallelism().unwrap().get();
    println!("Concurrency {}", n);
}
fn destroy()
{

}

fn run_main_loop()
{

}



// fn main()
// {
    // let path = native_dialog::FileDialog::new()
    //     .add_filter("PNG Image", &["png"])
    //     .add_filter("JPEG Image", &["jpg", "jpeg"])
    //     .show_open_single_file()
    //     .unwrap();
    //
    // let yes = native_dialog::MessageDialog::new()
    //     .set_type(native_dialog::MessageType::Info)
    //     .set_title("Do you want to open the file?")
    //     .set_text(&format!("{:#?}", path))
    //     .show_confirm()
    //     .unwrap();


//     let event_loop = EventLoop::new().unwrap();
//
//     let window = WindowBuilder::new()
//         .with_title("Test Title")
//         .with_inner_size(winit::dpi::LogicalSize::new(1280, 720))
//         .build(&event_loop)
//         .unwrap();
//
//     event_loop.run(move |event, elwt| {
//         println!("{event:?}");
//
//         match event {
//             Event::WindowEvent { event, window_id } if window_id == window.id() => match event {
//                 WindowEvent::CloseRequested => elwt.exit(),
//                 WindowEvent::RedrawRequested => {
//                     // Notify the windowing system that we'll be presenting to the window.
//                     window.pre_present_notify();
//
//                 }
//                 _ => (),
//             },
//             Event::AboutToWait => {
//                 window.request_redraw();
//             }
//
//             _ => (),
//         }
//     }).unwrap();
// }
