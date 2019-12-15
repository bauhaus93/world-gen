extern crate ncurses;
extern crate glm;
extern crate rand;

extern crate world;

use std::{thread, time::Duration};
use ncurses::*;

use world::{HeightMap, HydraulicErosion};

fn main() {
	initscr();
	noecho();
	cbreak();
	curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
	let hm = HeightMap::new(16, 1);
	let mut erosion = HydraulicErosion::from(hm);
	erosion.rain(100., 1);
	for i in 0..100 {
		erosion.simulate(10);
		let state = erosion.get_state();
		clear();
		mvaddstr(0, 0, "age");
		mvaddstr(0, 20, &format!("{:5}", state.get_age()));
		mvaddstr(1, 0, "water");
		mvaddstr(1, 20, &format!("{:5.2}", state.get_total_water()));

		for y in 0..state.get_size() {
			for x in 0..state.get_size() {
				mvaddstr(5 + y, 8 * x, &format!("{:5.2}", state.get_cell(&[x, y]).unwrap().get_water_height()));
			}
		}

		refresh();
		thread::sleep(Duration::from_millis(100));
	}

	getch();
	endwin();
}
