use std::cmp::Ordering;
use std::fmt;
use glm::{ GenNum, Vector3 };

use utility::{ Float, cmp_vec };

#[derive(Copy, Clone)]
pub struct Vertex {
    pos: Vector3<Float>,
    uv: Vector3<Float>,
}

impl Vertex {

    pub fn get_pos(&self) -> Vector3<Float> {
        self.pos
    }

    pub fn get_uv(&self) -> Vector3<Float> {
        self.uv
    }

    pub fn set_pos(&mut self, new_pos: Vector3<Float>) {
        self.pos = new_pos;
    }

    pub fn set_uv(&mut self, new_uv: Vector3<Float>) {
        self.uv = new_uv;
    }

    pub fn set_uv_layer(&mut self, layer: u32) {
        self.uv.z = layer as Float;
    }

}

impl Default for Vertex {
    fn default() -> Self {
        Vertex {
            pos: Vector3::from_s(0.),
            uv: Vector3::from_s(0.),
        }
    }
}

impl PartialEq for Vertex {
    fn eq(&self, other: &Vertex) -> bool {
        match cmp_vec(&self.pos, &other.pos) {
            Ordering::Equal => {
                match cmp_vec(&self.uv, &other.uv) {
                    Ordering::Equal => true,
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
                    Ordering::Equal => Ordering::Equal,
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
        write!(f, "v = {:.2}/{:.2}/{:.2}, uv = {:.2}/{:.2}/{:.2}",
            self.pos[0], self.pos[1], self.pos[2],
            self.uv[0], self.uv[1], self.uv[2])
    }
}





