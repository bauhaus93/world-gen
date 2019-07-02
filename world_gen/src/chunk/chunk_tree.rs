use std::rc::Rc;
use glm::{ Vector2, Vector3 };

use utility::Float;
use graphics::{ GraphicsError, ShaderProgram };
use crate::{ BoundingBox, Visibility, Camera, Model, traits::{ Renderable, Translatable } };
use super::{ Chunk, CHUNK_SIZE };

pub struct ChunkTree {
    model: Model,
    bounding_box: BoundingBox<i32>,
    node: Node
}

enum Node {
    Leaf(Option<Rc<Chunk>>),
    Branch(Box<[ChunkTree; 4]>)
}

impl ChunkTree {
    pub fn new(center: Vector2<i32>, size: i32) -> ChunkTree {
        let mut model = Model::default();
        let node = if size == 1 {
            Node::Leaf(None)
        } else {
            create_branch(center, size)
        };
        model.set_translation(Vector3::new((center.x * CHUNK_SIZE) as Float, (center.y * CHUNK_SIZE) as Float, 0.));
        ChunkTree {
            model: model,
            bounding_box: BoundingBox::new(center.extend(0), size),
            node: node
        }
    }

    pub fn rebuild(&self, new_center: Vector2<i32>) -> ChunkTree {

    }

    pub fn insert(&mut self, chunk: Rc<Chunk>) {
        if self.bounding_box.get_size() == 1 {
            self.set_leaf(chunk);
        } else {
            self.pass_down(chunk);
        }
    }

    pub fn contains(&self, chunk_pos: Vector2<i32>) -> bool {
        match self.node {
            Node::Branch(children) => {
                let index = self.calculate_index(chunk_pos);
                children[index].contains(chunk_pos)
            },
            Node::Leaf(Some(chunk)) => {
                true
            },
            Node::Leaf(None) => {
                false
            }
        }
    }

    fn set_leaf(&mut self, chunk: Rc<Chunk>) {
        self.node = Node::Leaf(Some(chunk));
    }

    fn pass_down(&mut self, chunk: Rc<Chunk>) {
        let chunk_pos = chunk.get_pos();
        let chunk_center = Vector2::new(chunk_pos[0], chunk_pos[1]);
        let index = self.calculate_index(chunk_center);

        match self.node {
            Node::Branch(ref mut children) => children[index].insert(chunk),
            _ => unreachable!("Tree must not be leaf at this place")
        }
    }

    fn calculate_index(&self, chunk_center: Vector2<i32>) -> usize {
        let center = self.bounding_box.get_center_xy();
        match (center, chunk_center) {
            (c, cc) if cc.x < c.x && cc.y < c.y => 0,
            (c, cc) if cc.x >= c.x && cc.y < c.y => 1,
            (c, cc) if cc.x < c.x && cc.y >= c.y => 2,
            _ => 3
        }
    }

    fn render_cascade(&self, camera: &Camera, shader: &ShaderProgram, lod: u8) -> Result<(), GraphicsError> {
        match self.node {
            Node::Branch(ref children) => children.iter().try_for_each(|child| child.render_cascade(camera, shader, lod)),
            Node::Leaf(Some(chunk)) => chunk.render(camera, shader, lod),
            _ => Ok(())
        }
    }

    fn render_cascade_check(&self, camera: &Camera, shader: &ShaderProgram, lod: u8) -> Result<(), GraphicsError> {
        match self.node {
            Node::Branch(ref children) => {
                children.iter().try_for_each(|child| {
                    match child.check_visibility(camera) {
                        Visibility::Inside => child.render_cascade(camera, shader, lod),
                        Visibility::Intersection => child.render_cascade_check(camera, shader, lod),
                        Visibility::Outside => Ok(())
                    }
                })
            },
            Node::Leaf(Some(chunk)) => chunk.render(camera, shader, lod),
            _ => Ok(())
        }
    }

    fn check_visibility(&self, camera: &Camera) -> Visibility {
        let mvp = camera.create_mvp_matrix(&self.model);
        self.bounding_box.check_visibility_scaled(mvp, CHUNK_SIZE as Float)
    }
}

fn calculate_child_center(center: Vector2<i32>, size: i32, index: usize) -> Vector2<i32> {
    let size_quarter = size / 4;
    match index {
        0 => center - size_quarter,
        1 => Vector2::new(center.x + size_quarter, center.y - size_quarter),
        2 => Vector2::new(center.x - size_quarter, center.y + size_quarter),
        3 => center + size_quarter,
        _ => unreachable!("Only indices between 0-3 allowed")
    }
}

fn create_branch(center: Vector2<i32>, size: i32) -> Node {
    let children_size = size / 2;
    let children = [
        ChunkTree::new(calculate_child_center(center, size, 0), children_size),
        ChunkTree::new(calculate_child_center(center, size, 1), children_size),
        ChunkTree::new(calculate_child_center(center, size, 2), children_size),
        ChunkTree::new(calculate_child_center(center, size, 3), children_size),
    ];
    Node::Branch(Box::new(children))
}

impl Renderable for ChunkTree {
    fn render(&self, camera: &Camera, shader: &ShaderProgram, lod: u8) -> Result<(), GraphicsError> {
        match self.node {
            Node::Branch(ref children) => {
                match self.check_visibility(camera) {
                    Visibility::Inside => self.render_cascade(camera, shader, lod),
                    Visibility::Intersection => self.render_cascade_check(camera, shader, lod),
                    Visibility::Outside => Ok(())
                }
            },
            Node::Leaf(Some(chunk)) => chunk.render(camera, shader, lod),
            _ => Ok(())
        }
    }
}
