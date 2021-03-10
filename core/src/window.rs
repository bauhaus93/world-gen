use gl;
use glutin;
use glutin::dpi::{LogicalSize, Size};
use glutin::event_loop::EventLoop;
use glutin::window::{Window, WindowBuilder};
use glutin::{ContextBuilder, ContextWrapper, PossiblyCurrent, WindowedContext};

use crate::graphics;

pub fn init_opengl(
    window_size: [f64; 2],
    event_loop: &EventLoop<()>,
) -> Result<ContextWrapper<PossiblyCurrent, Window>, graphics::GraphicsError> {
    info!(
        "Creating window, size {}x{}",
        window_size[0], window_size[1]
    );
    let window_builder =
        WindowBuilder::new().with_inner_size(LogicalSize::new(window_size[0], window_size[1]));
    let windowed_context = ContextBuilder::new()
        .with_vsync(true)
        .with_depth_buffer(8)
        .build_windowed(window_builder, &event_loop)?;
    let windowed_context = unsafe { windowed_context.make_current().map_err((|e| e.1))? };
    windowed_context.window().set_cursor_visible(false);
    info!("Loading opengl functions");
    gl::load_with(|s| windowed_context.get_proc_address(s) as *const _);
    graphics::check_opengl_error("gl::load_with")?;

    windowed_context.window().set_visible(true);
    unsafe {
        gl::Enable(gl::DEPTH_TEST);
        gl::DepthFunc(gl::LESS);
        gl::Enable(gl::CULL_FACE); //Default is front = CCW, cull back
        gl::ClearDepth(1.);
    }
    graphics::check_opengl_error("gl setup")?;

    match graphics::get_opengl_version() {
        Ok(version) => {
            info!("opengl version: {}", version)
        }
        Err(e) => {
            warn!("Could not convert opengl version string: {}", e);
        }
    }
    info!("Successfully created opengl context");
    Ok(windowed_context)
}
