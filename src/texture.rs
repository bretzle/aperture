use crate::color::Color;

pub trait Texture {}

pub struct ConstantColor {
    val: Color,
}

impl ConstantColor {
    pub fn new(val: Color) -> Self {
        Self { val }
    }
}

impl Texture for ConstantColor {}

pub struct ConstantScalar {
    val: f32,
}

impl ConstantScalar {
    pub fn new(val: f32) -> Self {
        Self { val }
    }
}

impl Texture for ConstantScalar {}
