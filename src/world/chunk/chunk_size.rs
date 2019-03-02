use glm::Vector3;

use crate::utility::Float;

pub const CHUNK_SIZE: [i32; 2] = [128, 128];

pub fn get_chunk_pos(world_pos: Vector3<Float>) -> [i32; 2] {
    [world_pos.x as i32 / CHUNK_SIZE[0],
     world_pos.y as i32 / CHUNK_SIZE[1]]
}

pub fn get_world_pos(chunk_pos: &[i32; 2], offset: &[i32; 2], resolution: Float) -> [Float; 2] {
    [(chunk_pos[0] * CHUNK_SIZE[0]) as Float + offset[0] as Float * resolution,
     (chunk_pos[1] * CHUNK_SIZE[1]) as Float + offset[1] as Float * resolution]
}
