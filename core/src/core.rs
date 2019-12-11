use std::{ thread, time, ops::{ Add, Sub } };
use std::collections::BTreeMap;

use glutin;
use gl;
use gl::types::GLsizei;

use crate::{Rotatable, CoreError, window, graphics::{GraphicsError, check_opengl_error }};
use crate::{ Config, Float };
use crate::traits::Renderable;

pub struct Core {
    window: glutin::GlWindow,
    events_loop: glutin::EventsLoop,
    window_size: [f64; 2],
    quit: bool,
    hibernate: bool,
    time_passed: u32,
	last_update: time::Instant,
    sleep_time: time::Duration,
	key_pressed: BTreeMap<glutin::VirtualKeyCode, bool>,
	last_mouse_delta: Option<(f64, f64)>,
    title_update_passed: u32,
    target_frequency: u32
}

impl Core {
    pub fn new(config: &Config) -> Result<Core, CoreError> {

        let window_size = get_window_size(config);
        if window_size[0] <= 0. || window_size[1] <= 0. {
            return Err(CoreError::InvalidWindowSize(window_size[0], window_size[1]));
        }

        let events_loop = glutin::EventsLoop::new();
        let window = window::init_window(window_size, &events_loop)?;

        window.set_title("world_gen");
        
        let app = Self {
            events_loop: events_loop,
            window: window,
            window_size: window_size,
            quit: false,
            hibernate: false,
            time_passed: 0,
			last_update: time::Instant::now(),
            sleep_time: time::Duration::from_millis(50),
			key_pressed: BTreeMap::new(),
			last_mouse_delta: None,
            title_update_passed: 0,
            target_frequency: 30
        };
        Ok(app)
    }

	pub fn should_quit(&self) -> bool {
		self.quit
	}

	pub fn is_hibernating(&self) -> bool {
		self.hibernate
	}

	pub fn key_pressed(&self, keycode: glutin::VirtualKeyCode) -> bool {
		match self.key_pressed.get(&keycode) {
			Some(pressed) => *pressed,
			None => false
		}
	}

	pub fn has_mouse_delta(&self) -> bool {
		self.last_mouse_delta.is_some()
	}

	pub fn get_mouse_delta(&self) -> (f64, f64) {
		match self.last_mouse_delta {
			Some((x, y)) => (x, y),
			None => (0., 0.)
		}
	}

	pub fn get_time_passed(&self) -> u32 {
		self.time_passed
	}

	pub fn center_mouse(&mut self) {
        if let Err(msg) = self.window.set_cursor_position(glutin::dpi::LogicalPosition::new(self.window_size[0] / 2., self.window_size[1] / 2.)) {
            warn!("window.set_cursor position: {}", msg);
        }
	}

	pub fn update(&mut self) -> Result<(), CoreError> {
		self.handle_events();

		if !self.hibernate {
			if self.title_update_passed > 1000 {
				self.update_title();
			} else {
				self.title_update_passed += self.time_passed;
			}

			self.update_sleep_time();
		}

        self.time_passed = self.last_update.elapsed().as_secs() as u32 * 1000 + self.last_update.elapsed().subsec_millis();
        self.last_update = time::Instant::now();
		thread::sleep(self.sleep_time);
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
		self.last_mouse_delta = None;
        for event in events {
            match event {
                glutin::Event::WindowEvent { event, .. } => {
                    match event {
                        glutin::WindowEvent::CloseRequested => { self.quit = true; },
                        glutin::WindowEvent::Focused(focused) => { self.hibernate = !focused; },
                        glutin::WindowEvent::Resized(logical_size) => { self.handle_resize((*logical_size).into()); },
                        glutin::WindowEvent::KeyboardInput { input, .. } => {
                            if let Some((key, down)) = get_keycode(*input) {
								self.key_pressed.insert(key, down);
                            }
                        },
                        glutin::WindowEvent::MouseWheel { delta, phase, .. } if !self.hibernate => { self.handle_mousewheel(*delta, *phase); }
                        _ => {}
                    }
                },
                glutin::Event::DeviceEvent { event, .. } => {
                    match event {
                        glutin::DeviceEvent::MouseMotion { delta } if !self.hibernate => {
							self.last_mouse_delta = Some(*delta);
							},
                        _ => {}
                    }
                },
                _ => {}
            }
        }
    }

    fn handle_resize(&mut self, new_size: (u32, u32)) {
        unsafe { gl::Viewport(0, 0, new_size.0 as GLsizei, new_size.1 as GLsizei); }
        self.window_size = [new_size.0 as f64, new_size.1 as f64];
        info!("Updated viewport to {}/{}/{}/{}", 0, 0, new_size.0, new_size.1);
        match check_opengl_error("gl::Viewport") {
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
        let diff: i32 = (self.time_passed * self.target_frequency) as i32 - 1000;
        if diff.abs() as u32 > self.target_frequency {
            let adj = time::Duration::from_millis(u64::min(u64::max(diff.abs() as u64 / 100, 1), 5 as u64));
            match diff.signum() {
                1 => {
                    if self.sleep_time >= adj {
                        self.sleep_time = self.sleep_time.sub(adj)
                    }
                },
                -1 => self.sleep_time = self.sleep_time.add(adj),
                _ => {}
            }
        } 
    }

    fn update_title(&mut self) {
        let idle: i32 = i32::min(100, (100. * (1. - (self.time_passed as i32 - self.sleep_time.as_millis() as i32) as f64 / self.time_passed as f64)) as i32);
        let info = format!("main thread idle = {}%, frequency = {}Hz", idle, 1000 / self.time_passed);
        self.window.set_title(&info);
        self.title_update_passed = 0;
    }

    fn render(&mut self, scene: &mut dyn Renderable) -> Result<(), CoreError> {
        unsafe { gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT) }        
		//scene.render(self.camera)?;
        match self.window.swap_buffers() {
            Ok(_) => Ok(()),
            Err(e) => Err(CoreError::from(GraphicsError::from(e)))
        }
    }
}

fn get_window_size(config: &Config) -> [f64; 2] {
    [config.get_int_or_default("window_x", 1024) as f64,
     config.get_int_or_default("window_y", 768) as f64]
}

fn get_keycode(input: glutin::KeyboardInput) -> Option<(glutin::VirtualKeyCode, bool)> {
    match (input.virtual_keycode, input.state) {
        (Some(keycode), glutin::ElementState::Pressed) => Some((keycode, true)),
        (Some(keycode), glutin::ElementState::Released) => Some((keycode, false)),
        (_, _) => None
    }
}
