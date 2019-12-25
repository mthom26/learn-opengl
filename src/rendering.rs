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
    #[sem(name = "normal", repr = "[f32; 3]", wrapper = "VertexNormal")]
    Normal,
    #[sem(name = "tex_coord", repr = "[f32; 2]", wrapper = "VertexTexCoord")]
    TexCoord,
}

#[repr(C)]
#[derive(Vertex)]
#[vertex(sem = "Semantics")]
pub struct Vertex {
    pos: VertexPos,
    #[vertex(normalized = "true")]
    normal: VertexNormal,
    #[vertex(normalized = "true")]
    tex_coord: VertexTexCoord,
}

impl Vertex {
    // Convenience function to build a Vertex
    pub fn from(pos: [f32; 3], normal: [f32; 3], tex_coord: [f32; 2]) -> Self {
        Vertex {
            pos: VertexPos::new(pos),
            normal: VertexNormal::new(normal),
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
    // #[uniform(unbound)]
    // pub object_color: Uniform<[f32; 3]>,
    #[uniform(unbound)]
    pub mat_amb: Uniform<[f32; 3]>,
    #[uniform(unbound)]
    pub mat_diff: Uniform<[f32; 3]>,
    #[uniform(unbound)]
    pub mat_spec: Uniform<[f32; 3]>,
    #[uniform(unbound)]
    pub mat_shininess: Uniform<f32>,

    #[uniform(unbound)]
    pub light_amb: Uniform<[f32; 3]>,
    #[uniform(unbound)]
    pub light_diff: Uniform<[f32; 3]>,
    #[uniform(unbound)]
    pub light_spec: Uniform<[f32; 3]>,
    #[uniform(unbound)]
    pub light_pos: Uniform<[f32; 3]>,
    #[uniform(unbound)]
    pub cam_pos: Uniform<[f32; 3]>,
}
