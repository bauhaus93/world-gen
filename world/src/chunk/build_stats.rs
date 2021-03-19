pub struct BuildStats {
    build_time_accumulated: u32,
    build_count: u32,
}

impl Default for BuildStats {
    fn default() -> Self {
        Self {
            build_time_accumulated: 0,
            build_count: 0,
        }
    }
}

impl BuildStats {
    pub fn add_time(&mut self, build_time: u32) {
        if self.build_count > 40000 {
            self.build_time_accumulated = 0;
            self.build_count = 0;
        }
        self.build_time_accumulated += build_time;
        self.build_count += 1;
    }
    pub fn get_avg_time(&mut self) -> f64 {
        if self.build_count > 0 {
            self.build_time_accumulated as f64 / self.build_count as f64
        } else {
            0.
        }
    }
}
