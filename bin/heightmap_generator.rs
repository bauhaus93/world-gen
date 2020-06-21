#[macro_use]
extern crate log;
extern crate env_logger;
extern crate chrono;

extern crate core;
extern crate world;

use std::io::Write;
use std::time::Instant;
use env_logger::{ Builder, fmt::Formatter };
use log::Record;


fn main() {
    const CONFIG_PATH: &'static str = "resources/default.yaml";
    init_custom_logger();

    info!("Started heightmap generation");
    let start = Instant::now();


    let work_duration = start.elapsed().as_secs() as u32;

    info!("Finished heightmap generation in {}s", work_duration);
}

fn init_custom_logger() {
    let format = |buf: &mut Formatter , record: &Record| {
        let time = chrono::Local::now();
        writeln!(buf, "[{} {:-5}] {}", time.format("%Y-%m-%d %H:%M:%S"), record.level(), record.args()) 
    };
    Builder::from_default_env()
        .format(format)
        .init();
}
