use glm::Vector3;

use crate::graphics::mesh::{Triangle, Vertex};
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

pub fn build_triangles(field_values: Vec<f64>, field_size: Vector3<i32>) -> Vec<Triangle> {
    let mut triangles = Vec::new();
    for z in 0..field_size.z {
        for y in 0..field_size.y {
            for x in 0..field_size.x {
                let base = Vector3::new(x, y, z);
                let value: u8 = CORNER
                    .iter()
                    .zip(0..8)
                    .map(|(corner, i)| {
                        if field_values[pos_to_index(base + *corner, field_size)] > 0. {
                            (1 << i)
                        } else {
                            0
                        }
                    })
                    .fold(0, |acc, v| acc | v);
                if (0..8).any(|v| 1 << v == value) {
                    triangles.push(gen_single_point(base, value as usize));
                }
            }
        }
    }
    Vec::new()
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
