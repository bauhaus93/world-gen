use rand::prelude::SliceRandom;
use rand::Rng;
use std::rc::Rc;

use glm::{length, normalize, sin, Vector2, Vector3};

use super::direction::{get_neighbour_pos, get_opposite_direction, Direction, DirectionIterator};
use super::{Cell, Parameter};
use crate::HeightMap;

#[derive(Clone)]
pub struct State {
    age: usize,
    cells: Vec<Cell>,
    parameter: Rc<Parameter>,
}

impl State {
    pub fn get_size(&self) -> i32 {
        self.parameter.get_size()
    }
    pub fn get_total_water(&self) -> f64 {
        self.cells.iter().map(|cell| cell.get_water_height()).sum()
    }
    pub fn get_age(&self) -> usize {
        self.age
    }

    pub fn age_increment(&mut self) {
        self.age += 1;
    }

    pub fn rain<R: Rng + ?Sized>(&self, total_water: f64, drop_count: u32, rng: &mut R) -> State {
        let drop_size = total_water / drop_count as f64;
        let mut next_state = self.clone();
        for _ in 0..drop_count {
            next_state.add_water_drop(drop_size, rng);
        }
        next_state
    }

    pub fn has_water(&self) -> bool {
        self.cells.iter().any(|c| c.has_water())
    }

    pub fn calculate_flow(&mut self, prev_state: &State) {
        prev_state
            .cells
            .iter()
            .filter(|prev| prev.has_water())
            .for_each(|prev| {
                let new_flow = prev_state.calculate_flow_for_cell(prev);
                let k =
                    calculate_flow_coefficent(&new_flow, prev.get_water_height(), &self.parameter);
                let next = self.get_cell_mut(prev.get_pos()).unwrap();
                DirectionIterator::default()
                    .zip(new_flow.iter())
                    .for_each(|(dir, flow)| next.set_flow(dir, flow * k));
            });
    }

    pub fn apply_flow(&mut self) {
        let water_delta: Vec<f64> = self
            .cells
            .iter()
            .map(|cell| {
                DirectionIterator::default()
                    .map(|dir| (dir, self.get_neighbour(cell.get_pos(), dir)))
                    .filter(|(_dir, nb)| nb.is_some())
                    .map(|(dir, nb)| {
                        nb.expect("Should have been some")
                            .get_flow(get_opposite_direction(dir))
                            - cell.get_flow(dir)
                    })
                    .sum()
            })
            .collect();
        self.cells
            .iter_mut()
            .zip(water_delta.into_iter())
            .for_each(|(cell, delta)| cell.mod_water(delta));
    }

    pub fn calculate_velocity(&mut self, prev_state: &State) {
        prev_state.cells.iter().for_each(|prev| {
            let vel = prev_state.calculate_velocity_for_cell(prev);
            let next = self.get_cell_mut(prev.get_pos()).unwrap();
            next.set_velocity(vel);
        });
    }

    pub fn calculate_normals(&mut self, prev_state: &State) {
        prev_state.cells.iter().for_each(|prev| {
            let normal = prev_state.calculate_normal_for_cell(prev);
            let next = self.get_cell_mut(prev.get_pos()).expect("Must be some");
            next.set_normal(normal);
        });
    }

    pub fn calculate_transport_capacity(&mut self, prev_state: &State) {
        prev_state.cells.iter().for_each(|prev| {
            let speed = prev.get_speed();
            let tilt = prev.get_tilt();
            let capacity = self.parameter.get_sediment_capacity() * speed * sin(tilt);
            let next = self.get_cell_mut(prev.get_pos()).expect("Must be some");
            next.set_transport_capacity(capacity);
        });
    }

    pub fn apply_erosion_deposition(&mut self) {
        let diss_const = self.parameter.get_dissolving_constant();
        let depo_const = self.parameter.get_deposition_constant();
        self.cells.iter_mut().for_each(|cell| {
            let transport_capacity = cell.get_transport_capacity();
            let suspended_sediment = cell.get_suspended_sediment();
            let suspended_delta = if transport_capacity > suspended_sediment {
                diss_const * (transport_capacity - suspended_sediment)
            } else {
                -depo_const * (suspended_sediment - transport_capacity)
            };
            cell.mod_height(-suspended_delta);
            cell.mod_suspended_sediment(suspended_delta);
        });
    }

    pub fn apply_sediment_transportation(&mut self) {
        let transported_sediment: Vec<f64> = self
            .cells
            .iter()
            .map(|cell| {
                let source_point = cell.get_sediment_source(self.parameter.get_time_delta());
                let points = get_nearest_grid_points(source_point);
                let (sediment_sum, distance_sum) = points
                    .iter()
                    .map(|p| {
                        (
                            Vector2::<f64>::new(p.x as f64, p.y as f64),
                            self.get_cell(&[p.x, p.y])
                                .map_or(cell.get_suspended_sediment(), |c| {
                                    c.get_suspended_sediment()
                                }),
                        )
                    })
                    .map(|(p, susp)| (susp, length(source_point - p)))
                    .fold((0., 0.), |(sed_sum, dist_sum), (susp, dist)| {
                        (sed_sum + susp / f64::max(dist, 1e-6), dist_sum + dist)
                    });
                sediment_sum * distance_sum
            })
            .collect();
        self.cells
            .iter_mut()
            .zip(transported_sediment.into_iter())
            .for_each(|(cell, sed)| cell.set_suspended_sediment(sed));
    }

