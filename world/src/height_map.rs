use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::Path;

use std::cmp::Ordering;

use core::graphics::mesh::Triangle;
use core::{FileError, Point2f, Point2i, Point3f, Seed};
use crate::{Noise, triangulate};

pub struct HeightMap {
    size: i32,
    resolution: f32,
    height_list: Vec<f32>,
}

impl HeightMap {
    pub fn new(size: i32, resolution: f32) -> Self {
        debug_assert!(size > 0);
        debug_assert!(resolution > 0.);
        let mut height_list = Vec::new();
        height_list.resize((size * size) as usize, 0.);
        Self {
            size: size,
            resolution: resolution,
            height_list: height_list,
        }
    }

    pub fn from_noise(origin: Point2f, size: i32, resolution: f32, noise: &dyn Noise) -> Self {
        debug_assert!(size > 0);
        debug_assert!(resolution > 0.);

        debug!("Creating heightmap from noise, size = {}x{}", size, size);
        let mut height_list = Vec::with_capacity((size * size) as usize);
        for y in 0..size {
            for x in 0..size {
                let h = noise
                    .get_noise(origin + Point2f::new(x as f32 * resolution, y as f32 * resolution));
                height_list.push(h);
                if (height_list.len()) % ((size * size) as usize / 20) == 0 {
                    debug!(
                        "Progress: {:2}%",
                        100 * height_list.len() / (size * size) as usize
                    )
                }
            }
        }
        debug!("Created heightmap from noise!");
        Self {
            size: size,
            resolution: resolution,
            height_list: height_list,
        }
    }

    pub fn get_interpolated_height(&self, relative_pos: Point2f) -> f32 {
        let root_pos = Point2i::new(
            clamp(
                (relative_pos[0].floor() / self.resolution) as i32,
                0,
                self.size - 1,
            ),
            clamp(
                (relative_pos[1].floor() / self.resolution) as i32,
                0,
                self.size - 1,
            ),
        );
        let reference_height: [f32; 4] = [
            self.get(root_pos),
            self.get(Point2i::new(
                i32::min(root_pos[0] + 1, self.size - 1),
                root_pos[1],
            )),
            self.get(Point2i::new(
                root_pos[0],
                i32::min(root_pos[1] + 1, self.size - 1),
            )),
            self.get(Point2i::new(
                i32::min(root_pos[0] + 1, self.size - 1),
                i32::min(root_pos[1] + 1, self.size - 1),
            )),
        ];
        let relative_point = relative_pos - Point2f::from(root_pos);

        let res = interpolate(relative_point, reference_height);
        res
    }

    pub fn triangulate(&self) -> Option<Vec<Triangle>> {
        let mut points = Vec::new();
        for y in 0..self.size {
            for x in 0..self.size {
                let point = Point3f::new(x as f32 * self.resolution, y as f32 * self.resolution, self.get(Point2i::new(x, y)));
                points.push(point);
            }
        }
        triangulate(&points)
    }

    #[allow(unused)]
    pub fn get_size(&self) -> i32 {
        self.size
    }

    pub fn get_resolution(&self) -> f32 {
        self.resolution
    }

    pub fn get_list(&self) -> &[f32] {
        self.height_list.as_slice()
    }

    pub fn get_min(&self) -> f32 {
        match self.height_list.iter().min_by(|a, b| {
            if a > b {
                Ordering::Greater
            } else if a < b {
                Ordering::Less
            } else {
                Ordering::Equal
            }
        }) {
            Some(value) => *value,
            None => unreachable!(),
        }
    }

    pub fn get_max(&self) -> f32 {
        match self.height_list.iter().max_by(|a, b| {
            if a > b {
                Ordering::Greater
            } else if a < b {
                Ordering::Less
            } else {
                Ordering::Equal
            }
        }) {
            Some(value) => *value,
            None => unreachable!(),
        }
    }

    pub fn set(&mut self, pos: Point2i, height: f32) {
        let index = self.calculate_index(pos);
        self.height_list[index] = height;
    }

    pub fn get(&self, pos: Point2i) -> f32 {
        self.height_list[self.calculate_index(pos)]
    }
    #[allow(unused)]
    pub fn set_by_index(&mut self, index: usize, height: f32) {
        self.height_list[index] = height;
    }

    #[allow(unused)]
    pub fn get_by_index(&self, index: usize) -> f32 {
        debug_assert!(index < self.height_list.len());
        self.height_list[index]
    }

    #[allow(unused)]
    fn get_quad_heights(&self, anchor: Point2i) -> [f32; 4] {
        [
            self.get(anchor),
            self.get(anchor + Point2i::new(1, 0)),
            self.get(anchor + Point2i::new(0, 1)),
            self.get(anchor + Point2i::new(1, 1)),
        ]
    }

    fn calculate_index(&self, pos: Point2i) -> usize {
        debug_assert!(pos[0] >= 0 && pos[1] >= 0);
        ((pos[0] % self.size) + self.size * (pos[1] % self.size)) as usize
    }
}

fn interpolate(p: Point2f, reference: [f32; 4]) -> f32 {
    let anchor = [p[0].floor() as i32, p[1].floor() as i32];
    let a = anchor[0] as f32 + 1. - p[0];
    let b = p[0] - anchor[0] as f32;
    let r_1 = a * reference[0] + b * reference[1];
    let r_2 = a * reference[2] + b * reference[3];
    let c = anchor[1] as f32 + 1. - p[1];
    let d = p[1] - anchor[1] as f32;
    c * r_1 + d * r_2
}

fn clamp<T>(value: T, min: T, max: T) -> T
where
    T: Ord,
{
    T::min(T::max(value, min), max)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn check_heightmap_triangulation() {
        let hm = HeightMap::new(16, 1.);
        assert!(hm.triangulate().is_some());
    }
}
