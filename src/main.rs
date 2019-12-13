use std::{path::Path, time::Instant};

use luminance::{context::GraphicsContext, render_state::RenderState, shader::program::Program};
use luminance_glutin::{
    ElementState, Event, GlutinSurface, KeyboardInput, Surface, VirtualKeyCode, WindowDim,
    WindowEvent, WindowOpt,
};

mod rendering;
use rendering::{Semantics, ShaderInterface};
mod utils;
use utils::{load_texture_rgb, load_texture_rgba};
mod shapes;
use shapes::{quad, triangle};

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

// Shaders
const VS: &str = include_str!("../shaders/vertex.glsl");
const FS: &str = include_str!("../shaders/fragment.glsl");

fn main() {
    let mut surface = GlutinSurface::new(
        WindowDim::Windowed(WIDTH, HEIGHT),
        "Learn OpenGL",
        WindowOpt::default(),
    )
    .expect("Could not create GLUTIN surface.");

    let back_buffer = surface.back_buffer().unwrap();

    // Shader Program
    let program: Program<Semantics, (), ShaderInterface> =
        Program::from_strings(None, VS, None, FS)
            .expect("Could not create program.")
            .ignore_warnings();

    // Triangle
    let _triangle = triangle(&mut surface, None);

    // Quad
    let quad = quad(&mut surface, None);

    // Load textures
    let (tex, _width, _height) =
        load_texture_rgb(&mut surface, Path::new("assets/textures/container.jpg"));
    let (smiley, _width, _height) =
        load_texture_rgba(&mut surface, Path::new("assets/textures/awesomeface.png"));

    let t_start = Instant::now();

    'app: loop {
        // Handle Input
        for event in surface.poll_events() {
            if let Event::WindowEvent { event, .. } = event {
                match event {
                    // Close the window
                    WindowEvent::CloseRequested
                    | WindowEvent::Destroyed
                    | WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                state: ElementState::Released,
                                virtual_keycode: Some(VirtualKeyCode::Escape),
                                ..
                            },
                        ..
                    } => break 'app,
                    _ => (),
                }
            }
        }

        // Rendering
        let clear_color = [0.2, 0.3, 0.3, 1.0];
        let t = {
            // Value varies from 0.0 to 1.0
            let t = t_start.elapsed().as_millis() as f32 / 1000.0;
            t.sin().abs()
        };

        surface
            .pipeline_builder()
            .pipeline(&back_buffer, clear_color, |pipeline, mut shd_gate| {
                let bound_tex = pipeline.bind_texture(&tex);
                let bound_smiley = pipeline.bind_texture(&smiley);

                shd_gate.shade(&program, |iface, mut rdr_gate| {
                    // Update the time uniform
                    iface.time.update(t);
                    // Update textures
                    iface.tex.update(&bound_tex);
                    iface.tex_smiley.update(&bound_smiley);

                    rdr_gate.render(RenderState::default(), |mut tess_gate| {
                        tess_gate.render(&quad);
                    });
                });
            });

        surface.swap_buffers();
    }
}
