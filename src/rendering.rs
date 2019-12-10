use luminance_derive::{Semantics, Vertex};

#[derive(Copy, Clone, Debug, Semantics)]
pub enum Semantics {
    #[sem(name = "position", repr = "[f32; 3]", wrapper = "VertexPos")]
    Position,
}

#[repr(C)]
#[derive(Vertex)]
#[vertex(sem = "Semantics")]
pub struct Vertex {
    pos: VertexPos,
}

impl Vertex {
    // Convenience function to build a Vertex
    pub fn from(pos: [f32; 3]) -> Self {
        Vertex {
            pos: VertexPos::new(pos),
        }
    }
}
