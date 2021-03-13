use std::time::Instant;

use gl;
use gl::types::GLsizei;
use glutin;
use glutin::dpi::{LogicalSize, PhysicalPosition};
use glutin::event::{DeviceEvent, ElementState, Event, KeyboardInput, WindowEvent};
use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::window::Window;
use glutin::{ContextWrapper, PossiblyCurrent};

use crate::Config;
use crate::{graphics::check_opengl_error, window, CoreError, Input, State};

pub struct Core {
    context: ContextWrapper<PossiblyCurrent, Window>,
    event_loop: EventLoop<()>,
}

impl Core {
    pub fn new(config: &Config) -> Result<Core, CoreError> {
        let window_size = get_window_size(config);
        if window_size[0] <= 0. || window_size[1] <= 0. {
            return Err(CoreError::InvalidWindowSize(window_size[0], window_size[1]));
        }

        let event_loop = EventLoop::new();
        let context_wrapper = window::init_opengl(window_size, &event_loop)?;

        let core = Self {
            context: context_wrapper,
            event_loop: event_loop,
        };
        Ok(core)
    }

    pub fn run(self, mut state: Box<dyn State>) -> ! {
        let context = self.context;
        let mut hibernate = false;
        let mut state_input = Input::default();
        let mut last_update = Instant::now();

        self.event_loop
            .run(move |evt, _tgt, control_flow| match evt {
                Event::MainEventsCleared => {
                    state_input.set_time_passed(
                        last_update.elapsed().as_secs() as u32 * 1000
                            + last_update.elapsed().subsec_millis(),
                    );
                    last_update = Instant::now();
                    if let Err(e) = state.update(&state_input) {
                        error!("Update state: {}", e);
                        *control_flow = ControlFlow::Exit;
                        return;
                    }

                    if state_input.has_mouse_delta() {
                        center_mouse(context.window());
                        state_input.clear_mouse_delta();
                    }

                    unsafe { gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT) }
                    if let Err(e) = state.render() {
                        error!("Render state: {}", e);
                        *control_flow = ControlFlow::Exit;
                        return;
                    }
                    if let Err(e) = context.swap_buffers() {
                        error!("Swap Buffers: {}", e);
                        *control_flow = ControlFlow::Exit;
                    }
                }

                Event::DeviceEvent {
                    event: DeviceEvent::MouseMotion { delta, .. },
                    ..
                } => {
                    if !hibernate {
                        state_input.set_mouse_delta(delta);
                    }
                }
                Event::WindowEvent {
                    event: WindowEvent::KeyboardInput { input, .. },
                    ..
                } => {
                    handle_keyboard_event(&input, &mut state_input);
                }

                Event::WindowEvent { event, .. } => {
                    match handle_window_event(&event, context.window()) {
                        0 if hibernate => {
                            info!("Leave hibernation");
                            hibernate = false;
                        }
                        1 => {
                            info!("Enter hibernation");
                            hibernate = true;
                        }
                        2 => {
                            *control_flow = ControlFlow::Exit;
                        }
                        _ => {}
                    }
                }
                _ => {}
            })
    }
}

fn center_mouse(window: &Window) {
    let size = window.inner_size();
    if let Err(msg) =
        window.set_cursor_position(PhysicalPosition::new(size.width / 2, size.height / 2))
    {
        warn!("window.set_cursor position: {}", msg);
    }
}

fn handle_keyboard_event(event: &KeyboardInput, input: &mut Input) {
    let pressed = match event.state {
        ElementState::Pressed => true,
        ElementState::Released => false,
    };
    match event.scancode {
        0x11 => {
            input.set_key_pressed("W", pressed);
        }
        0x1E => {
            input.set_key_pressed("A", pressed);
        }
        0x1F => {
            input.set_key_pressed("S", pressed);
        }
        0x20 => {
            input.set_key_pressed("D", pressed);
        }
        0x39 => {
            input.set_key_pressed("SPACE", pressed);
        }
        0x3B => {
            input.set_key_pressed("F1", pressed);
        }
        0x3C => {
            input.set_key_pressed("F2", pressed);
        }
        0x3D => {
            input.set_key_pressed("F3", pressed);
        }
        _ => {}
    }
}

fn handle_window_event(window_event: &WindowEvent, window: &Window) -> i32 {
    match window_event {
        WindowEvent::CloseRequested => 2,
        WindowEvent::Focused(true) => 0,
        WindowEvent::Focused(false) => 1,
        WindowEvent::ScaleFactorChanged {
            scale_factor: _,
            new_inner_size: _,
        } => 0,
        WindowEvent::Resized(physical_size) => {
            let new_size: LogicalSize<GLsizei> = physical_size.to_logical(window.scale_factor());
            unsafe {
                gl::Viewport(0, 0, new_size.width, new_size.height);
            }
            info!(
                "Updated viewport to {}/{}/{}/{}",
                0, 0, new_size.width, new_size.height
            );
            match check_opengl_error("gl::Viewport") {
                Ok(_) => {}
                Err(e) => {
                    warn!("{}", e);
                }
            }
            0
        }
        _ => 0,
    }
}

fn get_window_size(config: &Config) -> [f64; 2] {
    [
        config.get_int_or_default("window_x", 1024) as f64,
        config.get_int_or_default("window_y", 768) as f64,
    ]
}
