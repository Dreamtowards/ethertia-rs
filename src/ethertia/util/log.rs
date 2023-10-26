

pub fn init_log()
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
                     dim.value(format!("][{} {}:{}]", record.target(), std::path::Path::new(record.file()).file_name().unwrap().to_str().unwrap(), record.line().unwrap())),
                     record.args())
        })
        .init();
}