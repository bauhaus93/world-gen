use crate::world::noise::Noise;

pub struct ChunkLoader {
    height_noise: Box<Noise>
}

impl ChunkLoader {
    pub fn new(height_noise: Box<Noise>) -> ChunkLoader {
        Self {
            height_noise: height_noise
        }
    }
}