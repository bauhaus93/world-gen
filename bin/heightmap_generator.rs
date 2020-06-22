#[macro_use]
extern crate log;
extern crate byteorder;
extern crate chrono;
extern crate env_logger;

extern crate core;
extern crate world;

use byteorder::{LittleEndian, WriteBytesExt};
use env_logger::{fmt::Formatter, Builder};
use log::Record;
use std::collections::BTreeMap;
use std::fs::File;
use std::io;
use std::io::{BufWriter, Write};

use std::time::Instant;

use core::{Point2f, Point2i, Seed};
use world::noise::{Noise, NoiseBuilder};

type HeightMap = BTreeMap<Point2i, f32>;

fn write_heightmap(
    heightmap: HeightMap,
    size: Point2i,
    writer: &mut impl Write,
) -> Result<(), io::Error> {
    info!("Writing heightmap to file...");
    writer.write_i32::<LittleEndian>(size[0])?;
    writer.write_i32::<LittleEndian>(size[1])?;
    for y in 0..size[1] {
        for x in 0..size[0] {
            let h = heightmap.get(&Point2i::new(x, y)).unwrap_or(&0.);
            writer.write_f32::<LittleEndian>(*h)?;
            if (y * size[0] + x) % (size[0] * size[1] / 20) == 0 {
                info!(
                    "Progress: {:2}%",
                    (100. * (y * size[0] + x) as f32 / (size[0] * size[1]) as f32).round() as i32
                )
            }
        }
    }
    info!("Written heightmap to file!");
    Ok(())
}

fn heightmap_from_noise(noise: &dyn Noise, size: Point2i) -> HeightMap {
    info!("Creating heightmap from noise...");
    let mut heightmap = HeightMap::new();
    for y in 0..size[1] {
        for x in 0..size[0] {
            let h = noise.get_noise(Point2f::new(x as f32, y as f32));
            heightmap.insert(Point2i::new(x, y), h);
            if (y * size[0] + x) % (size[0] * size[1] / 20) == 0 {
                info!(
                    "Progress: {:2}%",
                    (100. * (y * size[0] + x) as f32 / (size[0] * size[1]) as f32).round() as i32
                )
            }
        }
    }
    info!("Created heightmap from noise!");
    heightmap
}

fn main() {
    const FILENAME: &'static str = "heightmap.dat";
    init_custom_logger();

    info!("Started heightmap generation");
    let start = Instant::now();

    let seed = Seed::from_entropy();
    info!("Seed = {}", seed);

    let size = Point2i::from_scalar(10000);
    info!("Heightmap size = {}", size);

    let noise = NoiseBuilder::new()
        .seed(seed)
        .octaves(6)
        .scale(1e-3)
        .roughness(0.5)
        .range([0., 100.])
        .finish();

    let heightmap = heightmap_from_noise(noise.as_ref(), size);

    match File::create(FILENAME) {
        Ok(f) => match write_heightmap(heightmap, size, &mut BufWriter::new(f)) {
            Ok(_) => info!("Saved heightmap in '{}'", FILENAME),
            Err(e) => error!("Could not save heightmap in '{}': {}", FILENAME, e),
        },
        Err(e) => {
            error!("Could not create file '{}': {}", FILENAME, e);
        }
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
