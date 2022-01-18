use crate::{
    film::Colorf,
    linalg::clamp,
    texture::{bilinear_interpolate, Texture, Textures},
};
use image::{self, GenericImage};

/// An `Image` texture is a `Texture` whose samples come
/// from an image file.
pub struct Image {
    img: image::DynamicImage,
}

impl Image {
    pub fn new_texture(img: image::DynamicImage) -> Textures {
        Textures::Image(Image { img })
    }

    fn get_float(&self, x: u32, y: u32) -> f32 {
        let dims = self.img.dimensions();
        let x = clamp(x, 0, dims.0 - 1);
        let y = clamp(y, 0, dims.1 - 1);
        self.img.get_pixel(x, y).data[0] as f32 / 255.0
    }

    fn get_color(&self, x: u32, y: u32) -> Colorf {
        let dims = self.img.dimensions();
        let x = clamp(x, 0, dims.0 - 1);
        let y = clamp(y, 0, dims.1 - 1);
        let px = self.img.get_pixel(x, y);
        Colorf::with_alpha(
            px.data[0] as f32 / 255.0,
            px.data[1] as f32 / 255.0,
            px.data[2] as f32 / 255.0,
            px.data[3] as f32 / 255.0,
        )
    }
}

impl Texture for Image {
    fn sample_f32(&self, u: f32, v: f32, _: f32) -> f32 {
        let dims = self.img.dimensions();
        let x = u * dims.0 as f32;
        let y = v * dims.1 as f32;
        bilinear_interpolate(x, y, |px, py| self.get_float(px, py))
    }

    fn sample_color(&self, u: f32, v: f32, _: f32) -> Colorf {
        let x = u * self.img.dimensions().0 as f32;
        let y = v * self.img.dimensions().1 as f32;
        bilinear_interpolate(x, y, |px, py| self.get_color(px, py))
    }
}
