use rand::Rng;

use crate::utility::Float;
use super::chunk_size::CHUNK_SIZE;

pub struct HeightMap {
    size: [i32; 2],
    height_list: Vec<Float>
}


impl HeightMap {

    pub fn new(size: [i32; 2]) -> Self {
        debug_assert!(size[0] > 0 && size[1] > 0);
        let mut height_list =  Vec::new();
        height_list.resize((size[0] * size[1]) as usize, 0.);
        Self {
            size: size,
            height_list: height_list
        }
    }

    pub fn get_size(&self) -> [i32; 2] {
        self.size
    }

    pub fn set(&mut self, pos: &[i32; 2], height: Float) {
        let index = self.calculate_index(pos);
        self.height_list[index] = height;
    }

    pub fn get(&self, pos: &[i32; 2]) -> Float {
        self.height_list[self.calculate_index(pos)]
    }

    fn erode<R: Rng + ?Sized>(&mut self, rng: &mut R) {
        let pos = [rng.gen_range(0., size[0] as Float),
                   rng.gen_range(0., size[1] as Float)];
        let mut height = self.interpolate_height(pos);
        for _lifetime in 0..20 {

        }
    }

    fn interpolate_height(&self, p: [Float; 2]) -> Float {
        let anchor = [p[0].floor() as i32,
                      p[1].floor() as i32];
        let heights = self.get_quad_heights(anchor);
        let a = anchor[0] + 1 - p[0];
        let b = p[0] - anchor[0];
        let r_1 = a * heights[0] + b * heights[1];
        let r_2 = a * heights[2] + b * heights[3];
        let c = anchor[1] + 1 - p[1];
        let d = p[1] - anchor[1];
        c * r_1 + d * r_2
    }

    fn get_quad_heights(&self, anchor: [i32; 2]) -> [Float; 4] {
        [self.get(anchor),
         self.get([anchor[0] + 1, anchor[1]]),
         self.get([anchor[0], anchor[1] + 1]),
         self.get([anchor[0] + 1, anchor[1] + 1])]
    }



    fn calculate_index(&self, pos: &[i32; 2]) -> usize {
        debug_assert!(pos[0] >= 0 && pos[1] >= 0);
        debug_assert!(pos[0] < self.size[0] && pos[1] < self.size[1]);
        (pos[0] + self.size[0] * pos[1]) as usize
    }
}