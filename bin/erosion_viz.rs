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
    ncurses::addstr(&format!("{:3.1}", w));

    ncurses::attroff(ncurses::COLOR_PAIR(1));
    ncurses::attroff(ncurses::COLOR_PAIR(2));
}

fn print_terrain_height(p: Point2i, offset: Point2i, width: i32, model: &Model) {
    ncurses::mv(offset[1] + p[1], offset[0] + width * p[0]);
    let h = model.get_terrain_height(p) as i32;
    ncurses::addstr(&format!("{:3}", h));
}

fn print_delta_terrain_height(p: Point2i, offset: Point2i, width: i32, curr_model: &Model, orig_model: &Model) {
    ncurses::mv(offset[1] + p[1], offset[0] + width * p[0]);
    let dh = curr_model.get_terrain_height(p) as i32 - orig_model.get_terrain_height(p) as i32;
    if dh.abs() >= 1  {
        ncurses::attron(ncurses::COLOR_PAIR(4));
    }
    ncurses::addstr(&format!("{:3}", dh));
    ncurses::attroff(ncurses::COLOR_PAIR(4));
}

fn print_velocity(p: Point2i, offset: Point2i, width: i32, model: &Model) {
    ncurses::mv(offset[1] + p[1], offset[0] + width * p[0]);
    let v = model.get_velocity(p);
    ncurses::addstr(&format!("{:.1}/{:.1}", v[0], v[1]));
}

fn print_suspended_sediment(p: Point2i, offset: Point2i, width: i32, model: &Model) {
    ncurses::mv(offset[1] + p[1], offset[0] + width * p[0]);
    let ss = model.get_suspended_sediment(p);
    if ss > 1. {
        ncurses::attron(ncurses::COLOR_PAIR(4));
    } else if ss >= 1e-1 {
        ncurses::attron(ncurses::COLOR_PAIR(3));
    }
    ncurses::addstr(&format!("{:3.1}", ss));

    ncurses::attroff(ncurses::COLOR_PAIR(3));
    ncurses::attroff(ncurses::COLOR_PAIR(4));
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
    let orig_model = model.clone();
    for i in 0..10000 {
        ncurses::clear();
        ncurses::mv(1, 1);
        ncurses::addstr("total water:");
        ncurses::mv(1, 20);
        ncurses::addstr(&format!("{:.1}", model.get_total_water()));

        ncurses::mv(2, 1);
        ncurses::addstr("total height:");
        ncurses::mv(2, 20);
        ncurses::addstr(&format!("{:.1}", model.get_total_terrain_height()));

        ncurses::mv(3, 1);
        ncurses::addstr("total susp sed:");
        ncurses::mv(3, 20);
        ncurses::addstr(&format!("{:.1}", model.get_total_suspended_sediment()));


        ncurses::mv(4, 1);
        ncurses::addstr("h + ss:");
        ncurses::mv(4, 20);
        ncurses::addstr(&format!("{:.1}", model.get_total_terrain_height() + model.get_total_suspended_sediment()));

        ncurses::mv(5, 1);
        ncurses::addstr("total velocity:");
        ncurses::mv(5, 20);
        ncurses::addstr(&format!("{:.1}", model.get_total_velocity()));

        ncurses::mv(6, 1);
        ncurses::addstr("total height delta:");
        ncurses::mv(6, 20);
        ncurses::addstr(&format!("{:.1}", model.get_total_terrain_height() - orig_model.get_total_terrain_height()));



        for y in 0..size {
            for x in 0..size {
                let p = Point2i::new(x, y);
                print_water_height(p, Point2i::new(10, 10), 5, &model);
                print_suspended_sediment(p, Point2i::new(10, 31), 5, &model);
                print_terrain_height(p, Point2i::new(10, 52), 5, &model);
                print_delta_terrain_height(p, Point2i::new(120, 52), 5, &model, &orig_model);
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
