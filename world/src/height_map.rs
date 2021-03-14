use std::cmp::Ordering;
use std::convert::TryInto;

use crate::{chunk::ChunkError, triangulate, Noise};
use core::graphics::mesh::Triangle;
use core::{GraphicsError, Mesh, Point2f, Point2i, Point3f, Texture, TextureBuilder};

#[derive(Clone)]
pub struct HeightMap {
    size: i32,
    scale_factor: f32,
    origin: Point2f,
    height_list: Vec<f32>,
    normal_list: Vec<Point3f>,
}

impl HeightMap {
    pub fn new(size: i32, scale_factor: f32) -> Self {
        debug_assert!(size > 0);
        let mut height_list = Vec::new();
        height_list.resize((size * size) as usize, 0.);
        let mut normal_list = Vec::new();
        normal_list.resize((size * size) as usize, Point3f::new(0., 0., 1.));
        Self {
            size: size,
            scale_factor: scale_factor,
            origin: Point2f::from_scalar(0.),
            height_list: height_list,
            normal_list: normal_list,
        }
    }

    pub fn from_noise(origin: Point2f, size: i32, scale_factor: f32, noise: &dyn Noise) -> Self {
        debug_assert!(size > 0);

        let mut height_list = Vec::with_capacity((size * size) as usize);
        for y in 0..size {
            for x in 0..size {
                let h = noise.get_noise(
                    origin + Point2f::new(x as f32 * scale_factor, y as f32 * scale_factor),
                );
                height_list.push(h);
            }
        }

        let mut normal_list = Vec::with_capacity((size * size) as usize);
        normal_list.resize((size * size) as usize, Point3f::new(0., 0., 1.));
        let mut hm = Self {
            size: size,
            scale_factor: scale_factor,
            origin: origin,
            height_list: height_list,
            normal_list: normal_list,
        };
        hm.update_normals(noise);
        hm
    }

    fn update_normals(&mut self, fallback_noise: &dyn Noise) {
        for y in 0..self.size {
            for x in 0..self.size {
                let r = if x + 1 == self.size {
                    fallback_noise.get_noise(
                        Point2f::new(
                            (x + 1) as f32 * self.scale_factor,
                            y as f32 * self.scale_factor,
                        ) + self.origin,
                    )
                } else {
                    self.get(Point2i::new(x + 1, y))
                };
                let l = if x == 0 {
                    fallback_noise.get_noise(
                        Point2f::new(
                            (x - 1) as f32 * self.scale_factor,
                            y as f32 * self.scale_factor,
                        ) + self.origin,
                    )
                } else {
                    self.get(Point2i::new(x - 1, y))
                };
                let b = if y + 1 == self.size {
                    fallback_noise.get_noise(
                        Point2f::new(
                            x as f32 * self.scale_factor,
                            (y + 1) as f32 * self.scale_factor,
                        ) + self.origin,
                    )
                } else {
                    self.get(Point2i::new(x, y + 1))
                };
                let t = if y == 0 {
                    fallback_noise.get_noise(
                        Point2f::new(
                            x as f32 * self.scale_factor,
                            (y - 1) as f32 * self.scale_factor,
                        ) + self.origin,
                    )
                } else {
                    self.get(Point2i::new(x, y - 1))
                };

                let normal = Point3f::new((r - l) / (2.), (b - t) / (2. * self.scale_factor), 1.)
                    .as_normalized();
                self.set_normal(Point2i::new(x, y), normal);
            }
        }
    }

    pub fn get_interpolated_height(&self, relative_pos: Point2f) -> f32 {
        let root_pos = Point2i::new(
            clamp(
                (relative_pos[0].floor() / self.scale_factor) as i32,
                0,
                self.size - 1,
            ),
            clamp(
                (relative_pos[1].floor() / self.scale_factor) as i32,
                0,
                self.size - 1,
            ),
        );
        let reference_height: [f32; 4] = [
            self.get(root_pos),
            self.get(Point2i::new(
                i32::min(root_pos[0] + 1, self.size - 1),
                root_pos[1],
            )),
            self.get(Point2i::new(
                root_pos[0],
                i32::min(root_pos[1] + 1, self.size - 1),
            )),
            self.get(Point2i::new(
                i32::min(root_pos[0] + 1, self.size - 1),
                i32::min(root_pos[1] + 1, self.size - 1),
            )),
        ];
        let relative_point = relative_pos - Point2f::from(root_pos);

        let res = interpolate(relative_point, reference_height);
        res
    }

