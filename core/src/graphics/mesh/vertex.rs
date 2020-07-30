use std::cmp::{Ord, Ordering};
use std::fmt;

use crate::{Point2f, Point3f};

#[derive(Copy, Clone)]
enum UV {
    Dim2(Point2f),
    Dim3(Point3f),
}

#[derive(Copy, Clone)]
pub struct Vertex {
    pos: Point3f,
    normal: Point3f,
    uv: UV,
}

impl Vertex {
    pub fn get_pos(&self) -> Point3f {
        self.pos
    }

    pub fn get_uv(&self) -> Point2f {
        match self.uv {
            UV::Dim2(uv) => uv,
            UV::Dim3(uv) => uv.as_xy(),
        }
    }

    pub fn get_uv_layered(&self) -> Point3f {
        match self.uv {
            UV::Dim2(uv) => uv.extend(0.),
            UV::Dim3(uv) => uv,
        }
    }

    pub fn get_normal(&self) -> Point3f {
        self.normal
    }

    pub fn set_pos(&mut self, new_pos: Point3f) {
        self.pos = new_pos;
    }

    pub fn set_uv(&mut self, new_uv: Point2f) {
        match &mut self.uv {
            UV::Dim2(uv) => {
                *uv = new_uv;
            }
            UV::Dim3(uv) => {
                uv[0] = new_uv[0];
                uv[1] = new_uv[1];
            }
        }
    }

    pub fn set_normal(&mut self, new_normal: Point3f) {
        self.normal = new_normal;
    }

    //extends uv to 3d, if 2d uv existing
    pub fn set_uv_layer(&mut self, layer: u32) {
        let new_uv = match self.uv {
            UV::Dim3(uv) => {
                let mut new_uv = uv;
                new_uv[2] = layer as f32;
                new_uv
            }
            UV::Dim2(uv) => uv.extend(layer as f32),
        };
        self.uv = UV::Dim3(new_uv);
    }

    pub fn get_uv_dim(&self) -> u8 {
        match self.uv {
            UV::Dim2(_) => 2,
            UV::Dim3(_) => 3,
        }
    }
}

impl Default for Vertex {
    fn default() -> Self {
        Vertex {
            pos: Point3f::from_scalar(0.),
            uv: UV::Dim2(Point2f::from_scalar(0.)),
            normal: Point3f::from_scalar(0.),
        }
    }
}

impl PartialEq for Vertex {
    fn eq(&self, other: &Vertex) -> bool {
        match self.pos.cmp(&other.pos) {
            Ordering::Equal => match (&self.uv, &other.uv) {
                (UV::Dim2(a), UV::Dim2(b)) => a.cmp(b) == Ordering::Equal,
                (UV::Dim3(a), UV::Dim3(b)) => a.cmp(b) == Ordering::Equal,
                (_, _) => {
                    panic!("Wanted to compare vertex with different uv dimensions.");
                }
            },
            _ => false,
        }
    }
}

impl Eq for Vertex {}

impl Ord for Vertex {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.pos.cmp(&other.pos) {
            Ordering::Equal => match (&self.uv, &other.uv) {
                (UV::Dim2(a), UV::Dim2(b)) => a.cmp(b),
                (UV::Dim3(a), UV::Dim3(b)) => a.cmp(b),
                (_, _) => {
                    panic!("Wanted to compare vertex with different uv dimensions.");
                }
            },
            order => order,
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
            UV::Dim2(uv) => write!(
                f,
                "v = {:.2}/{:.2}/{:.2}, uv = {:.2}/{:.2}",
                self.pos[0], self.pos[1], self.pos[2], uv[0], uv[1]
            ),
            UV::Dim3(uv) => write!(
                f,
                "v = {:.2}/{:.2}/{:.2}, uv = {:.2}/{:.2}/{:.2}",
                self.pos[0], self.pos[1], self.pos[2], uv[0], uv[1], uv[2]
            ),
        }
    }
}
