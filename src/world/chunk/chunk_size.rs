use glm::Vector3;

use crate::utility::Float;

pub const CHUNK_SIZE: [i32; 2] = [128, 128];

pub fn get_chunk_pos(world_pos: Vector3<Float>) -> [i32; 2] {
    [world_pos.x as i32 / CHUNK_SIZE[0],
     world_pos.y as i32 / CHUNK_SIZE[1]]
}
