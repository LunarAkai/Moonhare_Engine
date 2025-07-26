use glium::implement_vertex;

#[derive(Copy, Clone)]
pub(crate) struct Vertex {
    pub(crate) position: [f32; 2],
    pub(crate) color: [f32; 3],
}
implement_vertex!(Vertex, position, color);