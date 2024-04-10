#[macro_use]
extern crate glium;

use glium::{implement_vertex, Surface};

fn main() {
    let event_loop = winit::event_loop::EventLoopBuilder::new().build().expect("event loop building");
    let (_window, display) = glium::backend::glutin::SimpleWindowBuilder::new().build(&event_loop);

    // Compile shaders
    let program = glium::Program::from_source(&display, include_str!("vertex.glsl"), include_str!("fragment.glsl"), None).unwrap();

    #[derive(Copy, Clone)]
    struct Vertex {
        a_position: [f32; 2],
    }
    implement_vertex!(Vertex, a_position);

    let shape = vec![
        Vertex { a_position: [-0.5, -0.5] },
        Vertex { a_position: [ 0.0,  0.5] },
        Vertex { a_position: [ 0.5, -0.5] }
    ];

    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let mut target = display.draw();
    target.clear_color(0.0, 0.0, 1.0, 1.0);
    target.draw(&vertex_buffer, &indices, &program, &glium::uniforms::EmptyUniforms,
        &Default::default()).unwrap();
    target.finish().unwrap();

    let _ = event_loop.run(move |event, window_target| {
        match event {
            winit::event::Event::WindowEvent { event, .. } => match event {
                winit::event::WindowEvent::CloseRequested => window_target.exit(),
                _ => (),
            },
            _ => (),
        };
    });
}
