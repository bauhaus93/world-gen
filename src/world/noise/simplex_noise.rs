use std::iter;
use rand;
use rand::Rng;
use rand::prelude::{ SmallRng, SliceRandom };
use rand::{ SeedableRng };

use super::Noise;

/*
    Noise calculation based on code by
    Stefan Gustavson & Peter Eastman
    itn.liu.se/~stegu/simplexnoise/SimplexNoise.java
*/

lazy_static! {
    static ref F2: f32 = 0.5 * (f32::sqrt(3.) - 1.);
    static ref G2: f32 = (3. - f32::sqrt(3.)) / 6.;
}

const GRADIENTS: [(i32, i32); 12] = [
    (1, 1), (-1, 1), (1, -1), (-1, -1),
    (1, 0), (-1, 0), (1, 0), (-1, 0),
    (0, 1), (0, -1), (0, 1), (0, -1)
];

pub struct SimplexNoise {
    seed: [u8; 16],
    permutation_table: Vec<u8>,
}

fn create_permutation_table(seed: [u8; 16]) -> Vec<u8> {
    let mut rng = SmallRng::from_seed(seed);
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
    pub fn from_str_seed(seed_str: &str) -> Self {
        Self::from_seed(seed_str.as_bytes())
    }

    pub fn from_seed(input_seed: &[u8]) -> Self {
        let mut seed: [u8; 16] = [0; 16];
        for (v, s) in seed.iter_mut().zip(input_seed.iter()) {
            *v = *s;
        }
        Self {
            seed: seed,
            permutation_table: create_permutation_table(seed),
        }
    }
}

impl Default for SimplexNoise {
    fn default() -> Self {
        let mut seed_rng = rand::thread_rng();
        let seed: Vec<u8> = iter::repeat_with(|| seed_rng.gen()).take(16).collect();
        Self::from_seed(&seed)
    }
}

impl Noise for SimplexNoise {
    fn get_noise(&self, p: (f32, f32)) -> f32 {
        let skew = (p.0 + p.1) * *F2;
        let skew_coord: (i32, i32) = ((p.0 + skew) as i32,
                                      (p.1 + skew) as i32);
        let unskew = (skew_coord.0 + skew_coord.1) as f32 * *G2;

        let cell_origin: (f32, f32) = (skew_coord.0 as f32 - unskew,
                                       skew_coord.1 as f32 - unskew);

        let corner = calculate_corners(p, cell_origin);
        
        let table_base_index: (i32, i32) = (skew_coord.0 & 0xFF, skew_coord.1 & 0xFF);      //maybe try % 256 to work with neg points? (p neg -> i neg -> i & 0xFF ??)
        let table_offset: [(i32, i32); 3] = [(0, 0),
                                       get_second_corner_offset(corner[0]),
                                      (1, 1)];
        let mut contrib_sum: f32 = 0.;
        for i in 0..3 {
            let grad_index = calculate_gradient_index(table_base_index, table_offset[i], &self.permutation_table);
            contrib_sum += calculate_corner_contribution(grad_index, corner[i]);
        }
        debug_assert!((70. * contrib_sum).abs() <= 1.);
        70. * contrib_sum
    }
    fn get_range(&self) -> (f32, f32) {
        (-1., 1.)
    }
}

fn calculate_corners(p: (f32, f32), cell_origin: (f32, f32))  -> [(f32, f32); 3] {
    let mut corner = [(0., 0.); 3];
    
    corner[0].0 = p.0 - cell_origin.0;
    corner[0].1 = p.1 - cell_origin.1;
    
    let offset = get_second_corner_offset(corner[0]);
    corner[1].0 = corner[0].0 - offset.0 as f32 + *G2;
    corner[1].1 = corner[0].1 - offset.1 as f32 + *G2;
    
    corner[2].0 = corner[0].0 - 1. + 2. * *G2;
    corner[2].1 = corner[0].1 - 1. + 2. * *G2;
    corner 
}

fn get_second_corner_offset(first_corner: (f32, f32)) -> (i32, i32) {
    match first_corner.0 > first_corner.1 {
        true => (1, 0),
        _ => (0, 1)
    }
}

fn calculate_gradient_index(base: (i32, i32), off: (i32, i32), table: &[u8]) -> u8 {
    table[256usize + (base.0 + off.0 + table[(base.1 + off.1) as usize % 256] as i32) as usize % 256]
}

fn calculate_corner_contribution(grad_index: u8, corner_offset: (f32, f32)) -> f32 {
    let t: f32 = 0.5 - corner_offset.0.powf(2.) - corner_offset.1.powf(2.);
    if t < 0. {
        0.
    } else {
       t.powf(4.) * dot(GRADIENTS[grad_index as usize], corner_offset) 
    }
}

fn dot(grad: (i32, i32), p: (f32, f32)) -> f32 {
    grad.0 as f32 * p.0 + grad.1 as f32 * p.1
}
