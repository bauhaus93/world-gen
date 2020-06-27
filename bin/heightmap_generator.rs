#[macro_use]
extern crate log;
extern crate byteorder;
extern crate chrono;
extern crate env_logger;

extern crate core;
extern crate world;

use env_logger::{fmt::Formatter, Builder};
use log::Record;
use std::io::Write;
use std::path::Path;

use std::time::Instant;

use core::{Point2f, Seed};
use world::erosion::Model;
use world::noise::presets::get_default_noise;
use world::HeightMap;
use world::CHUNK_SIZE;

fn main() {
    const FILENAME: &'static str = "heightmap.dat";
    init_custom_logger();

    info!("Started heightmap generation");
    let start = Instant::now();

    let seed = Seed::from_entropy();
    info!("Seed = {}", seed);

    let size = CHUNK_SIZE * 5;
    info!("Heightmap size = {}x{}", size, size);

    let noise = get_default_noise(seed);

    let heightmap = HeightMap::from_noise(Point2f::from_scalar(0.), size, 1, noise.as_ref());

    let erosion_heightmap: HeightMap = Model::from(heightmap).run(5000, seed).into();

    match erosion_heightmap.into_file(&Path::new(FILENAME)) {
        Ok(_) => info!("Successfully written heightmap to'{}'", FILENAME),
        Err(e) => error!("Could not write heightmap into '{}': {}", FILENAME, e),
    }

    let work_duration = start.elapsed().as_secs() as u32;

    info!("Finished heightmap generation in {}s", work_duration);
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
