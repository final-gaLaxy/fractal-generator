extern crate glium;
extern crate winit;

fn main() {
    let event_loop = winit::event_loop::EventLoopBuilder::new().build().expect("event loop building");

    let (_window, display) = glium::backend::glutin::SimpleWindowBuilder::new().build(&event_loop);

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
