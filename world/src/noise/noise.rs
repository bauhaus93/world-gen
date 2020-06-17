use core::Point2f;

pub trait Noise: Sync + Send {
    fn get_noise(&self, point: Point2f) -> f32;
    fn get_range(&self) -> [f32; 2];
}
