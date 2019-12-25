use luminance::tess::Tess;

// TODO - Maybe add transform, rotation and method to return a model matrix
pub struct Cube {
    pub tess: Tess,
    pub mat: Material,
}

impl Cube {
    pub fn new(
        tess: Tess,
        ambient: [f32; 3],
        diffuse: [f32; 3],
        specular: [f32; 3],
        shininess: f32,
    ) -> Self {
        let mat = Material {
            ambient,
            diffuse,
            specular,
            shininess,
        };

        Cube { tess, mat }
    }
}

pub struct Material {
    pub ambient: [f32; 3],
    pub diffuse: [f32; 3],
    pub specular: [f32; 3],
    pub shininess: f32,
}
