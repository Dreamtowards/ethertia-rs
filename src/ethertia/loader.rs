

fn sys_info()
{
    // 2023-10-25T10:16:33.428130Z  INFO bevy_render::renderer: AdapterInfo { name: "NVIDIA GeForce RTX 2060 SUPER", vendor: 4318, device: 7942, device_type: DiscreteGpu, driver: "NVIDIA", driver_info: "537.13", backend: Vulkan }
    // 2023-10-25T10:16:33.814679Z  INFO bevy_diagnostic::system_information_diagnostics_plugin::internal: SystemInfo { os: "Windows 11 Pro for Workstations", kernel: "22621", cpu: "Intel(R) Xeon(R) CPU E5-2690 v4 @ 2.60GHz", core_count: "14", memory: "31.8 GiB" }
    // 2023-10-25T10:16:33.837185Z  INFO bevy_input::gamepad: Gamepad { id: 0 } Connected
    use sysinfo::{CpuExt, SystemExt};

    let mut sys = sysinfo::System::new();
    sys.refresh_cpu();
    sys.refresh_memory();

    let os = sys.long_os_version().unwrap();
    let kernel_ver = sys.kernel_version().unwrap();
    let cpu_name = sys.global_cpu_info().brand().trim().to_string();
    const BYTES_TO_GIB: f64 = 1.0 / 1024.0 / 1024.0 / 1024.0;

    println!("os: {} kernel {}; cpu: {}, {}C/{}T; mem: {:.1} / {:.1} GiB",
             os, kernel_ver,
             cpu_name, sys.physical_core_count().unwrap(), std::thread::available_parallelism().unwrap().get(),
             sys.used_memory() as f64 * BYTES_TO_GIB,
             sys.total_memory() as f64 * BYTES_TO_GIB
    );


    use memory_stats::memory_stats;

    if let Some(usage) = memory_stats() {
        println!("Current physical memory usage: {}", byte_unit::Byte::from_bytes(usage.physical_mem as u128).get_appropriate_unit(false).to_string());
        println!("Current virtual memory usage: {}", byte_unit::Byte::from_bytes(usage.virtual_mem as u128).get_appropriate_unit(false).to_string());
    } else {
        println!("Couldn't get the current memory usage :(");
    }

    let n = std::thread::available_parallelism().unwrap().get();
    println!("Concurrency {}", n);


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

}