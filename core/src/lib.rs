#[macro_use]
extern crate log;
extern crate gl;
extern crate glutin;
extern crate glm;
extern crate image;
extern crate num_traits;

pub mod traits;
pub mod object;
pub mod graphics;
pub mod config;
pub mod file;
pub mod bounding_box;
pub mod camera;
pub mod cmp;
pub mod core;
pub mod distance;
pub mod float;
pub mod format;
pub mod player;
pub mod skybox;
pub mod sun;
pub mod timer;
pub mod update_error;
pub mod core_error;
mod window;

pub use self::core::Core;
pub use self::core_error::CoreError;

pub use self::bounding_box::BoundingBox;
pub use self::camera::Camera;
pub use self::player::Player;
pub use self::skybox::Skybox;
pub use self::timer::Timer;
pub use self::sun::Sun;

pub use self::traits::Translatable;
pub use self::traits::Rotatable;
pub use self::traits::Scalable;
pub use self::traits::Renderable;
pub use self::traits::Updatable;

pub use self::object::ObjectManager;
pub use self::object::Object;

pub use self::graphics::ShaderProgram;
pub use self::graphics::ShaderProgramBuilder;
pub use self::graphics::Texture;
pub use self::graphics::TextureBuilder;
pub use self::graphics::Mesh;
pub use self::graphics::Model;

pub use self::config::Config;
pub use self::float::Float;
pub use self::update_error::UpdateError;
