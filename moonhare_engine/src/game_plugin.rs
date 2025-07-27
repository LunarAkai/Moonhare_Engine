use glium::Frame;


pub trait GamePlugin {
    fn init(&mut self);
    fn update(&mut self);
    fn render(&mut self, target: &mut Frame);
    fn cleanup(&mut self);
}