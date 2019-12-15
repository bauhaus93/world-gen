const GRAVITY: f64 = 1.;
const TIME_DELTA: f64 = 0.05;
const PIPE_AREA: f64 = 0.001;
const PIPE_LENGTH: f64 = 1.;
const GRID_DISTANCE: [f64; 2] = [1., 1.];
const SEDIMENT_CAPACITY_CONSTANT: f64 = 35.;
const DISSOLVING_CONSTANT: f64 = 0.0012;
const DEPOSITION_CONSTANT: f64 = 0.0012;
const EVAPORATION_CONSTANT: f64 = 0.001;

#[derive(Clone, Copy)]
pub struct Parameter {
    size: i32,
    pipe_area: f64,
    pipe_length: f64,
    grid_distance: [f64; 2],
    gravity: f64,
    time_delta: f64,
    sediment_capacity_constant: f64,
    dissolving_constant: f64,
    deposition_constant: f64,
    evaporation_constant: f64,
}

impl Parameter {
    pub fn new(size: i32) -> Parameter {
        let mut params = Parameter::default();
        params.size = size;
        params
    }

    pub fn get_size(&self) -> i32 {
        self.size
    }

    pub fn get_time_delta(&self) -> f64 {
        self.time_delta
    }

    pub fn get_grid_distance(&self) -> &[f64; 2] {
        &self.grid_distance
    }

    pub fn get_flow_factor(&self) -> f64 {
        self.time_delta * self.pipe_area * self.gravity / self.pipe_length
    }
    pub fn get_sediment_capacity(&self) -> f64 {
        self.sediment_capacity_constant
    }
    pub fn get_dissolving_constant(&self) -> f64 {
        self.dissolving_constant
    }
    pub fn get_deposition_constant(&self) -> f64 {
        self.deposition_constant
    }
    pub fn get_evaporation_constant(&self) -> f64 {
        self.evaporation_constant
    }
}

impl Default for Parameter {
    fn default() -> Self {
        Self {
            size: 0,
            pipe_area: PIPE_AREA,
            pipe_length: PIPE_LENGTH,
            grid_distance: GRID_DISTANCE,
            gravity: GRAVITY,
            time_delta: TIME_DELTA,
            sediment_capacity_constant: SEDIMENT_CAPACITY_CONSTANT,
            dissolving_constant: DISSOLVING_CONSTANT,
            deposition_constant: DEPOSITION_CONSTANT,
            evaporation_constant: EVAPORATION_CONSTANT,
        }
    }
}
