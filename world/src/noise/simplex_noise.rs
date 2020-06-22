use rand;
use rand::prelude::SliceRandom;
use rand::{prelude::SmallRng, Rng};
use std::{f32, iter};

use super::Noise;
use core::{Point2f, Seed};

/*
    Noise calculation based on code by
    Stefan Gustavson & Peter Eastman
    itn.liu.se/~stegu/simplexnoise/SimplexNoise.java
*/

lazy_static! {
    static ref F2: f32 = 0.5 * (f32::sqrt(3.) - 1.);
    static ref G2: f32 = (3. - f32::sqrt(3.)) / 6.;
}

const GRADIENTS: [[i32; 2]; 12] = [
    [1, 1],
    [-1, 1],
    [1, -1],
    [-1, -1],
    [1, 0],
    [-1, 0],
    [1, 0],
    [-1, 0],
    [0, 1],
    [0, -1],
    [0, 1],
    [0, -1],
];

#[derive(Clone)]
pub struct SimplexNoise {
    seed: Seed,
    permutation_table: Vec<u8>,
}

fn create_permutation_table(seed: Seed) -> Vec<u8> {
    let mut rng: SmallRng = seed.into();
    let mut permutation: Vec<u8> = (0u8..255).chain(iter::once(255u8)).collect();
    permutation.shuffle(&mut rng);
    let perm_clone = permutation.clone();
    permutation.extend(perm_clone);
    for v in permutation.iter_mut().skip(256) {
        *v = *v % 12;
    }
    permutation
}

impl SimplexNoise {
    pub fn from_seed(seed: Seed) -> Self {
        Self {
            seed: seed,
            permutation_table: create_permutation_table(seed),
        }
    }
}

impl Noise for SimplexNoise {
    fn get_noise(&self, p: Point2f) -> f32 {
        let skew = (p[0] + p[1]) * *F2;
        /*  if not floored, noise can have sharp edges on negative coordinates
            https://stackoverflow.com/questions/10705640/perlin-noise-with-negative-coordinate-input
        */
        let skew_coord: [i32; 2] = [
            f32::floor(p[0] + skew) as i32,
            f32::floor(p[1] + skew) as i32,
        ];
        let unskew = (skew_coord[0] + skew_coord[1]) as f32 * *G2;

        let cell_origin: [f32; 2] = [skew_coord[0] as f32 - unskew, skew_coord[1] as f32 - unskew];

        let corner = calculate_corners(p, cell_origin);

        let table_base_index: [i32; 2] = [skew_coord[0] & 0xFF, skew_coord[1] & 0xFF];
        let table_offset: [[i32; 2]; 3] = [[0, 0], get_second_corner_offset(corner[0]), [1, 1]];
        let mut contrib_sum: f32 = 0.;
        for i in 0..3 {
            let grad_index = calculate_gradient_index(
                table_base_index,
                table_offset[i],
                &self.permutation_table,
            );
            contrib_sum += calculate_corner_contribution(grad_index, corner[i]);
        }
        debug_assert!((70. * contrib_sum).abs() <= 1.);
        70. * contrib_sum
    }

    fn get_range(&self) -> [f32; 2] {
        [-1., 1.]
    }

    fn get_cycle(&self) -> Point2f {
        Point2f::from_scalar(f32::INFINITY)
    }
}

fn calculate_corners(p: Point2f, cell_origin: [f32; 2]) -> [[f32; 2]; 3] {
    let mut corner = [[0., 0.]; 3];

    corner[0][0] = p[0] - cell_origin[0];
    corner[0][1] = p[1] - cell_origin[1];

    let offset = get_second_corner_offset(corner[0]);
    corner[1][0] = corner[0][0] - offset[0] as f32 + *G2;
    corner[1][1] = corner[0][1] - offset[1] as f32 + *G2;

    corner[2][0] = corner[0][0] - 1. + 2. * *G2;
    corner[2][1] = corner[0][1] - 1. + 2. * *G2;
    corner
}

fn get_second_corner_offset(first_corner: [f32; 2]) -> [i32; 2] {
    match first_corner[0] > first_corner[1] {
        true => [1, 0],
        _ => [0, 1],
    }
}

fn calculate_gradient_index(base: [i32; 2], off: [i32; 2], table: &[u8]) -> u8 {
    table[256usize
        + (base[0] + off[0] + table[(base[1] + off[1]) as usize % 256] as i32) as usize % 256]
}

fn calculate_corner_contribution(grad_index: u8, corner_offset: [f32; 2]) -> f32 {
    let t: f32 = 0.5 - corner_offset[0].powf(2.) - corner_offset[1].powf(2.);
    if t < 0. {
        0.
    } else {
        t.powf(4.) * dot(GRADIENTS[grad_index as usize], corner_offset)
    }
}

fn dot(grad: [i32; 2], p: [f32; 2]) -> f32 {
    grad[0] as f32 * p[0] + grad[1] as f32 * p[1]
}
