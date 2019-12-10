use luminance::shader::program::Uniform;
use luminance_derive::{Semantics, UniformInterface, Vertex};

#[derive(Copy, Clone, Debug, Semantics)]
pub enum Semantics {
    #[sem(name = "position", repr = "[f32; 3]", wrapper = "VertexPos")]
    Position,
    #[sem(name = "color", repr = "[u8; 3]", wrapper = "VertexCol")]
    Color,
}

#[repr(C)]
#[derive(Vertex)]
#[vertex(sem = "Semantics")]
pub struct Vertex {
    pos: VertexPos,
    #[vertex(normalized = "true")]
    col: VertexCol,
}

impl Vertex {
    // Convenience function to build a Vertex
    pub fn from(pos: [f32; 3], col: [u8; 3]) -> Self {
        Vertex {
            pos: VertexPos::new(pos),
            col: VertexCol::new(col),
        }
    }
}

#[derive(Debug, UniformInterface)]
pub struct ShaderInterface {
    // A simple float ranging from 0.0 to 1.0 changing over time
    pub time: Uniform<f32>,
}
