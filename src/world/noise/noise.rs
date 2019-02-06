pub trait Noise {
    fn get_noise(&self, point: (f32, f32)) -> f32;
    fn get_range(&self) -> (f32, f32);
}
