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

    pub fn get_interpolated_height(&self, relative_pos: [Float; 2]) -> Float {
        let root_pos = [clamp((relative_pos[0].floor() as i32) / self.resolution, 0, self.size - 1),
                        clamp((relative_pos[1].floor() as i32) / self.resolution, 0, self.size - 1)];
        let reference_height: [Float; 4] = [
            self.get(&root_pos),
            self.get(&[i32::min(root_pos[0] + 1, self.size - 1), root_pos[1]]),
            self.get(&[root_pos[0], i32::min(root_pos[1] + 1, self.size - 1)]),
            self.get(&[i32::min(root_pos[0] + 1, self.size - 1), i32::min(root_pos[1] + 1, self.size - 1)])
        ];
        let relative_point = [relative_pos[0] - root_pos[0] as Float,
                              relative_pos[1] - root_pos[1] as Float];

        let res = interpolate(relative_point, reference_height);
        res
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

fn interpolate(p: [Float; 2], reference: [Float; 4]) -> Float {
    let anchor = [p[0].floor() as i32, p[1].floor() as i32];
    let a = anchor[0] as Float + 1. - p[0];
    let b = p[0] - anchor[0] as Float;
    let r_1 = a * reference[0] + b * reference[1];
    let r_2 = a * reference[2] + b * reference[3];
    let c = anchor[1] as Float + 1. - p[1];
    let d = p[1] - anchor[1] as Float;
    c * r_1 + d * r_2
}

fn clamp<T>(value: T, min: T, max: T) -> T
where T: Ord {
    T::min(T::max(value, min), max)
}