#[macro_use]
extern crate log;
extern crate chrono;
extern crate env_logger;

extern crate core;
extern crate world;

mod application;
mod application_error;

use env_logger::{fmt::Formatter, Builder};
use log::Record;
use std::io::Write;

use crate::{application::Application, application_error::ApplicationError};

fn main() {
    const CONFIG_PATH: &'static str = "default.yaml";

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
        Err(e) => error!("{}", e),
    }
}

fn init_custom_logger() {
    let format = |buf: &mut Formatter, record: &Record| {
        let time = chrono::Local::now();
        writeln!(
            buf,
            "[{} {:-5}] {}",
            time.format("%Y-%m-%d %H:%M:%S"),
            record.level(),
            record.args()
        )
    };
    Builder::from_default_env().format(format).init();
}
