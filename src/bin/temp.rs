use std::error::Error;
use std::num::NonZeroU32;

use glow::HasContext;
use raw_window_handle::HasRawWindowHandle;
use winit::event::Event;
use winit::event::WindowEvent;
use winit::event_loop::EventLoopBuilder;
use winit::window::WindowBuilder;

use glutin::{
    config::{Config, ConfigTemplateBuilder, GlConfig},
    context::{ContextAttributesBuilder, NotCurrentGlContext},
    display::{GetGlDisplay, GlDisplay},
    surface::{GlSurface, SwapInterval}
};

use glutin_winit::{DisplayBuilder, GlWindow};

fn main()-> Result<(), Box<dyn Error>> {
    unsafe {
        // Create event loop
        let event_loop = EventLoopBuilder::new()
        .build()
        .expect("event loop building");

        // Windows requires the window before display creation
        let window_builder = WindowBuilder::new()
            .with_transparent(true)
            .with_title("Fractal Generator");

        let template = ConfigTemplateBuilder::new().with_alpha_size(8);

        let display_builder = DisplayBuilder::new().with_window_builder(Some(window_builder));

        let (window, gl_config) = display_builder
            .build(
                &event_loop,
                template,
                gl_config_picker
            )
            .unwrap();

        let raw_window_handle = window.as_ref().map(|window| window.raw_window_handle());

        let gl_display = gl_config.display();

        let context_attributes = ContextAttributesBuilder::new()
            .build(raw_window_handle);

        let not_current_gl_context = gl_display
            .create_context(&gl_config, &context_attributes)
            .expect("failed to create context");

        let window = window.unwrap();

        let attrs = window.build_surface_attributes(Default::default());
        let gl_surface = gl_display
            .create_window_surface(&gl_config, &attrs)
            .unwrap();

        let gl_context = not_current_gl_context.make_current(&gl_surface).unwrap();

        let gl = glow::Context::from_loader_function_cstr(move|s| {
            gl_display.get_proc_address(s) as *const _
        });

        let _ = gl_surface
            .set_swap_interval(&gl_context, SwapInterval::Wait(NonZeroU32::new(1).unwrap()))
            .unwrap();

        let vertex_array = gl
            .create_vertex_array()
            .expect("Cannot create vertex array");
        gl.bind_vertex_array(Some(vertex_array));

        let program = gl.create_program().expect("Cannot create program");

        let shader_sources = [
            (glow::VERTEX_SHADER, include_str!("simple.vert")),
            (glow::FRAGMENT_SHADER, include_str!("mandelbrot.frag"))
        ];

        let mut shaders = Vec::with_capacity(shader_sources.len());

        for (shader_type, shader_source) in shader_sources.iter() {
            let shader = gl
                .create_shader(*shader_type)
                .expect("Cannot create shader");
            gl.shader_source(shader, &format!("{}", shader_source));
            gl.compile_shader(shader);
            if !gl.get_shader_compile_status(shader) {
                panic!("{}", gl.get_shader_info_log(shader));
            }
            gl.attach_shader(program, shader);
            shaders.push(shader);
        }

        gl.link_program(program);
        if !gl.get_program_link_status(program) {
            panic!("{}", gl.get_program_info_log(program));
        }

        for shader in shaders {
            gl.detach_shader(program, shader);
            gl.delete_shader(shader);
        }

        gl.use_program(Some(program));
        gl.clear_color(0.1, 0.2, 0.3, 1.0);

        let _ = event_loop.run(move |event, elwt| {
            if let Event::WindowEvent { event, .. } = event {
                match event {
                    WindowEvent::CloseRequested => {
                        elwt.exit();
                    },
                    WindowEvent::RedrawRequested => {
                        gl.clear(glow::COLOR_BUFFER_BIT);
                        gl.draw_arrays(glow::TRIANGLES, 0, 3);
                        gl_surface.swap_buffers(&gl_context).unwrap()
                    },
                    _ => (),
                }
            }
        });

        Ok(())
    }
}

pub fn gl_config_picker(configs: Box<dyn Iterator<Item = Config> +'_>) -> Config {
    configs.reduce(|accum, config| {
        let transparency_check = config.supports_transparency().unwrap_or(false)
            & !accum.supports_transparency().unwrap_or(false);

        if transparency_check || config.num_samples() > accum.num_samples() {
            config
        } else {
            accum
        }
    })
    .unwrap()
}