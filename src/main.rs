extern crate nalgebra as na;

use clap::{Arg, Command};
use glow::{HasContext, NativeBuffer, NativeProgram, NativeVertexArray};
use glutin::{
    config::{Config, ConfigTemplateBuilder, GlConfig},
    context::{ContextAttributesBuilder, NotCurrentGlContext},
    display::{GetGlDisplay, GlDisplay},
    surface::{GlSurface, SwapInterval, WindowSurface},
};
use glutin_winit::{DisplayBuilder, GlWindow};
use na::{
    Matrix4, Orthographic3, Rotation3, SMatrix, SVector, Scale3, Translation3, Vector2, Vector4,
};
use raw_window_handle::HasRawWindowHandle;
use std::{
    collections::HashSet, error::Error, fs::File, io::Read, num::NonZeroU32, path::Path,
    process::exit,
};
use winit::{
    dpi::LogicalSize,
    event::{ElementState, Event, KeyEvent, WindowEvent},
    event_loop::{EventLoop, EventLoopBuilder},
    keyboard::{Key, NamedKey},
    window::WindowBuilder,
};

struct Camera {
    pos: Translation3<f32>,
    angle: Rotation3<f32>,
    scale: Scale3<f32>,
}

impl Camera {
    fn get_view_matrix(&self) -> Matrix4<f32> {
        let view_matrix: Matrix4<f32> = self.pos.to_homogeneous() * self.angle.to_homogeneous();

        view_matrix.try_inverse().unwrap()
    }

    fn get_projection_matrix(&self) -> Matrix4<f32> {
        let ortho: Orthographic3<f32> = Orthographic3::new(
            -1.0 * self.scale.x,
            1.0 * self.scale.x,
            -1.0 * self.scale.y,
            1.0 * self.scale.y,
            -1.0 * self.scale.z,
            1.0 * self.scale.z,
        );
        ortho.to_homogeneous()
    }
}

struct KeysPressed {
    keys_down: HashSet<Key>,
}

impl KeysPressed {
    fn key_down(&self) -> bool {
        !self.keys_down.is_empty()
    }

