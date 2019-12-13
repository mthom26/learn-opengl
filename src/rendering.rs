use luminance::{
    linear::M44,
    pipeline::BoundTexture,
    pixel::NormUnsigned,
    shader::program::Uniform,
    texture::{Dim2, Flat},
};
use luminance_derive::{Semantics, UniformInterface, Vertex};

#[derive(Copy, Clone, Debug, Semantics)]
pub enum Semantics {
    #[sem(name = "position", repr = "[f32; 3]", wrapper = "VertexPos")]
    Position,
    #[sem(name = "color", repr = "[u8; 3]", wrapper = "VertexCol")]
    Color,
    #[sem(name = "tex_coord", repr = "[f32; 2]", wrapper = "VertexTexCoord")]
    TexCoord,
}

#[repr(C)]
#[derive(Vertex)]
#[vertex(sem = "Semantics")]
pub struct Vertex {
    pos: VertexPos,
    #[vertex(normalized = "true")]
    col: VertexCol,
    #[vertex(normalized = "true")]
    tex_coord: VertexTexCoord,
}

impl Vertex {
    // Convenience function to build a Vertex
    pub fn from(pos: [f32; 3], col: [u8; 3], tex_coord: [f32; 2]) -> Self {
        Vertex {
            pos: VertexPos::new(pos),
            col: VertexCol::new(col),
            tex_coord: VertexTexCoord::new(tex_coord),
        }
    }
}

#[derive(UniformInterface)]
pub struct ShaderInterface {
    // A simple float ranging from 0.0 to 1.0 changing over time
    #[uniform(unbound)]
    pub time: Uniform<f32>,
    pub tex: Uniform<&'static BoundTexture<'static, Flat, Dim2, NormUnsigned>>,
    pub tex_smiley: Uniform<&'static BoundTexture<'static, Flat, Dim2, NormUnsigned>>,
    #[uniform(unbound)]
    pub trans: Uniform<M44>,
}
