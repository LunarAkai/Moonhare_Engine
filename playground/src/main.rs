use std::fs::read_to_string;

use glium::{index::NoIndices, Frame, Program, VertexBuffer};
use glium::{uniform, Surface};
use moonhare_engine::{core::game::Game, game_plugin::GamePlugin, vertex::Vertex};

struct PlaygroundGame {
    t: f32,
    shape: Vec<Vertex>, 
    vertex_buffer: Option<VertexBuffer<Vertex>>,
    indices: Option<NoIndices>,
    program: Option<Program>,
}

impl GamePlugin for PlaygroundGame {
    fn init(&mut self) {
        self.t = 0.0;
    }
    fn update(&mut self) { 
        self.t += 0.02;
        // use 't' as an offset -> smooth animation

    }
    fn render(&mut self, target: &mut Frame) {
        
        target.clear_color(
            0.0, 
            0.0, 
            1.0, 
            1.0
        );
        let x_offset = self.t.sin() * 0.5;
                    
        // Note: In OpenGL matrices are column-major
        // Standard mathematical notation is row major:
        //      1.0     0.0     0.0     x_offset
        //      0.0     1.0     0.0     0.0
        //      0.0     0.0     1.0     0.0
        //      0.0     0.0     0.0     1.0
        let uniforms = uniform! {
            matrix: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [x_offset, 0.0, 0.0, 1.0f32],
            ]
        };    

        target.draw(
            self.vertex_buffer.as_ref().unwrap(), 
            &self.indices.unwrap(), 
            self.program.as_ref().unwrap(), 
            &uniforms,
            &Default::default()
        ).unwrap();
    }
    fn cleanup(&mut self) {
        
    }
    fn handle_events(&mut self) {
        
    }
}



fn main() {

    let mut game = Game::new();
    game.register_plugin(Box::new(PlaygroundGame{ t: 0.0, shape: Default::default(), vertex_buffer: None, indices: None, program: None }));
    game.init();

    let binding = Some(game.display.clone()).unwrap();
    // todo: unwraps on none
    let display = binding;

    let shape = Vertex::define_shape(
    Vertex { position: [-0.5, -0.5], color: [1.0, 0.0, 0.0] },
    Vertex { position: [ 0.0,  0.5], color: [0.0, 1.0, 0.0] },
    Vertex { position: [ 0.5, -0.5], color: [0.0, 0.0, 1.0] }
    );


    // "Upload" shape to the memory of the GPU (Vertex Buffer)
    // Isn't strictly necessary but, makes tge drawing operation faster
    
    let vertex_buffer: VertexBuffer<Vertex> = VertexBuffer::new(&display, &shape).unwrap();

        
    // Complex shapes consist of hundreds/thousands of vertices -> need to have a list of vertices and tell OpenGL how to link these
    // vertices together to obtain triangles.
    // For only one triangle -> pass dummy marker to glium
    // This line tells OpenGl that we don't use indices and instand want to draw a certain number of seperate triangles
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    // (Simplified) Render Pipeline:
    // Vertex Shader -> Fragment (Pixel) Shader 
    // uniform:
    // value is set when we draw by passing its value to the draw function
    // (easiest way is uniform! macro)
    // Important to write matrix * vertex -> Matrix operations produce different results depending on the order
    // out: defines a variable that is going to be passed along to the fragment shader

    let vertex_shader_src = read_to_string("playground/src/shaders/vertex_shader.glsl").unwrap_or("vertex shader failed to unwrap".to_owned());
    let fragment_shader_src = read_to_string("playground/src/shaders/fragment_shader.glsl").unwrap_or("fragment shader failed to unwrap".to_owned());

    // send shader source code to glium
    let program = match glium::Program::from_source(
        &display, 
        &vertex_shader_src, 
        &fragment_shader_src, 
        None
    )  {
        Ok(program) => program,
        Err(err) => {
            eprintln!("Shader compilation error: {:?}", err);
            panic!("Failed to compile shaders")
        }
    };               


    let pg_game = PlaygroundGame { 
        t: 0.0,
        shape: shape,
        vertex_buffer: Some(vertex_buffer),
        indices: Some(indices),
        program: Some(program),
    };

    //let mut a = game.borrow_mut();
    game.register_plugin(Box::new(pg_game));

    game.run();
}