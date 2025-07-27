use glium::implement_vertex;

#[derive(Copy, Clone)]
pub struct Vertex {
    pub position: [f32; 2],
    pub color: [f32; 3],
}
implement_vertex!(Vertex, position, color);

impl Vertex {
    pub fn define_shape(v1: Vertex, v2: Vertex, v3: Vertex) -> Vec<Vertex> {
        let shape = vec![
            v1,
            v2,
            v3
        ];
        return shape;
    }
}