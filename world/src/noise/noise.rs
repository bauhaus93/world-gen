
pub trait Noise: Sync + Send {
    fn get_noise(&self, point: [f64; 2]) -> f64;
    fn get_range(&self) -> [f64; 2];
}
