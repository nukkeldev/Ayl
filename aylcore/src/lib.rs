use aylcommon::backend::Backend;
use aylgl::GlfwBackend;

pub struct Application {
    title: Box<String>,
    size: (usize, usize),
    backend: Box<dyn Backend>,
}

#[derive(Default)]
pub enum Backends {
    #[default]
    GLOW,
}

impl Application {
    pub fn init(title: &str, size: (usize, usize)) -> Self {
        let title = Box::new(title.to_owned());
        Self {
            backend: Box::new(GlfwBackend::new(title.clone(), size)),
            title,
            size,
        }
    }

    pub fn start(&mut self) {
        self.backend.start()
    }
}
