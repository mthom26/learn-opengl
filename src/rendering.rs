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
    #[uniform(unbound)]
    pub model: Uniform<M44>,
    pub view: Uniform<M44>,
    pub proj: Uniform<M44>,

    // Lighting
    #[uniform(unbound)]
    pub object_color: Uniform<[f32; 3]>,
    #[uniform(unbound)]
    pub light_color: Uniform<[f32; 3]>,
}
