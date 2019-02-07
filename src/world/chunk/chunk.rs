
use glm::Vector3;

use crate::graphics::{ ShaderProgram, GraphicsError, mesh::{ Mesh, Vertex, Triangle, Buffer } };
use crate::world::traits::{ Translatable, Renderable };
use crate::world::{ Object, Camera, Noise };
use crate::utility::Float;

const CHUNK_SIZE: [i32; 2] = [128, 128];

pub struct Chunk {
    object: Object
}

impl Chunk {
    pub fn new(pos: [i32; 2], mesh: Mesh) -> Self {
        let mut object = Object::new(mesh);
        object.set_translation(Vector3::new((pos[0] * CHUNK_SIZE[0]) as Float, (pos[1] * CHUNK_SIZE[1]) as Float, 0.));
        Self {
            object: object
        }
    }

    pub fn get_vertex_count(&self) -> u32 {
        self.object.get_vertex_count()
    }
}

impl Renderable for Chunk {
    fn render(&self, camera: &Camera, shader: &ShaderProgram) -> Result<(), GraphicsError> {
        self.object.render(camera, shader)
    }
}

pub fn create_chunk_vertices(chunk_pos: [i32; 2], height_noise: &Noise) -> Buffer {
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
            let absolute_position: [Float; 2] = [(chunk_pos[0] * CHUNK_SIZE[0] + x) as Float,
                                                 (chunk_pos[1] * CHUNK_SIZE[1] + y) as Float];
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
    trace!("Created chunk vertices for {}/{}: triangle count = {}", chunk_pos[0], chunk_pos[1], triangles.len());
    Buffer::from(triangles)
}