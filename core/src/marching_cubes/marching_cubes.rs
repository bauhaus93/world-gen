use std::convert::TryFrom;

use glm::Vector3;

use crate::graphics::mesh::{Mesh, MeshError, Triangle, Vertex};
use crate::Float;

lazy_static! {
    static ref CORNER: [Vector3<i32>; 8] = [
        Vector3::new(0, 0, 0),
        Vector3::new(1, 0, 0),
        Vector3::new(1, 1, 0),
        Vector3::new(0, 1, 0),
        Vector3::new(0, 0, 1),
        Vector3::new(1, 0, 1),
        Vector3::new(1, 1, 1),
        Vector3::new(0, 1, 1),
    ];
}

pub fn create_mesh_by_field(field: &[f64], field_size: [i32; 3]) -> Result<Mesh, MeshError> {
    let triangles = create_triangles(
        field,
        Vector3::new(field_size[0], field_size[1], field_size[2]),
    );
    if triangles.is_empty() {
        Ok(Mesh::default())
    } else {
        Mesh::try_from(triangles.as_slice())
    }
}

fn create_triangles(field: &[f64], field_size: Vector3<i32>) -> Vec<Triangle> {
    let mut triangles = Vec::new();
    for z in 0..field_size.z - 1 {
        for y in 0..field_size.y - 1 {
            for x in 0..field_size.x - 1 {
                let origin = Vector3::new(x, y, z);
                let index = index_for_cube(origin, field_size, &field);
				info!("INDEX = {}", index);
                create_cube_triangles(origin, index, &mut triangles);
            }
        }
    }
    triangles
}

fn create_cube_triangles(origin: Vector3<i32>, index: u8, triangles: &mut Vec<Triangle>) {
    if (0..8).any(|v| 1 << v == index) {
		info!(">>> SINGLE P INDEX = {}", index);
        triangles.push(gen_single_point(origin, index as usize));
    }
}

fn index_for_cube(origin: Vector3<i32>, field_size: Vector3<i32>, field: &[f64]) -> u8 {
    CORNER
        .iter()
        .zip(0..8)
        .map(|(corner, i)| {
            if field[pos_to_index(origin + *corner, field_size)] > 0. {
                (1 << i)
            } else {
                0
            }
        })
        .fold(0, |acc, v| acc | v)
}

fn gen_single_point(cube_origin: Vector3<i32>, corner_index: usize) -> Triangle {
    let vertices = [
        Vertex::new(get_edge_pos(cube_origin, corner_index, 0)),
        Vertex::new(get_edge_pos(cube_origin, corner_index, 1)),
        Vertex::new(get_edge_pos(cube_origin, corner_index, 2)),
    ];
    Triangle::new(vertices)
}

fn get_edge_pos(cube_origin: Vector3<i32>, corner_index: usize, axis: usize) -> Vector3<Float> {
    debug_assert!(corner_index < 8);
    debug_assert!(axis < 3);
    let mut edge = Vector3::new(
        (cube_origin.x + CORNER[corner_index].x) as Float,
        (cube_origin.y + CORNER[corner_index].y) as Float,
        (cube_origin.z + CORNER[corner_index].z) as Float,
    );
    if CORNER[corner_index][axis] == 0 {
        edge[axis] += 0.5;
    } else {
        edge[axis] -= 0.5;
    }
    edge
}

fn pos_to_index(pos: Vector3<i32>, size: Vector3<i32>) -> usize {
    pos.x as usize
        + pos.y as usize * size.x as usize
        + pos.z as usize * size.x as usize * size.y as usize
}
