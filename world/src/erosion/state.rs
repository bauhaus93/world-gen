use rand::prelude::SliceRandom;
use rand::Rng;
use std::rc::Rc;

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
                    .filter(|(dir, nb)| nb.is_some())
                    .map(|(dir, nb)| {
                        nb.unwrap().get_flow(get_opposite_direction(dir)) - cell.get_flow(dir)
                    })
                    .sum()
            })
            .collect();
        self.cells
            .iter_mut()
            .zip(water_delta.into_iter())
            .for_each(|(cell, delta)| cell.mod_water(delta));
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
