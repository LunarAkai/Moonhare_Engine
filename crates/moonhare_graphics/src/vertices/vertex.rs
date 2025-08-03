use glium::implement_vertex;

#[derive(Clone, Copy)]
pub struct Vertex {
    pub(crate) position: [f32; 2],
    pub(crate) color: [f32; 3],
}
implement_vertex!(Vertex, position, color);
