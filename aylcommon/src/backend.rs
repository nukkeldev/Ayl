pub trait Backend {
    fn init(&mut self);
    fn start(&mut self);
    fn get_title(&self) -> Box<String>;
    fn get_size(&self) -> (usize, usize);
    fn should_close(&self) -> bool;
}
