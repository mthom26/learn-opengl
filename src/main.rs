use luminance::{
    context::GraphicsContext,
    render_state::RenderState,
    shader::program::Program,
    tess::{Mode, TessBuilder},
};
use luminance_derive::{Semantics, Vertex};
use luminance_glutin::{
    ElementState, Event, GlutinSurface, KeyboardInput, Surface, VirtualKeyCode, WindowDim,
    WindowEvent, WindowOpt,
};

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

// Shaders
const VS: &str = include_str!("../shaders/vertex.glsl");
const FS: &str = include_str!("../shaders/fragment.glsl");

#[derive(Copy, Clone, Debug, Semantics)]
pub enum Semantics {
    #[sem(name = "position", repr = "[f32; 3]", wrapper = "VertexPos")]
    Position,
}

#[repr(C)]
#[derive(Vertex)]
#[vertex(sem = "Semantics")]
struct Vertex {
    pos: VertexPos,
}

impl Vertex {
    // Convenience function to build a Vertex
    fn from(pos: [f32; 3]) -> Self {
        Vertex {
            pos: VertexPos::new(pos),
        }
    }
}

fn main() {
    let mut surface = GlutinSurface::new(
        WindowDim::Windowed(WIDTH, HEIGHT),
        "Learn OpenGL",
        WindowOpt::default(),
    )
    .expect("Could not create GLUTIN surface.");

    let back_buffer = surface.back_buffer().unwrap();

    // Shader Program
    let program: Program<Semantics, (), ()> = Program::from_strings(None, VS, None, FS)
        .unwrap()
        .ignore_warnings();

    // Triangle
    let vertices = [
        Vertex::from([0.0, 0.5, 0.0]),
        Vertex::from([0.5, -0.5, 0.0]),
        Vertex::from([-0.5, -0.5, 0.0]),
    ];

    let triangle = TessBuilder::new(&mut surface)
        .add_vertices(vertices)
        .set_mode(Mode::Triangle)
        .build()
        .unwrap();

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

        surface
            .pipeline_builder()
            .pipeline(&back_buffer, clear_color, |_, mut shd_gate| {
                shd_gate.shade(&program, |_, mut rdr_gate| {
                    rdr_gate.render(RenderState::default(), |mut tess_gate| {
                        tess_gate.render(&triangle);
                    });
                });
            });

        surface.swap_buffers();
    }
}
