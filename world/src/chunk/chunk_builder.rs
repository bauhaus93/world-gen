use super::{Chunk, ChunkError};
use crate::architect::Architect;
use crate::HeightMap;
use core::Point2i;

pub struct ChunkBuilder {
    pos: Point2i,
    heightmap: HeightMap,
}

impl ChunkBuilder {
    pub fn new(pos: Point2i, architect: &Architect) -> Result<Self, ChunkError> {
        let heightmap = architect.create_heightmap(pos);
        let builder = Self {
            pos: pos,
            heightmap: heightmap,
        };
        Ok(builder)
    }

    pub fn finish(self) -> Result<Chunk, ChunkError> {
        Chunk::new(self.pos, self.heightmap)
    }
}
