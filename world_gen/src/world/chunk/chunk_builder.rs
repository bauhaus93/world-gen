use std::convert::TryFrom;
use std::cmp::{ min, max };

use glm::{ Vector3 };

use utility::Float;
use graphics::mesh::{ Vertex, Triangle, Mesh, Buffer };
use super::{ Chunk, ChunkError, height_map::HeightMap };
use super::chunk_size::{ CHUNK_SIZE, get_world_pos };

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

    pub fn create_surface_buffer(&mut self, height_map: &HeightMap) {
        let mut triangles: Vec<Triangle> = Vec::with_capacity((CHUNK_SIZE[0] * CHUNK_SIZE[1] * 2) as usize);
        for y in 0..CHUNK_SIZE[1] {
            for x in 0..CHUNK_SIZE[0] {
                triangles.extend(&add_quad_triangles(&self.pos, &[x, y], height_map));
            }
        }
        trace!("Created chunk vertices for {}/{}: triangle count = {}", self.pos[0], self.pos[1], triangles.len());
        self.vertex_buffer = Some(Buffer::from(triangles))
    }
}

fn add_quad_triangles(chunk_pos: &[i32; 2], offset: &[i32; 2], height_map: &HeightMap) -> [Triangle; 2] {
    const OFFSET: Float = 1.;
    const VERTEX_OFFSETS: [[Float; 2]; 6] = [
        [0., 0.],         [OFFSET, OFFSET], [0., OFFSET],
        [OFFSET, OFFSET], [0., 0.],         [OFFSET, 0.]
    ];
    let mut triangles = [Triangle::default(),
                         Triangle::default()];

    for i in 0..2 {
        let mut vertices: [Vertex; 3] = [Vertex::default(),
                                         Vertex::default(),
                                         Vertex::default()];
        let texture_layer = 1.;
        for (vert, off) in vertices.iter_mut().zip(VERTEX_OFFSETS.iter().skip(i * 3).take(3)) {
                let map_pos = [offset[0] + max(0, min(1, off[0] as i32)),
                               offset[1] + max(0, min(1, off[1] as i32))];
                let height = height_map.get(&map_pos);
                vert.set_pos(Vector3::new(offset[0] as Float + off[0],
                                          offset[1] as Float + off[1],
                                          height));
                debug_assert!(off[0] <= 1., off[1] <= 1.);
                vert.set_uv(Vector3::new(off[0],
                                         off[1],
                                         texture_layer as Float));
        }
        triangles[i] = Triangle::new(vertices);
    }
    triangles
}