    fn set_key(&mut self, key: Key, state: ElementState) {
        let _ = match state {
            ElementState::Pressed => self.keys_down.insert(key),
            ElementState::Released => self.keys_down.remove(&key),
        };
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    unsafe {
        let args = Command::new("fractal")
            .about("A real-time OpenGL renderer for fractals")
            .arg(
                Arg::new("mandelbrot")
                    .short('m')
                    .long("mandelbrot")
                    .num_args(0)
                    .help("Render the Mandelbrot set")
                    .conflicts_with("koch-snowflake"),
            )
            .arg(
                Arg::new("koch-snowflake")
                    .short('k')
                    .long("koch-snowflake")
                    .num_args(0)
                    .help("Render the Koch Snowflake fractal"),
            )
            .get_matches();

        let fragment_shader = if args.get_flag("mandelbrot") {
            "mandelbrot.frag"
        } else {
            "koch_snowflake.frag"
        };

        let fragment_shader =
            match read_shader_file(Path::new("src").join(fragment_shader).to_str().unwrap()) {
                Ok(contents) => contents,
                Err(e) => {
                    eprintln!("Failed to read the shader file: {}", e);
                    exit(1)
                }
            };

        // Create context from a winit window
        let (gl, gl_surface, gl_context, window, event_loop) = create_context();

        // Create shader program from source
        let program = create_program(&gl, include_str!("simple.vert"), &fragment_shader);
        gl.use_program(Some(program));

        // Create vertex buffer and vertex array object
        let vertices: [Vector4<f32>; 4] = [
            Vector4::new(-1.0, -1.0, 0.0, 1.0),
            Vector4::new(1.0, -1.0, 0.0, 1.0),
            Vector4::new(1.0, 1.0, 0.0, 1.0),
            Vector4::new(-1.0, 1.0, 0.0, 1.0),
        ];

        let (_vbo, _vao) = create_vertex_buffer(&gl, &vertices);

        // Initialise Camera
        let mut cam: Camera = Camera {
            pos: Translation3::<f32>::identity(),
            angle: Rotation3::<f32>::identity(),
            scale: Scale3::<f32>::identity(),
        };

        gl.clear_color(1.0, 1.0, 1.0, 1.0);

        let mut current_keys: KeysPressed = KeysPressed {
            keys_down: HashSet::new(),
        };

        let _ = event_loop.run(move |event, elwt| {
            if let Event::WindowEvent { event, .. } = event {
                match event {
                    WindowEvent::CloseRequested => {
                        elwt.exit();
                    }
                    WindowEvent::RedrawRequested => {
                        if current_keys.key_down() {
                            for key in current_keys.keys_down.clone().into_iter() {
                                match key.as_ref() {
                                    Key::Named(NamedKey::ArrowRight) => {
                                        cam.pos.x -= 0.01 / cam.scale.x;
                                    }
                                    Key::Named(NamedKey::ArrowLeft) => {
                                        cam.pos.x += 0.01 / cam.scale.x;
                                    }
                                    Key::Named(NamedKey::ArrowUp) => {
                                        cam.pos.y -= 0.01 / cam.scale.y;
                                    }
                                    Key::Named(NamedKey::ArrowDown) => {
                                        cam.pos.y += 0.01 / cam.scale.y;
                                    }
                                    Key::Character("w") => {
                                        cam.scale *= 1.01;
                                    }
                                    Key::Character("s") => {
                                        cam.scale *= 0.99;
                                    }
                                    Key::Character("d") => {
                                        let (roll, pitch, yaw) = cam.angle.euler_angles();
                                        cam.angle =
                                            Rotation3::from_euler_angles(roll, pitch, yaw - 0.01);
                                    }
                                    Key::Character("a") => {
                                        let (roll, pitch, yaw) = cam.angle.euler_angles();
                                        cam.angle =
                                            Rotation3::from_euler_angles(roll, pitch, yaw + 0.01);
                                    }
                                    _ => (),
                                }
                            }

                            window.request_redraw()
                        }

                        // Create MVP matrix
                        let mvp: Matrix4<f32> = cam.get_view_matrix() * cam.get_projection_matrix();

                        // Set uniforms
                        set_uniform(&gl, program, "u_screenSize", Vector2::new(800.0, 800.0));
                        set_uniform(&gl, program, "u_mvpMatrix", mvp);

                        gl.clear(glow::COLOR_BUFFER_BIT);
                        gl.draw_arrays(glow::TRIANGLE_FAN, 0, 4);
                        gl_surface.swap_buffers(&gl_context).unwrap();
                    }
                    WindowEvent::KeyboardInput {
                        event:
                            KeyEvent {
                                logical_key: key,
                                state,
                                ..
                            },
                        ..
                    } => match key.as_ref() {
                        Key::Named(NamedKey::ArrowRight)
                        | Key::Named(NamedKey::ArrowLeft)
                        | Key::Named(NamedKey::ArrowUp)
                        | Key::Named(NamedKey::ArrowDown)
                        | Key::Character("w")
                        | Key::Character("a")
                        | Key::Character("s")
                        | Key::Character("d") => {
                            current_keys.set_key(key, state);
                            window.request_redraw();
                        }
                        _ => (),
                    },
                    _ => (),
                }
            }
        });

        Ok(())
    }
}

unsafe fn create_context() -> (
    glow::Context,
    glutin::surface::Surface<WindowSurface>,
    glutin::context::PossiblyCurrentContext,
    winit::window::Window,
    EventLoop<()>,
) {
    // Create event loop
    let event_loop = EventLoopBuilder::new()
        .build()
        .expect("event loop building");

    // Windows requires the window before display creation
    let window_builder = WindowBuilder::new()
        .with_transparent(true)
        .with_title("Fractal Generator")
        .with_inner_size(LogicalSize::new(800, 800));

    let template = ConfigTemplateBuilder::new().with_alpha_size(8);

    let display_builder = DisplayBuilder::new().with_window_builder(Some(window_builder));

    let (window, gl_config) = display_builder
        .build(&event_loop, template, gl_config_picker)
        .unwrap();

    let raw_window_handle = window.as_ref().map(|window| window.raw_window_handle());

    let gl_display = gl_config.display();

    // Context creation
    let context_attributes = ContextAttributesBuilder::new().build(raw_window_handle);

    let not_current_gl_context = gl_display
        .create_context(&gl_config, &context_attributes)
        .expect("failed to create context");

    let window = window.unwrap();

    // Create surface
    let attrs = window.build_surface_attributes(Default::default());
    let gl_surface = gl_display
        .create_window_surface(&gl_config, &attrs)
        .unwrap();

    // Make context current
    let gl_context = not_current_gl_context.make_current(&gl_surface).unwrap();

    let gl = glow::Context::from_loader_function_cstr(move |s| {
        gl_display.get_proc_address(s) as *const _
    });

    gl_surface
        .set_swap_interval(&gl_context, SwapInterval::Wait(NonZeroU32::new(1).unwrap()))
        .unwrap();

    (gl, gl_surface, gl_context, window, event_loop)
}

pub fn gl_config_picker(configs: Box<dyn Iterator<Item = Config> + '_>) -> Config {
    configs
        .reduce(|accum, config| {
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

unsafe fn create_program(
    gl: &glow::Context,
    vertex_shader_source: &str,
    fragment_shader_source: &str,
) -> glow::NativeProgram {
    let program = gl.create_program().expect("Cannot create program");

    let shader_sources = [
        (glow::VERTEX_SHADER, vertex_shader_source),
        (glow::FRAGMENT_SHADER, fragment_shader_source),
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

    program
}

unsafe fn create_vertex_buffer<const D: usize>(
    gl: &glow::Context,
    vertices: &[SVector<f32, D>],
) -> (NativeBuffer, NativeVertexArray) {
    let vertices_u8: &[u8] = core::slice::from_raw_parts(
        vertices.as_ptr() as *const u8,
        vertices.len() * core::mem::size_of::<SVector<f32, D>>(),
    );

    let vbo = gl.create_buffer().unwrap();
    gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));
    gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, vertices_u8, glow::STATIC_DRAW);

    let vao = gl.create_vertex_array().unwrap();
    gl.bind_vertex_array(Some(vao));
    gl.enable_vertex_attrib_array(0);
    gl.vertex_attrib_pointer_f32(0, D as i32, glow::FLOAT, false, 0, 0);

    (vbo, vao)
}

unsafe fn set_uniform<const R: usize, const C: usize>(
    gl: &glow::Context,
    program: NativeProgram,
    name: &str,
    value: SMatrix<f32, R, C>,
) {
    let location = gl.get_uniform_location(program, name);
    match C {
        1 => match R {
            1 => gl.uniform_1_f32(location.as_ref(), value[0]),
            2 => gl.uniform_2_f32(location.as_ref(), value[0], value[1]),
            _ => (),
        },
        4 => match R {
            4 => gl.uniform_matrix_4_f32_slice(location.as_ref(), false, value.as_slice()),
            _ => (),
        },
        _ => (),
    }
}

fn read_shader_file(file_path: &str) -> Result<String, std::io::Error> {
    let mut file = File::open(file_path)?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok(contents)
}
