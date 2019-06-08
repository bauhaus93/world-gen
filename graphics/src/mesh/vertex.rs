use std::cmp::Ordering;
use std::fmt;
use glm::{ GenNum, Vector2, Vector3 };

use utility::{ Float, cmp_vec2, cmp_vec3 };

#[derive(Copy, Clone)]
enum UV {
    Dim2(Vector2<Float>),
    Dim3(Vector3<Float>)
}

#[derive(Copy, Clone)]
pub struct Vertex {
    pos: Vector3<Float>,
    uv: UV,
}

impl Vertex {

    pub fn get_pos(&self) -> Vector3<Float> {
        self.pos
    }

    pub fn get_uv(&self) -> Vector2<Float> {
        match self.uv {
            UV::Dim2(uv) => uv,
            UV::Dim3(uv) => uv.truncate(2)
        }
    }

    pub fn get_uv_layered(&self) -> Vector3<Float> {
        match self.uv {
            UV::Dim2(uv) => uv.extend(0.),
            UV::Dim3(uv) => uv
        }
    }

    pub fn set_pos(&mut self, new_pos: Vector3<Float>) {
        self.pos = new_pos;
    }

    pub fn set_uv(&mut self, new_uv: Vector2<Float>) {
        match &mut self.uv {
            UV::Dim2(uv) => {
                *uv = new_uv;
            },
            UV::Dim3(uv) => {
                uv[0] = new_uv[0];
                uv[1] = new_uv[1];
            }
        }
    }

    //extends uv to 3d, if 2d uv existing
    pub fn set_uv_layer(&mut self, layer: u32) {
        let new_uv = match self.uv {
            UV::Dim3(uv) => {
                let mut new_uv = uv;
                new_uv.z = layer as Float;
                new_uv
            },
            UV::Dim2(uv) => {
                uv.extend(layer as Float)
            }
        };
        self.uv = UV::Dim3(new_uv);
    }

    pub fn get_uv_dim(&self) -> u8 {
        match self.uv {
            UV::Dim2(_) => 2,
            UV::Dim3(_) => 3
        }
    }

}

impl Default for Vertex {
    fn default() -> Self {
        Vertex {
            pos: Vector3::from_s(0.),
            uv: UV::Dim2(Vector2::from_s(0.)),
        }
    }
}

impl PartialEq for Vertex {
    fn eq(&self, other: &Vertex) -> bool {
        match cmp_vec3(&self.pos, &other.pos) {
            Ordering::Equal => {
                match (&self.uv, &other.uv) {
                    (UV::Dim2(a), UV::Dim2(b)) => cmp_vec2(a, b) == Ordering::Equal,
                    (UV::Dim3(a), UV::Dim3(b)) => cmp_vec3(a, b) == Ordering::Equal,
                    (_, _) => {
                        panic!("Wanted to compare vertex with different uv dimensions.");
                    }
                }
            },
            _ => false
        }
    }
}

impl Eq for Vertex {}

impl Ord for Vertex {
    fn cmp(&self, other: &Self) -> Ordering {
        match cmp_vec3(&self.pos, &other.pos) {
            Ordering::Equal => {
                match (&self.uv, &other.uv) {
                    (UV::Dim2(a), UV::Dim2(b)) => cmp_vec2(a, b),
                    (UV::Dim3(a), UV::Dim3(b)) => cmp_vec3(a, b),
                    (_, _) => {
                        panic!("Wanted to compare vertex with different uv dimensions.");
                    }
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
        match &self.uv {
            UV::Dim2(uv) => {
                write!(f, "v = {:.2}/{:.2}/{:.2}, uv = {:.2}/{:.2}",
                    self.pos[0], self.pos[1], self.pos[2],
                    uv[0], uv[1])
            },
            UV::Dim3(uv) => {
                write!(f, "v = {:.2}/{:.2}/{:.2}, uv = {:.2}/{:.2}/{:.2}",
                    self.pos[0], self.pos[1], self.pos[2],
                    uv[0], uv[1], uv[2])
            }
        }

    }
}





