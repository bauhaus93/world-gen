#[macro_use]
extern crate log;
extern crate env_logger;
extern crate chrono;

extern crate core;
extern crate world;

mod application;
mod application_error;

use std::io::Write;
use env_logger::{ Builder, fmt::Formatter };
use log::Record;

use crate::{application::Application, application_error::ApplicationError};

fn main() {
    const CONFIG_PATH: &'static str = "resources/default.yaml";

    init_custom_logger();

    let app = match Application::new(CONFIG_PATH) {
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
