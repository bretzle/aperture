use crate::{
    film::Colorf,
    texture::{Texture, Textures},
};

/// A single valued, solid scalar texture
pub struct ConstantScalar {
    val: f32,
}

impl ConstantScalar {
    pub fn new(val: f32) -> Textures {
        Textures::ConstantScalar(Self { val })
    }
}

impl Texture for ConstantScalar {
    fn sample_f32(&self, _: f32, _: f32, _: f32) -> f32 {
        self.val
    }

    fn sample_color(&self, _: f32, _: f32, _: f32) -> Colorf {
        Colorf::broadcast(self.val)
    }
}

/// A single valued, solid color texture
pub struct ConstantColor {
    val: Colorf,
}

impl ConstantColor {
    pub fn new(val: Colorf) -> Textures {
        Textures::ConstantColor(Self { val })
    }
}

impl Texture for ConstantColor {
    fn sample_f32(&self, _: f32, _: f32, _: f32) -> f32 {
        self.val.luminance()
    }

    fn sample_color(&self, _: f32, _: f32, _: f32) -> Colorf {
        self.val
    }
}

pub struct UVColor;

impl Texture for UVColor {
    fn sample_f32(&self, u: f32, v: f32, _: f32) -> f32 {
        Colorf::new(u, v, 0.0).luminance()
    }

    fn sample_color(&self, u: f32, v: f32, _: f32) -> Colorf {
        Colorf::new(u, v, 0.0)
    }
}
