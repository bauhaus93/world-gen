use std::rc::{ Rc, Weak };
use std::cell::RefCell;
use rand::{ Rng, SeedableRng };
use rand::rngs::SmallRng;

use crate::utility::Float;
use crate::world::chunk::height_map::HeightMap;
use super::cell::Cell;

// http://www-ljk.imag.fr/Publications/Basilic/com.lmc.publi.PUBLI_Inproceedings@117681e94b6_fff75c/FastErosion_PG07.pdf

const GRAVITY: Float = 1.;
const TIME_DELTA: Float = 1.;
const SEDIMENT_CAPACITY_CONSTANT: Float = 1.;
const DISSOLVING_CONSTANT: Float = 1.;
const DEPOSITION_CONSTANT: Float = 1.;
const EVAPORATION_CONSTANT: Float = 0.2;

pub struct HydraulicErosion {
    rng: SmallRng,
    size: [usize; 2],
    cell_list: Vec<Rc<RefCell<Cell>>>
}

impl HydraulicErosion {
    pub fn new<R: Rng + ?Sized>(height_map: &HeightMap, rng_input: &mut R) -> Self {
        let size = height_map.get_size();
        let mut cell_list = Vec::with_capacity((size[0] * size[1]) as usize);
        for i in 0..size[0] * size[1] {
            let mut cell = Cell::default();
            cell.set_terrain_height(height_map.get_by_index(i as usize));
            cell_list.push(Rc::new(RefCell::new(cell)));
        }

        Self {
            rng: SmallRng::from_rng(rng_input).unwrap(),
            size: [size[0] as usize, size[1] as usize],
            cell_list: cell_list
        }
    }

    fn load_cell_neighbours(&mut self) {
        for cell_index in 0..self.cell_list.len() {
            let mut nb_cells: [Option<Weak<RefCell<Cell>>>; 4] = [None, None, None, None];
            for dir in 0..4 {
                if let Some(nb_index) = self.get_neighbour(cell_index, dir) {
                    nb_cells[dir as usize] = Some(Rc::downgrade(&self.cell_list[nb_index]));
                }
            }
            self.cell_list[cell_index].borrow_mut().set_neighbours(nb_cells);
        }
    }

    pub fn erode(&mut self) {
        self.add_water(10);

        for _i in 0..30 {
            self.update_flux();
            self.apply_flux();
            self.update_transport_capacity();
            self.apply_erosion_deposition();
            self.update_transported_sediment();
            self.apply_transported_sediment();
            self.apply_evaporation();
        }
    }

    fn add_water(&mut self, drop_count: u32) {
        for _i in 0..drop_count {
            let drop_index = self.rng.gen_range(0, self.cell_list.len());
            self.cell_list[drop_index].borrow_mut().mod_water_height(1.);
        }
    }

    fn update_flux(&mut self) {
        for cell_index in 0..self.cell_list.len() {
            self.cell_list[cell_index].borrow_mut().update_flux(GRAVITY, TIME_DELTA);
        }
    }

    fn apply_flux(&mut self) {
        for cell_index in 0..self.cell_list.len() {
            self.cell_list[cell_index].borrow_mut().apply_flux(TIME_DELTA);
        }
    }

    fn update_transport_capacity(&mut self) {
        for cell_index in 0..self.cell_list.len() {
            self.cell_list[cell_index].borrow_mut().update_transport_capacity(SEDIMENT_CAPACITY_CONSTANT);
        }
    }

    fn apply_erosion_deposition(&mut self) {
        for cell_index in 0..self.cell_list.len() {
            self.cell_list[cell_index].borrow_mut().apply_erosion_deposition(DISSOLVING_CONSTANT,
                                                                             DEPOSITION_CONSTANT);
        }
    }

    fn update_transported_sediment(&mut self) {
        for cell_index in 0..self.cell_list.len() {
            self.cell_list[cell_index].borrow_mut().update_transported_sediment(TIME_DELTA);
        } 
    }

    fn apply_transported_sediment(&mut self) {
        for cell_index in 0..self.cell_list.len() {
            self.cell_list[cell_index].borrow_mut().apply_transported_sediment();
        } 
    }

    fn apply_evaporation(&mut self) {
        for cell_index in 0..self.cell_list.len() {
            self.cell_list[cell_index].borrow_mut().apply_evaporation(EVAPORATION_CONSTANT, TIME_DELTA);
        } 
    }

    fn get_neighbour(&self, index: usize, dir: u8) -> Option<usize> {
        match dir {
            0 if index >= self.size[0] => Some(index - self.size[0]),                        // TOP
            1 if (index + 1) % self.size[0] != 0 => Some(index + 1),                         // RIGHT
            2 if index + self.size[0] < self.cell_list.len() => Some(index + self.size[0]),  // BOTTOM
            3 if index % self.size[0] != 0 => Some(index - 1),                               // LEFT
            _ => None
        }
    }
}

