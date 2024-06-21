use std::ffi::c_void;

use glfw::{GlfwReceiver, WindowEvent};

use crate::graphics_raw::glfwInit;

extern "system" fn glDebugCallback(
    source: u32,
    type_t: u32,
    id: u32,
    severity: u32,
    length: i32,
    message: *const i8,
    user_param: *mut c_void,
) {
    if severity == gl::DEBUG_SEVERITY_HIGH {
        // Handle high-severity messages (e.g., errors)
        println!("OpenGL Error (High Severity): {:?}", message);
    } else if severity == gl::DEBUG_SEVERITY_MEDIUM {
        // Handle medium-severity messages (e.g., warnings)
        println!("OpenGL Error (MED Severity): {:?}", message);
    } else {
        // Handle low-severity messages (e.g., performance notifications)
        println!("OpenGL Error (LOW Severity): {:?}", message);
    }
}

pub struct GLWindow {
    pub m_glfw: glfw::Glfw,
    pub m_window: glfw::PWindow,
    pub m_receiver: GlfwReceiver<(f64, WindowEvent)>,
}

impl GLWindow {
    pub fn new_configured(width: u32, height: u32) -> Self {
        let mut window = GLWindow::create_window(width, height);
        window
            .set_debug_callback()
            .set_polling(true, true, true)
            .set_view_port(0, 0, width as _, height as _);
        unsafe {
            gl::Enable(gl::DEPTH_TEST);
            // gl::Enable(gl::BLEND);
            // gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
            gl::Disable(gl::CULL_FACE);
        }
        window
    }
    pub fn create_window(width: u32, height: u32) -> Self {
        let tmp = glfwInit(width, height);
        Self {
            m_glfw: tmp.0,
            m_window: tmp.1,
            m_receiver: tmp.2,
        }
    }
    pub fn set_polling(
        &mut self,
        key_poll: bool,
        cursor_poll: bool,
        mouse_poll: bool,
    ) -> &mut Self {
        self.m_window.set_key_polling(key_poll);
        self.m_window.set_cursor_pos_polling(cursor_poll);
        self.m_window.set_mouse_button_polling(mouse_poll);
        unsafe {
            gl::Disable(gl::DEPTH_TEST);
        }
        self
    }
    pub fn set_view_port(&mut self, x: i32, y: i32, width: i32, height: i32) -> &mut Self {
        unsafe {
            gl::Viewport(x, y, width, height);
        }
        self
    }
    pub fn set_debug_callback(&mut self) -> &mut Self {
        unsafe {
            gl::Enable(gl::DEBUG_OUTPUT);
            gl::DebugMessageCallback(Some(glDebugCallback), 0 as _);
        }
        self
    }
}

impl Default for GLWindow {
    fn default() -> Self {
        Self::new_configured(1000, 1000)
    }
}
