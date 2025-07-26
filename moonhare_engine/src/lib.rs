mod vertex;

use glium::{uniform, uniforms, Surface};

use vertex::Vertex;

// rescaling:   position *= factor;
// rotating:    new_position = vec2(pos.x * cos(angle) - pos.y * sin(angle), pos.x * sin(single) + pos.y * cos(angle));
// skewing:     position.x += position.y * factor;


fn main() {
    let event_loop = glium::winit::event_loop::EventLoop::builder()
        .build()
        .expect("event loop building");
    
    let (_window, display) = glium::backend::glutin::SimpleWindowBuilder::new()
        .with_title("Moonhare Engine")
        .build(&event_loop);

    let shape = vec![
        Vertex { position: [-0.5, -0.5], color: [1.0, 0.0, 0.0] },
        Vertex { position: [ 0.0,  0.5], color: [0.0, 1.0, 0.0] },
        Vertex { position: [ 0.5, -0.25], color: [0.0, 0.0, 1.0] }
    ];

    // "Upload" shape to the memory of the GPU (Vertex Buffer)
    // Isn't strictly necessary but, makes tge drawing operation faster
    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    
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
    let vertex_shader_src = r#"
        #version 140
        
        in vec2 position;
        in vec3 color;
        out vec3 vertex_color;

        uniform mat4 matrix;

        void main() {
            vertex_color = color; 
            gl_Position = matrix * vec4(position, 0.0, 1.0);
        }
    "#;

    let fragment_shader_src = r#"
        #version 140

        in vec3 vertex_color;
        out vec4 color;

        void main() {
            color = vec4(vertex_color, 1.0);
        }
    "#;

    // send shader source code to glium
    let program = glium::Program::from_source(
        &display, 
        vertex_shader_src, 
        fragment_shader_src, 
        None
    ).unwrap();



    let mut t: f32 = 0.0;
    let _ = event_loop.run(move |event, window_target| {
        match event {
            glium::winit::event::Event::WindowEvent { event, .. } => match event {
                glium::winit::event::WindowEvent::CloseRequested => window_target.exit(),
                glium::winit::event::WindowEvent::Resized(window_size) => {
                    display.resize(window_size.into());
                }
                glium::winit::event::WindowEvent::RedrawRequested => {
                    t += 0.02;
                    // use 't' as an offset -> smooth animation
                    let x_offset = t.sin() * 0.5;

                    let mut target = display.draw();

                    target.clear_color(
                        0.0, 
                        0.0, 
                        1.0, 
                        1.0);
                    
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
                        &vertex_buffer, 
                        &indices, 
                        &program, 
                        &uniforms,
                        &Default::default()
                    ).unwrap();

                    target.finish().unwrap();
                },
                _ => (),
            },
            // request a redraw ourselves once we've finished rendering
            glium::winit::event::Event::AboutToWait => {
                _window.request_redraw();
            },
            _ => (),
        };
    });
}
