#[macro_use]
extern crate glium;

use glium::{implement_vertex, Surface};
use winit::event::WindowEvent::{CloseRequested, Resized};
use winit::event::Event::WindowEvent;

#[derive(Copy, Clone)]
struct Vertex {
    a_position: [f32; 2],
}
implement_vertex!(Vertex, a_position);

fn main() {
    // Create event loop
    let event_loop = winit::event_loop::EventLoopBuilder::new()
    .build()
    .expect("event loop building");

    // Create window
    let (_window, display) = glium::backend::glutin::SimpleWindowBuilder::new()
        .with_title("Fractal Generator")
        .with_inner_size(800, 800)
        .build(&event_loop);

    // Compile shaders
    let program = glium::Program::from_source(
        &display,
        include_str!("vertex.glsl"),
        include_str!("fragment.glsl"),
        None).unwrap();

    // Render triangle
    let shape = vec![
        Vertex { a_position: [-0.5, -0.5] },
        Vertex { a_position: [ 0.0,  0.5] },
        Vertex { a_position: [ 0.5, -0.5] }
    ];

    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let mut target = display.draw();
    target.clear_color(0.0, 0.0, 1.0, 1.0);

    let uniforms = uniform! {
        u_screenSize: display.get_framebuffer_dimensions()
    };

    target.draw(&vertex_buffer, &indices, &program, &uniforms,
        &Default::default()).unwrap();

    target.finish().unwrap();

    // Handle window events
    let _ = event_loop.run(move |event, window_target| {
        match event {
            WindowEvent { event, .. } => match event {
                CloseRequested => window_target.exit(),
                Resized(window_size) => {
                    display.resize(window_size.into())
                },
                _ => (),
            },
            _ => (),
        };
    });
}
