use std::{ thread, time, ops::{ Add, Sub } };

use glutin;
use gl;
use gl::types::GLsizei;
use glm::{ GenNum, Vector3, normalize, length, cross };

use crate::graphics;
use crate::world;
use crate::world::traits::{ Updatable, Translatable, Rotatable };
use crate::utility::Float;
use super::ApplicationError;
use super::window;

pub struct Application {
    world: world::World,
    window: glutin::GlWindow,
    events_loop: glutin::EventsLoop,
    window_size: [f64; 2],
    quit: bool,
    time_passed: u32,
    sleep_time: time::Duration,
    movement_keys_down: [bool; 5]
}

impl Application {
    pub fn new(window_size: [f64; 2]) -> Result<Application, ApplicationError> {
        let events_loop = glutin::EventsLoop::new();
        let window = window::init_window(window_size, &events_loop)?;
        
        let world = world::World::new()?;
        let app = Self {
            events_loop: events_loop,
            window: window,
            world: world,
            window_size: window_size,
            quit: false,
            time_passed: 0,
            sleep_time: time::Duration::from_millis(50),
            movement_keys_down: [false; 5]
        };
        Ok(app)
    }

    pub fn run(mut self) -> Result<(), ApplicationError> {
        let mut last_time = time::Instant::now();
        while !self.quit {
            self.handle_events();
            self.handle_movement();
            self.world.tick(self.time_passed)?;
            self.render()?;
            self.time_passed = last_time.elapsed().as_secs() as u32 * 1000 + last_time.elapsed().subsec_millis();
            last_time = time::Instant::now();
            self.update_sleep_time();
            thread::sleep(self.sleep_time);
        }
        Ok(())
    }

    fn handle_events(&mut self) {
        let events = self.collect_events();
        self.process_events(&events);
    }

    fn collect_events(&mut self) -> Vec<glutin::Event> {
        let mut events: Vec<glutin::Event> = Vec::new();
        self.events_loop.poll_events(|event| { events.push(event); });
        events
    }

    fn process_events(&mut self, events: &[glutin::Event]) {
        let mut keys_pressed: Vec<(glutin::VirtualKeyCode, bool)> = Vec::new();
        for event in events {
            match event {
                glutin::Event::WindowEvent { event, .. } => {
                    match event {
                        glutin::WindowEvent::CloseRequested => { self.quit = true; },
                        glutin::WindowEvent::Resized(logical_size) => { self.handle_resize((*logical_size).into()); },
                        glutin::WindowEvent::KeyboardInput { input, .. } => {
                            if let Some(key) = get_keycode(*input) {
                                keys_pressed.push(key);
                            }
                        },
                        glutin::WindowEvent::MouseWheel { delta, phase, .. } => { self.handle_mousewheel(*delta, *phase); }
                        _ => {}
                    }
                },
                glutin::Event::DeviceEvent { event, .. } => {
                    match event {
                        glutin::DeviceEvent::MouseMotion { delta } => { self.handle_mouse_movement(*delta); },
                        _ => {}
                    }
                },
                _ => {}
            }
        }
        self.handle_pressed_keys(&keys_pressed);
    }

    fn handle_mouse_movement(&mut self, delta: (f64, f64)) {
        if let Err(msg) = self.window.set_cursor_position(glutin::dpi::LogicalPosition::new(self.window_size[0] / 2., self.window_size[1] / 2.)) {
            warn!("window.set_cursor position: {}", msg);
        }
        let offset = Vector3::new(-delta.0 as Float, delta.1 as Float, 0.);
        self.world.get_camera_mut().mod_rotation(offset * 0.025 * (self.time_passed as Float / 1000.));
    }

    fn handle_movement(&mut self) {
        let mut move_offset: Vector3<Float> = Vector3::from_s(0.);
        if self.movement_keys_down[0] {
            move_offset = move_offset.add(self.world.get_camera().get_direction());
        }
        if self.movement_keys_down[1] {
            let right = cross(self.world.get_camera().get_direction(), Vector3::new(0., 0., 1.));
            move_offset = move_offset.sub(right);
        }
        if self.movement_keys_down[2] {
            move_offset = move_offset.sub(self.world.get_camera().get_direction());
        }
        if self.movement_keys_down[3] {
            let right = cross(self.world.get_camera().get_direction(), Vector3::new(0., 0., 1.));
            move_offset = move_offset.add(right);
        }
        if self.movement_keys_down[4] {
            move_offset = move_offset.add(Vector3::new(0., 0., 1.));
        }
        if length(move_offset) > 1e-3 {
            const SPEED: Float = 10.;
            self.world.get_camera_mut().mod_translation(normalize(move_offset) * SPEED);
        }
    }

    fn handle_pressed_keys(&mut self, key_list: &[(glutin::VirtualKeyCode, bool)]) {
        for (key, down) in key_list {
            match key {
                glutin::VirtualKeyCode::W => { self.movement_keys_down[0] = *down; },
                glutin::VirtualKeyCode::A => { self.movement_keys_down[1] = *down; },
                glutin::VirtualKeyCode::S => { self.movement_keys_down[2] = *down; },
                glutin::VirtualKeyCode::D => { self.movement_keys_down[3] = *down; },
                glutin::VirtualKeyCode::Space => { self.movement_keys_down[4] = *down; },
                glutin::VirtualKeyCode::P if *down => { self.world.toggle_camera_projection(); },
                _ => {}
            }
        }
    }

    fn handle_resize(&mut self, new_size: (u32, u32)) {
        unsafe { gl::Viewport(0, 0, new_size.0 as GLsizei, new_size.1 as GLsizei); }
        self.window_size = [new_size.0 as f64, new_size.1 as f64];
        info!("Updated viewport to {}/{}/{}/{}", 0, 0, new_size.0, new_size.1);
        match graphics::check_opengl_error("gl::Viewport") {
            Ok(_) => {},
            Err(e) => { warn!("{}", e); }
        } 
    }


    fn handle_mousewheel(&mut self, delta: glutin::MouseScrollDelta, phase: glutin::TouchPhase) {
        match phase {
            glutin::TouchPhase::Moved => {
                match delta {
                    glutin::MouseScrollDelta::LineDelta(_, dir) if dir > 0. => { },
                    glutin::MouseScrollDelta::LineDelta(_, dir) if dir < 0. => { },
                    _ => { warn!("meh."); }
                }
            },
            _ => {}
        }
    }

    fn update_sleep_time(&mut self) {
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
        self.world.render()?;
        match self.window.swap_buffers() {
            Ok(_) => Ok(()),
            Err(e) => Err(ApplicationError::from(graphics::GraphicsError::from(e)))
        }
    }
}

fn get_keycode(input: glutin::KeyboardInput) -> Option<(glutin::VirtualKeyCode, bool)> {
    match (input.virtual_keycode, input.state) {
        (Some(keycode), glutin::ElementState::Pressed) => Some((keycode, true)),
        (Some(keycode), glutin::ElementState::Released) => Some((keycode, false)),
        (_, _) => None
    }
}
