use std::path::Path;

use luminance::{
    pixel::{NormRGB8UI, NormRGBA8UI},
    texture::{Dim2, Flat, GenMipmaps, Sampler, Texture},
};
use luminance_glutin::GlutinSurface;

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
