use utility::Float;

pub struct HeightMap {
    size: i32,
    resolution: i32,
    height_list: Vec<Float>
}


impl HeightMap {

    pub fn new(size: i32, resolution: i32) -> Self {
        debug_assert!(size> 0);
        debug_assert!(resolution > 0);
        let mut height_list =  Vec::new();
        height_list.resize((size * size) as usize, 0.);
        Self {
            size: size,
            resolution: resolution,
            height_list: height_list
        }
    }

    #[allow(unused)]
    pub fn get_size(&self) -> i32 {
        self.size
    }

    pub fn get_resolution(&self) -> i32 {
        self.resolution
    }

    pub fn set(&mut self, pos: &[i32; 2], height: Float) {
        let index = self.calculate_index(pos);
        self.height_list[index] = height;
    }

    pub fn get(&self, pos: &[i32; 2]) -> Float {
        self.height_list[self.calculate_index(pos)]
    }
    #[allow(unused)]
    pub fn set_by_index(&mut self, index: usize, height: Float) {
        self.height_list[index] = height;
    }

    #[allow(unused)]
    pub fn get_by_index(&self, index: usize) -> Float {
        debug_assert!(index < self.height_list.len());
        self.height_list[index]
    }
    #[allow(unused)]
    fn get_quad_heights(&self, anchor: [i32; 2]) -> [Float; 4] {
        [self.get(&anchor),
         self.get(&[anchor[0] + 1, anchor[1]]),
         self.get(&[anchor[0], anchor[1] + 1]),
         self.get(&[anchor[0] + 1, anchor[1] + 1])]
    }

    fn calculate_index(&self, pos: &[i32; 2]) -> usize {
        debug_assert!(pos[0] >= 0 && pos[1] >= 0);
        debug_assert!(pos[0] < self.size && pos[1] < self.size);
        (pos[0] + self.size * pos[1]) as usize
    }
}