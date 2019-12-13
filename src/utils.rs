use std::path::Path;

use luminance::{
    pixel::{NormRGB8UI, NormRGBA8UI},
    texture::{Dim2, Flat, GenMipmaps, Sampler, Texture},
};
use luminance_glutin::GlutinSurface;

use ultraviolet::mat::Mat4;

// Load an rgb texture
pub fn load_texture_rgb(
    surface: &mut GlutinSurface,
    path: &Path,
) -> (Texture<Flat, Dim2, NormRGB8UI>, u32, u32) {
    let img = image::open(path)
        .map(|img| img.flipv().to_rgb())
        .expect("Could not create image.");

    let (width, height) = img.dimensions();
    let texels = img.into_raw();

    let tex = Texture::new(surface, [width, height], 0, Sampler::default())
        .expect("Failed to create Texture.");

    tex.upload_raw(GenMipmaps::No, &texels).unwrap();

    (tex, width, height)
}

// Load an rgba texture
pub fn load_texture_rgba(
    surface: &mut GlutinSurface,
    path: &Path,
) -> (Texture<Flat, Dim2, NormRGBA8UI>, u32, u32) {
    let img = image::open(path)
        .map(|img| img.flipv().to_rgba())
        .expect("Could not create image.");

    let (width, height) = img.dimensions();
    let texels = img.into_raw();

    let tex = Texture::new(surface, [width, height], 0, Sampler::default())
        .expect("Failed to create Texture.");

    tex.upload_raw(GenMipmaps::No, &texels).unwrap();

    (tex, width, height)
}

// Utility function to convert the ultraviolet `Mat4` to the needed `M44` for the
// shader. Note the scale here only works for rotating around the z-axis.
pub fn convert_mat4(mat: Mat4, scale: f32) -> [[f32; 4]; 4] {
    [
        [
            mat.cols[0].x * scale,
            mat.cols[0].y * scale,
            mat.cols[0].z,
            mat.cols[0].w,
        ],
        [
            mat.cols[1].x * scale,
            mat.cols[1].y * scale,
            mat.cols[1].z,
            mat.cols[1].w,
        ],
        [
            mat.cols[2].x,
            mat.cols[2].y,
            mat.cols[2].z * scale,
            mat.cols[2].w,
        ],
        [mat.cols[3].x, mat.cols[3].y, mat.cols[3].z, mat.cols[3].w],
    ]
}