    pub fn triangulate(&self) -> Option<Vec<Triangle>> {
        let mut points = Vec::new();
        for y in 0..self.size {
            for x in 0..self.size {
                let point = Point3f::new(
                    x as f32 * self.scale_factor,
                    y as f32 * self.scale_factor,
                    self.get(Point2i::new(x, y)),
                );
                points.push(point);
            }
        }
        match triangulate(&points) {
            Some(mut triangulation) => {
                for triangle in triangulation.iter_mut() {
                    for v in triangle.get_vertices_mut() {
                        let pos = Point2i::from(v.get_pos().as_xy() / self.scale_factor);
                        v.set_normal(self.get_normal(pos));
                    }
                }
                Some(triangulation)
            }
            None => None,
        }
    }

    #[allow(unused)]
    pub fn get_size(&self) -> i32 {
        self.size
    }

    pub fn get_list(&self) -> &[f32] {
        self.height_list.as_slice()
    }

    pub fn get_min(&self) -> f32 {
        match self.height_list.iter().min_by(|a, b| {
            if a > b {
                Ordering::Greater
            } else if a < b {
                Ordering::Less
            } else {
                Ordering::Equal
            }
        }) {
            Some(value) => *value,
            None => unreachable!(),
        }
    }

    pub fn get_max(&self) -> f32 {
        match self.height_list.iter().max_by(|a, b| {
            if a > b {
                Ordering::Greater
            } else if a < b {
                Ordering::Less
            } else {
                Ordering::Equal
            }
        }) {
            Some(value) => *value,
            None => unreachable!(),
        }
    }

    pub fn set(&mut self, pos: Point2i, height: f32) {
        let index = self.calculate_index(pos);
        self.height_list[index] = height;
    }

    pub fn get(&self, pos: Point2i) -> f32 {
        self.height_list[self.calculate_index(pos)]
    }
    pub fn get_normal(&self, pos: Point2i) -> Point3f {
        self.normal_list[self.calculate_index(pos)]
    }
    pub fn set_normal(&mut self, pos: Point2i, normal: Point3f) {
        let index = self.calculate_index(pos);
        self.normal_list[index] = normal;
    }
    #[allow(unused)]
    pub fn set_by_index(&mut self, index: usize, height: f32) {
        self.height_list[index] = height;
    }

    #[allow(unused)]
    pub fn get_by_index(&self, index: usize) -> f32 {
        debug_assert!(index < self.height_list.len());
        self.height_list[index]
    }

    #[allow(unused)]
    fn get_quad_heights(&self, anchor: Point2i) -> [f32; 4] {
        [
            self.get(anchor),
            self.get(anchor + Point2i::new(1, 0)),
            self.get(anchor + Point2i::new(0, 1)),
            self.get(anchor + Point2i::new(1, 1)),
        ]
    }

    fn calculate_index(&self, pos: Point2i) -> usize {
        debug_assert!(pos[0] >= 0 && pos[1] >= 0);
        ((pos[0] % self.size) + self.size * (pos[1] % self.size)) as usize
    }
}

impl TryInto<Texture> for HeightMap {
    type Error = GraphicsError;
    fn try_into(self) -> Result<Texture, Self::Error> {
        let texture = TextureBuilder::new_2d(Point2i::from_scalar(self.size))
            .format_rgba32f()
            .finish()?;
        let height_normal_list: Vec<f32> = self
            .height_list
            .iter()
            .zip(self.normal_list.iter())
            .fold(Vec::new(), |mut acc, (h, n)| {
                acc.extend(&[*h, n[0], n[1], n[2]]);
                acc
            });
        texture.write_data(height_normal_list.as_slice())?;
        Ok(texture)
    }
}

impl TryInto<Mesh> for HeightMap {
    type Error = ChunkError;
    fn try_into(self) -> Result<Mesh, Self::Error> {
        self.triangulate()
            .ok_or(ChunkError::HeightmapTriangulation)
            .and_then(|t| t.as_slice().try_into().map_err(ChunkError::from))
    }
}

fn interpolate(p: Point2f, reference: [f32; 4]) -> f32 {
    let anchor = [p[0].floor() as i32, p[1].floor() as i32];
    let a = anchor[0] as f32 + 1. - p[0];
    let b = p[0] - anchor[0] as f32;
    let r_1 = a * reference[0] + b * reference[1];
    let r_2 = a * reference[2] + b * reference[3];
    let c = anchor[1] as f32 + 1. - p[1];
    let d = p[1] - anchor[1] as f32;
    c * r_1 + d * r_2
}

fn clamp<T>(value: T, min: T, max: T) -> T
where
    T: Ord,
{
    T::min(T::max(value, min), max)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn check_heightmap_triangulation() {
        let hm = HeightMap::new(16, 1.);
        assert!(hm.triangulate().is_some());
    }
}
