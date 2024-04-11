#[macro_use]
extern crate glium;

use glium::{implement_vertex, Surface};
use winit::event::WindowEvent::{CloseRequested, Resized, RedrawRequested};
use winit::event::Event::{WindowEvent, AboutToWait};

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
    let (window, display) = glium::backend::glutin::SimpleWindowBuilder::new()
        .with_title("Fractal Generator")
        .with_inner_size(800, 800)
        .build(&event_loop);

    // Compile shaders
    let program = glium::Program::from_source(
        &display,
        include_str!("vertex.glsl"),
        include_str!("fragment.glsl"),
        None).unwrap();

    // Render Square
    let vertices = vec![
        Vertex { a_position: [-1.0, -1.0] },
        Vertex { a_position: [ 1.0, -1.0] },
        Vertex { a_position: [ 1.0,  1.0] },
        Vertex { a_position: [-1.0,  1.0] }
    ];

    let indices: [u32; 6] =  [
        0, 1, 2,
        0, 2, 3
    ];

    let vertex_buffer = glium::VertexBuffer::new(
        &display,
        &vertices).unwrap();
    let indices = glium::IndexBuffer::new(
        &display,
        glium::index::PrimitiveType::TrianglesList,
        &indices).unwrap();

    // Handle window events
    let mut frames = 0;
    let mut start = std::time::Instant::now();
    let _ = event_loop.run(move |event, window_target| {
        let elapsed = start.elapsed();
        frames += 1;
        if elapsed.as_secs() >= 1 {
            println!("{:.0}", frames as f64 / elapsed.as_millis() as f64 * 1000.0);
            start = std::time::Instant::now();
            frames = 0;
        }

        match event {
            WindowEvent { event, .. } => match event {
                CloseRequested => window_target.exit(),
                Resized(window_size) => {
                    display.resize(window_size.into())
                },
                RedrawRequested => {
                    let mut target = display.draw();
                    target.clear_color_and_depth((1.0, 1.0, 1.0, 1.0), 1.0);

                    let screen_size = display.get_framebuffer_dimensions();

                    let uniforms = uniform! {
                        u_screenSize: [screen_size.0 as f32, screen_size.1 as f32]
                    };

                    target.draw(&vertex_buffer, &indices, &program, &uniforms,
                        &Default::default()).unwrap();

                    target.finish().unwrap();
                },
                _ => ()
            },
            AboutToWait => {
                window.request_redraw();
            },
            _ => (),
        };
    });
}
