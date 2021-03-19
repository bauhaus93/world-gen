#[macro_use]
extern crate log;
extern crate chrono;
extern crate env_logger;

extern crate core;
extern crate world;

use core::{Config, Core, State};
use env_logger::{fmt::Formatter, Builder};
use log::Record;
use std::io::Write;
use world::WorldState;

fn main() {
    const CONFIG_PATH: &'static str = "resources/default.yaml";

    init_custom_logger();

    let config = match Config::read(CONFIG_PATH) {
        Ok(c) => c,
        Err(e) => {
            error!("Could not read config: {}", e);
            return;
        }
    };

    let core = match Core::new(&config) {
        Ok(c) => c,
        Err(e) => {
            error!("Could not initialize core: {}", e);
            return;
        }
    };

    let state: Box<dyn State> = match WorldState::new(&config) {
        Ok(s) => Box::new(s),
        Err(e) => {
            error!("Could not created world state: {}", e);
            return;
        }
    };
    core.run(state, 30);
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
