use std::sync::mpsc::Receiver;

use aylcommon::backend::Backend;
use glfw::{Action, Context, Glfw, Key, Window, WindowEvent};

pub struct GlfwBackend {
    title: Box<String>,
    size: (usize, usize),

    glfw: Option<Glfw>,
    window: Option<Window>,
    events: Option<Receiver<(f64, WindowEvent)>>,
}

impl Backend for GlfwBackend {
    fn init(&mut self) {
        let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

        let (mut window, events) = glfw
            .create_window(
                self.size.0 as u32,
                self.size.1 as u32,
                self.title.as_str(),
                glfw::WindowMode::Windowed,
            )
            .expect("Failed to create GLFW window.");

        window.set_sticky_keys(false);
        window.set_all_polling(true);

        window.make_current();

        self.glfw = Some(glfw);
        self.window = Some(window);
        self.events = Some(events);
    }

    fn start(&mut self) {
        let glfw = self.glfw.as_mut().unwrap();
        let window = self.window.as_mut().unwrap();
        let events = self.events.as_mut().unwrap();

        while !window.should_close() {
            window.swap_buffers();

            glfw.poll_events();
            for (_time, event) in glfw::flush_messages(&events) {
                println!("{:?}", event);
                match event {
                    glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                        window.set_should_close(true);
                    }
                    _ => {}
                }
            }
        }
    }

    fn get_title(&self) -> Box<String> {
        self.title.clone()
    }

    fn get_size(&self) -> (usize, usize) {
        self.size
    }

    fn should_close(&self) -> bool {
        self.window.as_ref().unwrap().should_close()
    }
}

impl GlfwBackend {
    pub fn new(title: Box<String>, size: (usize, usize)) -> Self {
        let mut v = Self {
            title,
            size,
            glfw: None,
            window: None,
            events: None,
        };
        v.init();
        v
    }
}
