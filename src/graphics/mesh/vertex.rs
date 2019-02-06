use std::ops::Add;
use std::cmp::Ordering;
use std::fmt;
use glm::{ GenNum, Vector3, Matrix4, builtin::{ normalize } };

use crate::graphics::{ create_rotation_matrix };
use crate::utility::{ Float, cmp_vec };

#[derive(Copy, Clone)]
pub struct Vertex {
    pos: Vector3<Float>,
    uv: Vector3<Float>,
    normal: Vector3<Float>
}

impl Vertex {

    pub fn get_pos(&self) -> Vector3<Float> {
        self.pos
    }

    pub fn get_uv(&self) -> Vector3<Float> {
        self.uv
    }

    pub fn get_normal(&self) -> Vector3<Float> {
        self.normal
    }

    pub fn set_pos(&mut self, new_pos: Vector3<Float>) {
        self.pos = new_pos;
    }

    pub fn set_uv(&mut self, new_uv: Vector3<Float>) {
        self.uv = new_uv;
    }
    
    pub fn set_normal(&mut self, new_normal: Vector3<Float>) {
        self.normal = new_normal;
    }

    pub fn set_uv_layer(&mut self, layer: u32) {
        self.uv.z = layer as Float;
    }

    pub fn move_pos(&mut self, offset: Vector3<Float>) {
        self.pos = self.pos.add(offset);
    }

    pub fn rotate(&mut self, rotation_matrix: Matrix4<Float>) {
        self.pos = (rotation_matrix * self.pos.extend(1.)).truncate(3);
        self.normal = (rotation_matrix * self.normal.extend(1.)).truncate(3);
    }

    pub fn on_plane(&self, axis: usize, value: Float) -> bool {
        debug_assert!(axis < 3);
        (self.pos[axis] - value).abs() < 1e-3
    }
}

impl Default for Vertex {
    fn default() -> Self {
        Vertex {
            pos: Vector3::from_s(0.),
            uv: Vector3::from_s(0.),
            normal: Vector3::from_s(0.), 
        }
    }
}

impl PartialEq for Vertex {
    fn eq(&self, other: &Vertex) -> bool {
        match cmp_vec(&self.pos, &other.pos) {
            Ordering::Equal => {
                match cmp_vec(&self.uv, &other.uv) {
                    Ordering::Equal => {
                        match cmp_vec(&self.normal, &other.normal) {
                            Ordering::Equal => true,
                            _ => false
                        }
                    },
                    _ => false
                }
            },
            _ => false
        }
    }
}

impl Eq for Vertex {}

impl Ord for Vertex {
    fn cmp(&self, other: &Self) -> Ordering {
        match cmp_vec(&self.pos, &other.pos) {
            Ordering::Equal => {
                match cmp_vec(&self.uv, &other.uv) {
                    Ordering::Equal => {
                        match cmp_vec(&self.normal, &other.normal) {
                            Ordering::Equal => Ordering::Equal,
                            order => order
                        }
                    },
                    order => order
                }
            },
            order => order
        }
    }
}

impl PartialOrd for Vertex {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl fmt::Display for Vertex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "v = {:.2}/{:.2}/{:.2}, uv = {:.2}/{:.2}/{:.2}, n = {:.2}/{:.2}/{:.2}",
            self.pos[0], self.pos[1], self.pos[2],
            self.uv[0], self.uv[1], self.uv[2],
            self.normal[0], self.normal[1], self.normal[2])
    }
}





