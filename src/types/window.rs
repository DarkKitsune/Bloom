use crate::*;
use fennec_algebra::*;
use glfw::Context;
use glfw::GLProc;
use std::sync::mpsc::Receiver;

pub struct Window {
    glfw_window: Option<glfw::Window>,
    event_receiver: Receiver<(f64, glfw::WindowEvent)>,
    closed: bool,
    size: Vec2u,
}

impl Window {
    pub fn new(glfw: &mut glfw::Glfw, size: Vec2u, title: impl AsRef<str>) -> Self {
        // Set hints for window
        glfw.window_hint(glfw::WindowHint::ContextVersion(4, 5));
        glfw.window_hint(glfw::WindowHint::OpenGlDebugContext(true));
        glfw.window_hint(glfw::WindowHint::OpenGlProfile(
            glfw::OpenGlProfileHint::Core,
        ));
        glfw.window_hint(glfw::WindowHint::SRgbCapable(true));

        // Create GLFW window and event receiver for the window
        let (mut glfw_window, event_receiver) = glfw
            .create_window(
                *size.x(),
                *size.y(),
                title.as_ref(),
                glfw::WindowMode::Windowed,
            )
            .expect("Could not create window.");

        // Set initial settings
        glfw_window.set_key_polling(true);
        glfw_window.set_close_polling(true);

        Self {
            glfw_window: Some(glfw_window),
            event_receiver,
            closed: false,
            size,
        }
    }

    pub fn process_events(&mut self) {
        if self.is_closed() {
            panic!("Window is not open");
        }
        for (_, event) in glfw::flush_messages(&self.event_receiver) {
            //println!("Event: {:?}", event);
            match event {
                glfw::WindowEvent::Close => self.closed = true,
                _ => (),
            }
        }
        if self.closed {
            let mut window = None;
            std::mem::swap(&mut window, &mut self.glfw_window);
            window.expect("Window was None").close();
            println!("Closed GLFW window!");
        }
    }

    pub fn is_closed(&self) -> bool {
        self.closed || self.glfw_window.is_none()
    }

    pub fn proc_address(&mut self, proc_name: impl AsRef<str>) -> GLProc {
        self.glfw_window
            .as_mut()
            .expect("Window was none")
            .get_proc_address(proc_name.as_ref())
    }

    pub fn swap_buffers(&mut self) {
        self.glfw_window
            .as_mut()
            .expect("Window was None")
            .swap_buffers();
    }

    pub fn poll_events(glfw: &mut glfw::Glfw) {
        glfw.poll_events();
    }

    pub fn size(&self) -> Vec2u {
        self.size
    }

    pub fn aspect_ratio(&self) -> f32 {
        *self.size.x() as f32 / *self.size.y() as f32
    }
}
