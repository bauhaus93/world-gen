use crate::utility::Float;

pub trait Noise {
    fn get_noise(&self, point: [Float; 2]) -> Float;
    fn get_range(&self) -> [Float; 2];
}
