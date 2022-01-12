use crate::film::color::Color;

pub trait Texture {}

//////////////////////////////////////

pub struct ConstantColor(pub Color);

impl Texture for ConstantColor {}

//////////////////////////////////////

pub struct ConstantScalar(pub f32);

impl Texture for ConstantScalar {}
