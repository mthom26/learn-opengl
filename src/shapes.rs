use luminance::tess::{Mode, Tess, TessBuilder};
use luminance_glutin::GlutinSurface;

use crate::rendering::Vertex;

pub fn triangle(surface: &mut GlutinSurface, vertices: Option<&[Vertex]>) -> Tess {
    let default = [
        Vertex::from([0.0, 0.5, 0.0], [230, 30, 30], [0.0, 0.0]),
        Vertex::from([0.5, -0.5, 0.0], [30, 230, 30], [0.0, 0.0]),
        Vertex::from([-0.5, -0.5, 0.0], [30, 30, 230], [0.0, 0.0]),
    ];

    let verts = match vertices {
        Some(vertices) => vertices,
        // No vertices supplied so return a default
        None => &default,
    };

    TessBuilder::new(surface)
        .add_vertices(verts)
        .set_mode(Mode::Triangle)
        .build()
        .unwrap()
}

pub fn quad(surface: &mut GlutinSurface, vertices: Option<&[Vertex]>) -> Tess {
    let default = [
        Vertex::from([-0.5, -0.5, 0.0], [255, 0, 0], [0.0, 0.0]), // Bottom left
        Vertex::from([0.5, -0.5, 0.0], [0, 255, 0], [1.0, 0.0]),  // Bottom right
        Vertex::from([0.5, 0.5, 0.0], [255, 255, 0], [1.0, 1.0]), // Top right
        Vertex::from([-0.5, 0.5, 0.0], [30, 30, 255], [0.0, 1.0]), // Top left
    ];

    let verts = match vertices {
        Some(vertices) => vertices,
        // No vertices supplied so return a default
        None => &default,
    };

    TessBuilder::new(surface)
        .add_vertices(verts)
        .set_mode(Mode::TriangleFan)
        .build()
        .unwrap()
}
