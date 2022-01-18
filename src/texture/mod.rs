//! Defines the trait implemented by all textured values

use crate::film::Colorf;
use std::ops::{Add, Mul};

pub use self::{animated_image::AnimatedImage, constant::*, image::Image};

pub mod animated_image;
pub mod constant;
pub mod image;

/// scalars or Colors can be computed on some image texture
/// or procedural generator
#[enum_dispatch(Textures)]
pub trait Texture {
    /// Sample the textured value at texture coordinates u,v
    /// at some time. u and v should be in [0, 1]
    fn sample_f32(&self, u: f32, v: f32, time: f32) -> f32;
    fn sample_color(&self, u: f32, v: f32, time: f32) -> Colorf;
}

#[enum_dispatch]
pub enum Textures {
    AnimatedImage,
    ConstantColor,
    ConstantScalar,
    Image,
    UVColor,
}

fn bilinear_interpolate<T, F>(x: f32, y: f32, get: F) -> T
where
    T: Copy + Add<T, Output = T> + Mul<f32, Output = T>,
    F: Fn(u32, u32) -> T,
{
    let p00 = (x as u32, y as u32);
    let p10 = (p00.0 + 1, p00.1);
    let p01 = (p00.0, p00.1 + 1);
    let p11 = (p00.0 + 1, p00.1 + 1);

    let s00 = get(p00.0, p00.1);
    let s10 = get(p10.0, p10.1);
    let s01 = get(p01.0, p01.1);
    let s11 = get(p11.0, p11.1);

    let sx = x - p00.0 as f32;
    let sy = y - p00.1 as f32;
    s00 * (1.0 - sx) * (1.0 - sy) + s10 * sx * (1.0 - sy) + s01 * (1.0 - sx) * sy + s11 * sx * sy
}
