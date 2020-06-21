use core::Point2f;

pub trait Noise: Sync + Send  {
    fn get_noise(&self, point: Point2f) -> f32;
    fn get_range(&self) -> [f32; 2];
    fn get_cycle(&self) -> Point2f;
    fn is_infinite(&self) -> bool {
        let cycle = self.get_cycle();
        cycle[0].is_infinite() || cycle[1].is_infinite()
    }
}
