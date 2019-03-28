use rand::{ Rng, SeedableRng };
use rand::rngs::SmallRng;
use glm::{ Vector2, GenNum };

use utility::Float;
use crate::chunk::height_map::HeightMap;
use super::cell::Cell;
use super::direction::{ Direction, get_neighbour_pos, get_opposite_direction };

// http://www-ljk.imag.fr/Publications/Basilic/com.lmc.publi.PUBLI_Inproceedings@117681e94b6_fff75c/FastErosion_PG07.pdf

const GRAVITY: Float = 9.81;
const TIME_DELTA: Float = 1.;
const PIPE_AREA: Float = 0.001;
const PIPE_LENGTH: Float = 1.;
const GRID_DISTANCE: [Float; 2] = [1., 1.];
const SEDIMENT_CAPACITY_CONSTANT: Float = 35.;
const DISSOLVING_CONSTANT: Float = 0.0012;
const DEPOSITION_CONSTANT: Float = 0.0012;
const EVAPORATION_CONSTANT: Float = 0.001;

const NEIGHBOURS: [Direction; 4] = [Direction::TOP,
                                    Direction::RIGHT,
                                    Direction::BOTTOM,
                                    Direction::LEFT];


pub struct HydraulicErosion {
    rng: SmallRng,
    size: [i32; 2],
    cells: Vec<Cell>,
    pipe_area: Float,
    pipe_length: Float,
    grid_distance: [Float; 2],
    gravity: Float
}

impl HydraulicErosion {
    pub fn new<R: Rng + ?Sized>(height_map: &HeightMap, rng_input: &mut R) -> Self {
        let size = height_map.get_size();
        let mut cells = Vec::with_capacity((size[0] * size[1]) as usize);
        for y in 0..size[1] {
            for x in 0..size[0] {
                let pos = [x, y];
                let mut cell = Cell::default();
                cell.set_terrain_height(height_map.get(&pos));
                cells.push(cell);
            }
        }

        let erosion = Self {
            rng: SmallRng::from_rng(rng_input).unwrap(),
            size: size,
            cells: cells,
            pipe_area: PIPE_AREA,
            pipe_length: PIPE_LENGTH,
            grid_distance: GRID_DISTANCE,
            gravity: GRAVITY
        };
        erosion
    }

    pub fn tick(&mut self) {
        const TIME_DELTA: Float = 1.;
        self.update_outflow(TIME_DELTA);
        self.apply_waterflow(TIME_DELTA);
    }

    pub fn rain(&mut self, drop_count: u32, drop_size: Float) {
        for _ in 0..drop_count {
            self.add_water_drop(drop_size);
        }
    }

    pub fn create_heightmap(&self) -> HeightMap {
        let mut height_map = HeightMap::new([self.size[0] as i32, self.size[1] as i32]);
        for y in 0..self.size[1] {
            for x in 0..self.size[0] {
                let pos = [x, y];
                let height = self.get_cell(&pos).get_terrain_height();
                height_map.set(&pos, height);
            }
        }
        height_map
    }

    fn update_outflow(&mut self, time_delta: Float) {
        let flow_factor = (time_delta * self.pipe_area * self.gravity) / self.pipe_length;
        for y in 0..self.size[1] {
            for x in 0..self.size[0] {
                self.update_cell_outflow(&[x, y], flow_factor, time_delta);
            }
        }
    }

    fn apply_waterflow(&mut self, time_delta: Float) {
        for y in 0..self.size[1] {
            for x in 0..self.size[0] {
                self.apply_cell_waterflow(&[x, y], time_delta);
            }
        }
    }

    fn update_cell_outflow(&mut self, pos: &[i32; 2], flow_factor: Float, time_delta: Float) {
        let cell = self.get_cell(pos);
        let mut new_flow = [0., 0., 0., 0.];
        let mut flow_sum = 0.;
        for dir in &NEIGHBOURS {
            if let Some(nb) = self.get_neighbour(pos, *dir) {
                let water_delta = cell.get_water_level() - nb.get_water_level();
                let index: usize = Direction::into(*dir);
                new_flow[index] = Float::max(0., cell.get_flow(*dir) + flow_factor * water_delta);
                flow_sum += new_flow[index];
            }
        }
        let k = Float::min(1., (cell.get_water_height() * self.grid_distance[0] * self.grid_distance[1]) / (flow_sum * time_delta));
        let cell = self.get_cell_mut(pos);
        for dir in &NEIGHBOURS {
            let index: usize = Direction::into(*dir);
            let scaled_flow = k * new_flow[index];
            cell.set_flow(*dir, scaled_flow);
        }
    }

    fn apply_cell_waterflow(&mut self, pos: &[i32; 2], time_delta: Float) {
        let cell = self.get_cell(pos);
        let mut flow_delta: [Float; 2] = [0., 0.];
        for dir in &NEIGHBOURS {
            if let Some(nb) = self.get_neighbour(pos, *dir) {
                let inflow = nb.get_flow(get_opposite_direction(*dir));
                let outflow = cell.get_flow(*dir);
                match dir {
                    Direction::TOP | Direction::BOTTOM => flow_delta[1] += inflow - outflow,
                    Direction::LEFT | Direction::RIGHT => flow_delta[0] += inflow - outflow
                }
            }
        }
        let water_delta = (time_delta * (flow_delta[0] + flow_delta[1])) / (self.grid_distance[0] * self.grid_distance[1]);
        let mut new_velocity = Vector2::from_s(0.);
        for axis in 0..2 {
            let flow_avg = flow_delta[axis] / 2.;
            let d = water_delta / 2.;
            new_velocity[axis] = flow_avg / (d * self.grid_distance[(axis + 1 % 2)]);
        }
        let cell = self.get_cell_mut(pos);
        cell.mod_water(water_delta);
        cell.set_velocity(new_velocity);
    }

    fn add_water_drop(&mut self, size: Float) {
        let pos = self.get_random_pos();
        self.get_cell_mut(&pos).mod_water(size);
        for dir in &NEIGHBOURS {
            if let Some(nb) = self.get_neighbour_mut(&pos, *dir) {
                nb.mod_water(size / 4.);
            }
        }
    }

    fn get_random_pos(&mut self) -> [i32; 2] {
        [self.rng.gen_range(0, self.size[0]),
         self.rng.gen_range(0, self.size[1])]
    }

    fn get_cell(&self, pos: &[i32; 2]) -> &Cell {
        let index = (pos[1] * self.size[0] + pos[0]) as usize;
        &self.cells[index]
    }
    fn get_cell_mut(&mut self, pos: &[i32; 2]) -> &mut Cell {
        let index = (pos[1] * self.size[0] + pos[0]) as usize;
        &mut self.cells[index]
    }

    fn get_neighbour(&self, pos: &[i32; 2], dir: Direction) -> Option<&Cell> {
        let nb_pos = get_neighbour_pos(pos, dir);
        let nb_index = (nb_pos[1] * self.size[0] + nb_pos[0]) as usize;
        if nb_index >= self.cells.len() {
            None
        } else {
            Some(&self.cells[nb_index])
        }
    }
    fn get_neighbour_mut(&mut self, pos: &[i32; 2], dir: Direction) -> Option<&mut Cell> {
        let nb_pos = get_neighbour_pos(pos, dir);
        let nb_index = (nb_pos[1] * self.size[0] + nb_pos[0]) as usize;
        if nb_index >= self.cells.len() {
            None
        } else {
            Some(&mut self.cells[nb_index])
        }
    }
}


