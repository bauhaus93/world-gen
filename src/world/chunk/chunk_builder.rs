use std::convert::TryFrom;

use glm::Vector3;

use crate::utility::Float;
use crate::graphics::mesh::{ Vertex, Triangle, Mesh, Buffer };
use crate::world::Noise;
use super::{ Chunk, ChunkError, CHUNK_SIZE };

pub struct ChunkBuilder {
    pos: [i32; 2],
    vertex_buffer: Option<Buffer>
}

impl ChunkBuilder {
    pub fn new(pos: [i32; 2]) -> Self {
        Self {
            pos: pos,
            vertex_buffer: None
        }
    }

    pub fn finish(self) -> Result<Chunk, ChunkError> {
        let mesh = match self.vertex_buffer {
            Some(vb) => Mesh::try_from(vb)?,
            _ => { return Err(ChunkError::NoBufferBuilt(self.pos)); }
        };
        Ok(Chunk::new(self.pos, mesh))
    }

    pub fn create_surface_buffer(&mut self, height_noise: &Noise) {
        const OFFSET: Float = 0.5;
        const VERTEX_OFFSETS: [[Float; 2]; 6] = [
            [OFFSET, -OFFSET],
            [-OFFSET, OFFSET],
            [-OFFSET, -OFFSET],
            [OFFSET, -OFFSET],
            [OFFSET, OFFSET],
            [-OFFSET, OFFSET]
        ];
        let mut triangles: Vec<Triangle> = Vec::with_capacity((CHUNK_SIZE[0] * CHUNK_SIZE[1] * 2) as usize);
        for y in 0..CHUNK_SIZE[1] {
            for x in 0..CHUNK_SIZE[0] {
                let absolute_position: [Float; 2] = [(self.pos[0] * CHUNK_SIZE[0] + x) as Float,
                                                    (self.pos[1] * CHUNK_SIZE[1] + y) as Float];
                for i in 0..2 {
                    let mut vertices: [Vertex; 3] = [Vertex::default(),
                                                    Vertex::default(),
                                                    Vertex::default()];
                    for (vert, off) in vertices.iter_mut().zip(VERTEX_OFFSETS.iter().skip(i * 3).take(3)) {
                        let height = height_noise.get_noise([absolute_position[0] + off[0],
                                                            absolute_position[1] + off[1]]);
                        vert.set_pos(Vector3::new(x as Float + off[0],
                                                y as Float + off[1],
                                                height));
                        vert.set_uv(Vector3::new(0.5 + off[0].signum() * 0.5,
                                                0.5 + off[1].signum() * 0.5,
                                                1.));
                    }
                    triangles.push(Triangle::new(vertices));
                }
            }
        }
        trace!("Created chunk vertices for {}/{}: triangle count = {}", self.pos[0], self.pos[1], triangles.len());
        self.vertex_buffer = Some(Buffer::from(triangles))
    }
}
