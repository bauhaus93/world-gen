
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

    pub fn set(&mut self, pos: &[i32; 2], height: Float) {
        let index = self.calculate_index(pos);
        self.height_list[index] = height;
    }

    pub fn get(&self, pos: &[i32; 2]) -> Float {
        self.height_list[self.calculate_index(pos)]
    }

    fn calculate_index(&self, pos: &[i32; 2]) -> usize {
        debug_assert!(pos[0] >= 0 && pos[1] >= 0);
        debug_assert!(pos[0] < self.size[0] && pos[1] < self.size[1]);
        (pos[0] + self.size[0] * pos[1]) as usize
    }
}