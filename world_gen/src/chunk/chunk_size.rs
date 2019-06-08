use glm::Vector3;

use utility::Float;

pub const CHUNK_SIZE: i32 = 64;

pub fn get_chunk_pos(world_pos: Vector3<Float>) -> [i32; 2] {
    [world_pos.x as i32 / CHUNK_SIZE,
     world_pos.y as i32 / CHUNK_SIZE]
}

#[allow(dead_code)]
pub fn get_world_pos(chunk_pos: &[i32; 2], offset: &[i32; 2], resolution: i32) -> [Float; 2] {
    [((chunk_pos[0] * CHUNK_SIZE) + offset[0] * resolution) as Float,
     ((chunk_pos[1] * CHUNK_SIZE) + offset[1] * resolution) as Float]
}
