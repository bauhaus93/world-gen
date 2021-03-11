use std::convert::TryFrom;

use super::{Chunk, ChunkError, CHUNK_SIZE};
use crate::architect::Architect;
use crate::HeightMap;
use core::graphics::mesh::{Mesh, VertexBuffer};
use core::Point2i;

pub struct ChunkBuilder {
    pos: Point2i,
    lod: u8,
    height_map: HeightMap,
    surface_vertices: VertexBuffer,
}

impl ChunkBuilder {
    pub fn new(pos: Point2i, lod: u8, architect: &Architect) -> Result<Self, ChunkError> {
        let resolution = match lod {
            0 => 1.,
            1 => 4.,
            _ => 8.,
        };
        let height_map = architect.create_heightmap(
            pos,
            (CHUNK_SIZE as f32 / resolution) as i32 + 1,
            resolution,
        );
        let surface_buffer = VertexBuffer::from(
            height_map
                .triangulate()
                .expect("Should not happen lel")
                .as_slice(),
        );
        let builder = Self {
            pos: pos,
            lod: lod,
            height_map: height_map,
            surface_vertices: surface_buffer,
        };
        Ok(builder)
    }

    pub fn finish(self) -> Result<Chunk, ChunkError> {
        let mesh = Mesh::try_from(self.surface_vertices)?;
        let chunk = Chunk::new(self.pos, self.height_map, self.lod, mesh);
        Ok(chunk)
    }
}
