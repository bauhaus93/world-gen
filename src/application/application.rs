use std::{ thread, time, ops::{ Add, Sub } };

use glutin;
use gl;
use gl::types::GLsizei;
use glm::Vector3;

use super::ApplicationError;
use super::window;
use crate::graphics;
use crate::world;
use crate::world::traits::{ Updatable, Translatable };

pub struct Application {
    world: world::World,
    shader_program: graphics::ShaderProgram,
    window: glutin::GlWindow,
    events_loop: glutin::EventsLoop,
    quit: bool,
    time_passed: u32,
    sleep_time: time::Duration
}

impl Application {
    pub fn new(window_size: (f64, f64)) -> Result<Application, ApplicationError> {
        let events_loop = glutin::EventsLoop::new();
        let window = window::init_window(window_size, &events_loop)?;
        let shader_program = graphics::ShaderProgramBuilder::new()
            .add_vertex_shader("resources/shader/VertexShader.glsl")
            .add_fragment_shader("resources/shader/FragmentShader.glsl")
            .finish()?;
        
        let world = world::World::new(5, [128, 128])?;
        let app = Self {
            events_loop: events_loop,
            window: window,
            shader_program: shader_program,
            world: world,
            quit: false,
            time_passed: 0,
            sleep_time: time::Duration::from_millis(50)
        };
        Ok(app)
    }

    pub fn run(mut self) -> Result<(), ApplicationError> {
        self.shader_program.use_program();
        let mut last_time = time::Instant::now();
        while !self.quit {
            self.handle_events();
            self.world.tick(self.time_passed);
            self.render()?;
            self.time_passed = last_time.elapsed().as_secs() as u32 * 1000 + last_time.elapsed().subsec_millis();
            last_time = time::Instant::now();
            self.handle_sleep_time();
            thread::sleep(self.sleep_time);
        }
        Ok(())
    }

    fn handle_events(&mut self) {
        let mut events: Vec<glutin::Event> = Vec::new();
        self.events_loop.poll_events(|event| { events.push(event); });
        for event in events {
            self.handle_event(event);
        }
    }

    fn handle_event(&mut self, event: glutin::Event) {
        match event {
            glutin::Event::WindowEvent { event, .. } => {
                match event {
                    glutin::WindowEvent::CloseRequested => { self.quit = true; },
                    glutin::WindowEvent::Resized(logical_size) => { self.handle_resize(logical_size.into()); },
                    glutin::WindowEvent::KeyboardInput { input, .. } => { self.handle_keyboard_input(input); },
                    glutin::WindowEvent::MouseWheel { delta, phase, .. } => { self.handle_mousewheel(delta, phase); }
                    _ => {}
                }
            },
            _ => {}
        }
    }

    fn handle_resize(&self, new_size: (u32, u32)) {
        unsafe {
            gl::Viewport(0, 0, new_size.0 as GLsizei, new_size.1 as GLsizei);
        }
        info!("Updated viewport to {}/{}/{}/{}", 0, 0, new_size.0, new_size.1);
        match graphics::check_opengl_error("gl::Viewport") {
            Ok(_) => {},
            Err(e) => { warn!("{}", e); }
        } 
    }

    fn handle_keyboard_input(&mut self, input: glutin::KeyboardInput) {
        match (input.virtual_keycode, input.state) {
            (Some(keycode), glutin::ElementState::Pressed) => {
                match keycode {
                    glutin::VirtualKeyCode::A => self.world.move_camera(Vector3::new(-1., 1., 0.)),
                    glutin::VirtualKeyCode::D => self.world.move_camera(Vector3::new(1., -1., 0.)),
                    glutin::VirtualKeyCode::W => self.world.move_camera(Vector3::new(1., 1., 0.)),
                    glutin::VirtualKeyCode::S => self.world.move_camera(Vector3::new(-1., -1., 0.)),
                    glutin::VirtualKeyCode::R => self.world.move_camera(Vector3::new(0., 0., 1.)),
                    glutin::VirtualKeyCode::F => self.world.move_camera(Vector3::new(0., 0., -1.)),
                    glutin::VirtualKeyCode::P => self.world.toggle_camera_projection(),
                    _ => {}
                }
            },
            (_, _) => {}
        }
    }
    fn handle_mousewheel(&mut self, delta: glutin::MouseScrollDelta, phase: glutin::TouchPhase) {
        match phase {
            glutin::TouchPhase::Moved => {
                match delta {
                    glutin::MouseScrollDelta::LineDelta(_, dir) if dir > 0. => { self.world.get_camera_mut().zoom(0.9); },
                    glutin::MouseScrollDelta::LineDelta(_, dir) if dir < 0. => { self.world.get_camera_mut().zoom(1.1); },
                    _ => { warn!("meh."); }
                }
            },
            _ => {}
        }
    }

    fn handle_sleep_time(&mut self) {
        const TARGET_FREQ: u32 = 30;
        let diff: i32 = (self.time_passed * TARGET_FREQ) as i32 - 1000;
        if diff.abs() as u32 > TARGET_FREQ {
            let adj = time::Duration::from_millis(std::cmp::min(std::cmp::max(diff.abs() as u64 / 100, 1), self.sleep_time.subsec_millis() as u64));
            match diff.signum() {
                1 => self.sleep_time = self.sleep_time.sub(adj),
                -1 => self.sleep_time = self.sleep_time.add(adj),
                _ => {}
            }
        } 
    }

    fn render(&mut self) -> Result<(), ApplicationError> {
        unsafe { gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT) }        
        self.world.render(&self.shader_program)?;
        match self.window.swap_buffers() {
            Ok(_) => Ok(()),
            Err(e) => Err(ApplicationError::from(graphics::GraphicsError::from(e)))
        }
    }
}