    fn calculate_flow_for_cell(&self, cell: &Cell) -> Vec<f64> {
        DirectionIterator::default()
            .map(|dir| (dir, self.get_neighbour(cell.get_pos(), dir)))
            .map(|(dir, opt_nb)| match opt_nb {
                Some(nb) => cell.calculate_flow(dir, nb, &self.parameter),
                None => 0.,
            })
            .collect()
    }

    fn calculate_velocity_for_cell(&self, cell: &Cell) -> Vector2<f64> {
        let nb: Vec<Option<&Cell>> = DirectionIterator::default()
            .map(|dir| self.get_neighbour(cell.get_pos(), dir))
            .collect();
        let flow_from_left = nb[usize::from(Direction::LEFT)]
            .map_or(0., |cell| cell.get_flow(Direction::RIGHT))
            - cell.get_flow(Direction::LEFT);
        let flow_to_right = cell.get_flow(Direction::RIGHT)
            - nb[usize::from(Direction::RIGHT)].map_or(0., |cell| cell.get_flow(Direction::LEFT));

        let flow_from_top = nb[usize::from(Direction::TOP)]
            .map_or(0., |cell| cell.get_flow(Direction::BOTTOM))
            - cell.get_flow(Direction::TOP);
        let flow_to_bottom = cell.get_flow(Direction::BOTTOM)
            - nb[usize::from(Direction::BOTTOM)].map_or(0., |cell| cell.get_flow(Direction::TOP));
        Vector2::new(
            flow_from_left - flow_to_right / 2.,
            flow_from_top - flow_to_bottom / 2.,
        )
    }

    fn calculate_normal_for_cell(&self, cell: &Cell) -> Vector3<f64> {
        let heights: Vec<f64> = DirectionIterator::default()
            .map(|dir| self.get_neighbour(cell.get_pos(), dir))
            .map(|nb| nb.map_or_else(|| cell.get_total_height(), |nb| nb.get_total_height()))
            .collect();
        let slope_x = heights[usize::from(Direction::LEFT)] - heights[usize::from(Direction::LEFT)];
        let slope_y =
            heights[usize::from(Direction::TOP)] - heights[usize::from(Direction::BOTTOM)];
        debug_assert!(!slope_x.is_nan());
        debug_assert!(!slope_y.is_nan());
        normalize(Vector3::new(slope_x, slope_y, 2.))
        //Vector3::new(0., 0., 1.)
    }

    fn add_water_drop<R: Rng + ?Sized>(&mut self, drop_size: f64, rng: &mut R) {
        match self.cells.choose_mut(rng) {
            Some(cell) => cell.mod_water(drop_size),
            None => unreachable!(),
        }
    }

    fn pos_to_index(&self, pos: &[i32; 2]) -> Option<usize> {
        let size = self.parameter.get_size();
        if pos[0] >= size || pos[1] >= size || pos[0] < 0 || pos[1] < 0 {
            return None;
        }
        let index = (pos[1] * self.parameter.get_size() + pos[0]) as usize;
        debug_assert!(index < self.cells.len());
        Some(index)
    }

    fn get_cell_mut(&mut self, pos: &[i32; 2]) -> Option<&mut Cell> {
        match self.pos_to_index(pos) {
            Some(index) => Some(&mut self.cells[index]),
            None => None,
        }
    }

    pub fn get_cell(&self, pos: &[i32; 2]) -> Option<&Cell> {
        match self.pos_to_index(pos) {
            Some(index) => Some(&self.cells[index]),
            None => None,
        }
    }

    fn get_neighbour(&self, pos: &[i32; 2], dir: Direction) -> Option<&Cell> {
        match self.pos_to_index(&get_neighbour_pos(pos, dir)) {
            Some(index) => Some(&self.cells[index]),
            None => None,
        }
    }
}

impl From<HeightMap> for State {
    fn from(height_map: HeightMap) -> State {
        let size = height_map.get_size();
        let mut cells = Vec::with_capacity((size * size) as usize);
        for y in 0..size {
            for x in 0..size {
                let pos = [x as i32, y as i32];
                let mut cell = Cell::new(pos);
                cell.set_terrain_height(height_map.get(&pos));
                cells.push(cell);
            }
        }
        State {
            age: 0,
            cells: cells,
            parameter: Rc::new(Parameter::new(size)),
        }
    }
}

impl Into<HeightMap> for State {
    fn into(self) -> HeightMap {
        let mut height_map = HeightMap::new(self.parameter.get_size(), 1);
        self.cells
            .iter()
            .for_each(|c| height_map.set(c.get_pos(), c.get_terrain_height()));
        height_map
    }
}

fn calculate_flow_coefficent(flow: &[f64], water_height: f64, params: &Parameter) -> f64 {
    let total_flow: f64 = flow.iter().sum();
    if total_flow > 0. {
        let grid_dist = params.get_grid_distance();
        let factor = f64::min(1., grid_dist[0] * grid_dist[1] / params.get_time_delta());
        f64::min(1., (water_height / total_flow) * factor)
    } else {
        0.
    }
}

fn get_nearest_grid_points(p: Vector2<f64>) -> [Vector2<i32>; 4] {
    [
        Vector2::<i32>::new(p.x.floor() as i32, p.y.floor() as i32),
        Vector2::<i32>::new(p.x.floor() as i32 + 1, p.y.floor() as i32),
        Vector2::<i32>::new(p.x.floor() as i32, p.y.floor() as i32 + 1),
        Vector2::<i32>::new(p.x.floor() as i32 + 1, p.y.floor() as i32 + 1),
    ]
}
