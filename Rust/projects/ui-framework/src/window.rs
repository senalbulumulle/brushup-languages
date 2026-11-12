use glfw::{Action, Context, Glfw, GlfwReceiver, Key, PWindow, WindowEvent};

pub struct AppWindow {
    pub glfw: Glfw,
    pub window: PWindow,
    pub events: GlfwReceiver<(f64, WindowEvent)>,
    pub width: u32,
    pub height: u32,
}

impl AppWindow {
    pub fn new(title: &str, width: u32, height: u32) -> Result<Self, String> {
        let mut glfw = glfw::init(glfw::fail_on_errors).map_err(|e| e.to_string())?;

        glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
        glfw.window_hint(glfw::WindowHint::OpenGlProfile(
            glfw::OpenGlProfileHint::Core,
        ));
        #[cfg(target_os = "macos")]
        glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));

        let (mut window, events) = glfw
            .create_window(width, height, title, glfw::WindowMode::Windowed)
            .ok_or_else(|| "failed to create GLFW window".to_string())?;

        window.make_current();
        window.set_key_polling(true);
        window.set_framebuffer_size_polling(true);
        window.set_cursor_pos_polling(true);
        window.set_mouse_button_polling(true);

        gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

        let mut app = Self {
            glfw,
            window,
            events,
            width,
            height,
        };
        app.set_viewport(width as i32, height as i32);
        Ok(app)
    }

    pub fn should_close(&self) -> bool {
        self.window.should_close()
    }

    pub fn request_close(&mut self) {
        self.window.set_should_close(true);
    }

    pub fn swap_buffers(&mut self) {
        self.window.swap_buffers();
    }

    pub fn poll_events(&mut self) {
        self.glfw.poll_events();
    }

    pub fn set_viewport(&mut self, width: i32, height: i32) {
        self.width = width.max(1) as u32;
        self.height = height.max(1) as u32;
        unsafe {
            gl::Viewport(0, 0, self.width as i32, self.height as i32);
        }
    }

    /// Drain the event queue. Returns `true` if Escape was pressed.
    pub fn drain_events(&mut self) -> bool {
        let pending: Vec<_> = glfw::flush_messages(&self.events).map(|(_, e)| e).collect();
        let mut escape = false;

        for event in pending {
            match event {
                WindowEvent::Key(Key::Escape, _, Action::Press, _) => escape = true,
                WindowEvent::FramebufferSize(w, h) => self.set_viewport(w, h),
                _ => {}
            }
        }

        escape
    }
}
