#[macro_use]
extern crate log;
extern crate gl;
extern crate glm;
extern crate glutin;
extern crate image;
extern crate num_traits;

pub mod bounding_box;
pub mod camera;
pub mod cmp;
pub mod config;
pub mod core;
pub mod core_error;
pub mod distance;
pub mod file;
pub mod float;
pub mod format;
pub mod graphics;
pub mod object;
pub mod player;
pub mod point;
pub mod skybox;
pub mod sun;
pub mod timer;
pub mod traits;
pub mod update_error;
pub mod seed;
mod window;

pub use self::core::Core;
pub use self::core_error::CoreError;

pub use self::bounding_box::BoundingBox;
pub use self::camera::Camera;
pub use self::player::Player;
pub use self::skybox::Skybox;
pub use self::sun::Sun;
pub use self::timer::Timer;

pub use self::traits::Renderable;
pub use self::traits::Rotatable;
pub use self::traits::Scalable;
pub use self::traits::Translatable;
pub use self::traits::Updatable;

pub use self::object::Object;
pub use self::object::ObjectManager;

pub use self::graphics::Mesh;
pub use self::graphics::Model;
pub use self::graphics::ShaderProgram;
pub use self::graphics::ShaderProgramBuilder;
pub use self::graphics::Texture;
pub use self::graphics::TextureBuilder;

pub use self::config::Config;
pub use self::float::Float;
pub use self::update_error::UpdateError;

pub use self::point::{Point2f, Point2i, Point3f};
pub use self::seed::Seed;
