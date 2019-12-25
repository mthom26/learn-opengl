use std::time::Instant;

use luminance::{context::GraphicsContext, render_state::RenderState, shader::program::Program};
use luminance_glutin::{
    ElementState, Event, GlutinSurface, KeyboardInput, Surface, VirtualKeyCode, WindowDim,
    WindowEvent, WindowOpt,
};

use ultraviolet::{
    bivec::Bivec3, mat::Mat4, projection::rh_yup::perspective_gl, rotor::Rotor3,
    transform::Similarity3, vec::Vec3,
};

mod rendering;
use rendering::{Semantics, ShaderInterface};
mod utils;
use utils::convert_mat4;
mod shapes;
use shapes::cube;
mod objects;
use objects::Cube;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

// Shaders
const VS: &str = include_str!("../shaders/vertex.glsl");
const FS: &str = include_str!("../shaders/fragment.glsl");
const LAMP_FS: &str = include_str!("../shaders/lamp_frag.glsl");

fn main() {
    let mut surface = GlutinSurface::new(
        WindowDim::Windowed(WIDTH, HEIGHT),
        "Learn OpenGL",
        WindowOpt::default(),
    )
    .expect("Could not create GLUTIN surface.");

    let back_buffer = surface.back_buffer().unwrap();

    // Shader Programs
    let program: Program<Semantics, (), ShaderInterface> =
        Program::from_strings(None, VS, None, FS)
            .expect("Could not create program.")
            .ignore_warnings();

    let lamp_program: Program<Semantics, (), ShaderInterface> =
        Program::from_strings(None, VS, None, LAMP_FS)
            .expect("Could not create program.")
            .ignore_warnings();

    // Cube
    let cube_01 = Cube::new(
        cube(&mut surface, None, 0.5),
        [0.9, 0.9, 0.9], // [1.0, 0.5, 0.31],
        [0.9, 0.9, 0.9], // [1.0, 0.5, 0.31],
        [0.5, 0.5, 0.5], // [0.5, 0.5, 0.5],
        10.0,
    );

    // Lamp
    let lamp = cube(&mut surface, None, 0.1);

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

        // Update state
        let clear_color = [0.2, 0.2, 0.2, 1.0];
        let t: f32 = t_start.elapsed().as_millis() as f32 / 1000.0;

        //--- Get view and projection matrices ---//

        // Camera and View matrix
        let (cam_x, cam_y, cam_z) = (-1.2, 1.1, 5.0);
        let cam_pos = Vec3::new(cam_x, cam_y, cam_z);
        let look_target = Vec3::new(0.0, 0.0, 0.0);
        let up = Vec3::new(0.0, 1.0, 0.0);
        let view = Mat4::look_at(cam_pos, look_target, up);
        let view_mat = convert_mat4(view);

        // Projection
        let projection = perspective_gl(
            (45.0 as f32).to_radians(),
            WIDTH as f32 / HEIGHT as f32,
            0.1,
            100.0,
        );
        let proj_mat = convert_mat4(projection);
        //-------------------------------------- -//

        //--- Update transform state for cube_01 ---//
        let model_mat_cube_01 = {
            let rot_vec = Vec3::new(0.5, -1.0, 0.0).normalized(); // Rotation axis
            let bi_vec = Bivec3::from_normalized_axis(rot_vec);
            let rot = Rotor3::from_angle_plane(0.0, bi_vec);

            let translate = Vec3::new(0.0, 0.0, 0.0);

            let mut sim = Similarity3::identity();
            sim.prepend_rotation(rot);
            sim.append_translation(translate);
            convert_mat4(sim.into_homogeneous_matrix())
        };
        //---------------------------------------//

        //--- Update transform state for lamp ---//
        let lamp_pos = [t.sin(), t.cos() * 0.5 - 0.2, 1.9];
        let model_mat_lamp = {
            let rot_vec = Vec3::new(0.5, -1.0, 0.0).normalized(); // Rotation axis
            let bi_vec = Bivec3::from_normalized_axis(rot_vec);
            let rot = Rotor3::from_angle_plane(0.0, bi_vec);

            let translate = Vec3::new(lamp_pos[0], lamp_pos[1], lamp_pos[2]);

            let mut sim = Similarity3::identity();
            sim.prepend_rotation(rot);
            sim.append_translation(translate);
            convert_mat4(sim.into_homogeneous_matrix())
        };
        //---------------------------------------//

        // Rendering
        surface.pipeline_builder().pipeline(
            &back_buffer,
            clear_color,
            |_pipeline, mut shd_gate| {
                // Cube
                shd_gate.shade(&program, |iface, mut rdr_gate| {
                    // Update model, view and projection matrices
                    iface.model.update(model_mat_cube_01);
                    iface.view.update(view_mat);
                    iface.proj.update(proj_mat);

                    // Update object color
                    iface.mat_amb.update(cube_01.mat.ambient);
                    iface.mat_diff.update(cube_01.mat.diffuse);
                    iface.mat_spec.update(cube_01.mat.specular);
                    iface.mat_shininess.update(cube_01.mat.shininess);

                    // Update light color
                    iface.light_amb.update([0.15, 0.15, 0.2]);
                    iface.light_diff.update([0.6, 0.6, 0.6]);
                    iface.light_spec.update([0.9, 0.8, 0.7]);
                    iface.light_pos.update(lamp_pos);

                    // Update camera position
                    iface.cam_pos.update([cam_x, cam_y, cam_z]);

                    rdr_gate.render(RenderState::default(), |mut tess_gate| {
                        tess_gate.render(&cube_01.tess);
                    });
                });
                // Lamp
                shd_gate.shade(&lamp_program, |iface, mut rdr_gate| {
                    // Update model, view and projection matrices
                    iface.model.update(model_mat_lamp);
                    iface.view.update(view_mat);
                    iface.proj.update(proj_mat);

                    // Object and light color not needed here
                    rdr_gate.render(RenderState::default(), |mut tess_gate| {
                        tess_gate.render(&lamp);
                    });
                });
            },
        );

        surface.swap_buffers();
    }
}
