extern crate byteorder;
extern crate chrono;
extern crate ncurses;
extern crate rand;

extern crate core;
extern crate world;

use rand::rngs::SmallRng;
use std::io::Write;
use std::path::Path;
use std::time::{Instant, Duration};
use std::thread::sleep;

use core::{Point2f, Point2i, Seed};
use world::erosion::Model;
use world::noise::{presets::get_default_noise, Noise, NoiseBuilder};
use world::HeightMap;

fn load_ncurses() {
    ncurses::initscr();
    ncurses::raw();
    ncurses::keypad(ncurses::stdscr(), true);
    ncurses::noecho();
    ncurses::curs_set(ncurses::CURSOR_VISIBILITY::CURSOR_INVISIBLE);
}

fn quit_ncurses() {
    ncurses::endwin();
}

fn main() {
    load_ncurses();

    let start = Instant::now();
    let seed = Seed::from_entropy();
    let size = 30;
    let noise = get_default_noise(seed);
    let heightmap = HeightMap::from_noise(Point2f::from_scalar(0.), size, 1, noise.as_ref());

    let mut rng: SmallRng = seed.into();
    let mut model = Model::from(heightmap);
    for i in 0..500 {
        for y in 0..size {
            for x in 0..size {
                ncurses::mv(y, 6 * x);
                ncurses::printw(&format!("{:4.2}", model.get_water_height(Point2i::new(x, y))));

                ncurses::mv(40 + y, 6 * x);
                ncurses::printw(&format!("{:4}", model.get_terrain_height(Point2i::new(x, y)) as i32));
            }
        }
        model = model.run(1, Seed::from_rng(&mut rng));
        ncurses::refresh();
        sleep(Duration::from_millis(100));
    }

    let work_duration = start.elapsed().as_secs() as u32;

    ncurses::getch();
    quit_ncurses();
}
