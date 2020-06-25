extern crate byteorder;
extern crate chrono;
extern crate ncurses;
extern crate rand;

extern crate core;
extern crate world;

use rand::rngs::SmallRng;
use std::io::Write;
use std::path::Path;
use std::thread::sleep;
use std::time::{Duration, Instant};

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
    ncurses::start_color();
    ncurses::init_pair(1, ncurses::COLOR_WHITE, ncurses::COLOR_BLUE);
    ncurses::init_pair(2, ncurses::COLOR_WHITE, ncurses::COLOR_CYAN);
    ncurses::init_pair(3, ncurses::COLOR_WHITE, ncurses::COLOR_GREEN);
    ncurses::init_pair(4, ncurses::COLOR_WHITE, ncurses::COLOR_MAGENTA);
    ncurses::nodelay(ncurses::stdscr(), true);
}

fn quit_ncurses() {
    ncurses::endwin();
}

fn print_water_height(p: Point2i, offset: Point2i, width: i32, model: &Model) {
    ncurses::mv(offset[1] + p[1], offset[0] + width * p[0]);
    let w = model.get_water_height(p);
    if w > 5. {
        ncurses::attron(ncurses::COLOR_PAIR(1));
    } else if w > 0. {
        ncurses::attron(ncurses::COLOR_PAIR(2));
    }
    ncurses::printw(&format!("{:3.1}", w));

    ncurses::attroff(ncurses::COLOR_PAIR(1));
    ncurses::attroff(ncurses::COLOR_PAIR(2));
}

fn print_terrain_height(p: Point2i, offset: Point2i, width: i32, model: &Model) {
    ncurses::mv(offset[1] + p[1], offset[0] + width * p[0]);
    let h = model.get_terrain_height(p) as i32;
    ncurses::printw(&format!("{:2}", h));
}

fn print_velocity(p: Point2i, offset: Point2i, width: i32, model: &Model) {
    ncurses::mv(offset[1] + p[1], offset[0] + width * p[0]);
    let v = model.get_velocity(p);
    ncurses::printw(&format!("{:.1}/{:.1}", v[0], v[1]));
}

fn main() {
    load_ncurses();

    let start = Instant::now();
    let seed = Seed::from_entropy();
    let size = 20;
    let noise = get_default_noise(seed);
    let heightmap = HeightMap::from_noise(Point2f::from_scalar(0.), size, 1, noise.as_ref());

    let mut rng: SmallRng = seed.into();
    let mut model = Model::from(heightmap);
    for i in 0..10000 {
        for y in 0..size {
            for x in 0..size {
                let p = Point2i::new(x, y);
                print_water_height(p, Point2i::new(10, 1), 5, &model);
                print_terrain_height(p, Point2i::new(10, 22), 5, &model);
                print_velocity(p, Point2i::new(10, 43), 10, &model);
            }
        }
        model = model.run(1, Seed::from_rng(&mut rng));
        ncurses::refresh();
        if ncurses::getch() != ncurses::ERR {
            break;
        }
        sleep(Duration::from_millis(100));
    }

    let work_duration = start.elapsed().as_secs() as u32;

    ncurses::getch();
    quit_ncurses();
}
