#[macro_use]
extern crate log;
extern crate env_logger;
extern crate chrono;

extern crate world_gen;

use std::io::Write;
use log::Record;
use env_logger::{ Builder, fmt::Formatter };

fn main() {
    const WINDOW_SIZE: (f64, f64) = (1024., 768.);

    init_custom_logger();

    let app = match world_gen::Application::new(WINDOW_SIZE) {
        Ok(app) => app,
        Err(e) => {
            error!("{}", e);
            return;
        }
    };
    match app.run() {
        Ok(_) => info!("Application exited successfully"),
        Err(e) => error!("{}", e)
    }
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
