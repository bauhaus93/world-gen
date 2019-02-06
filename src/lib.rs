#![feature(try_from)]

#[macro_use]
extern crate log;
extern crate env_logger;
extern crate gl;
extern crate glutin;
extern crate glm;
extern crate image;
extern crate num_traits;
extern crate rand;
#[macro_use]
extern crate lazy_static;
extern crate futures;
extern crate tokio_threadpool;

pub mod application;
mod graphics;
mod world;
mod utility;

pub use crate::application::Application;
