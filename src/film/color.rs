use crate::math;
use std::ops::{Add, Div, Index, IndexMut, Mul, Neg, Sub};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color {
    pub const WHITE: Self = Color::broadcast(1.0);
    pub const BLACK: Self = Color::broadcast(0.0);

    pub const fn new(r: f32, g: f32, b: f32) -> Self {
        Self { r, g, b, a: 1.0 }
    }

    pub const fn broadcast(r: f32) -> Self {
        Self {
            r,
            g: r,
            b: r,
            a: r,
        }
    }

    pub fn clamp(&self) -> Self {
        Self {
            r: math::clamp(self.r, 0.0, 1.0),
            g: math::clamp(self.g, 0.0, 1.0),
            b: math::clamp(self.b, 0.0, 1.0),
            a: math::clamp(self.a, 0.0, 1.0),
        }
    }

    pub fn to_srgb(&self) -> Self {
        let a = 0.055f32;
        let b = 1f32 / 2.4;
        let mut srgb = Color::broadcast(0.0);
        for i in 0..3 {
            if self[i] <= 0.0031308 {
                srgb[i] = 12.92 * self[i];
            } else {
                srgb[i] = (1.0 + a) * f32::powf(self[i], b) - a;
            }
        }
        srgb
    }
}

impl Add for Color {
    type Output = Color;

    fn add(self, rhs: Color) -> Color {
        Color {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b,
            a: self.a + rhs.a,
        }
    }
}

impl Sub for Color {
    type Output = Color;

    fn sub(self, rhs: Color) -> Color {
        Color {
            r: self.r - rhs.r,
            g: self.g - rhs.g,
            b: self.b - rhs.b,
            a: self.a - rhs.a,
        }
    }
}

impl Mul for Color {
    type Output = Color;

    fn mul(self, rhs: Color) -> Color {
        Color {
            r: self.r * rhs.r,
            g: self.g * rhs.g,
            b: self.b * rhs.b,
            a: self.a * rhs.a,
        }
    }
}

impl Mul<f32> for Color {
    type Output = Color;

    fn mul(self, rhs: f32) -> Color {
        Color {
            r: self.r * rhs,
            g: self.g * rhs,
            b: self.b * rhs,
            a: self.a * rhs,
        }
    }
}

impl Mul<Color> for f32 {
    type Output = Color;

    fn mul(self, rhs: Color) -> Color {
        Color {
            r: self * rhs.r,
            g: self * rhs.g,
            b: self * rhs.b,
            a: self * rhs.a,
        }
    }
}

impl Div for Color {
    type Output = Color;

    fn div(self, rhs: Color) -> Color {
        Color {
            r: self.r / rhs.r,
            g: self.g / rhs.g,
            b: self.b / rhs.b,
            a: self.a / rhs.a,
        }
    }
}

impl Div<f32> for Color {
    type Output = Color;

    fn div(self, rhs: f32) -> Color {
        Color {
            r: self.r / rhs,
            g: self.g / rhs,
            b: self.b / rhs,
            a: self.a / rhs,
        }
    }
}

impl Neg for Color {
    type Output = Color;

    fn neg(self) -> Color {
        Color {
            r: -self.r,
            g: -self.g,
            b: -self.b,
            a: -self.a,
        }
    }
}

impl Index<usize> for Color {
    type Output = f32;

    fn index(&self, i: usize) -> &f32 {
        match i {
            0 => &self.r,
            1 => &self.g,
            2 => &self.b,
            3 => &self.a,
            _ => panic!("Invalid index into color"),
        }
    }
}

impl IndexMut<usize> for Color {
    fn index_mut(&mut self, i: usize) -> &mut f32 {
        match i {
            0 => &mut self.r,
            1 => &mut self.g,
            2 => &mut self.b,
            3 => &mut self.a,
            _ => panic!("Invalid index into color"),
        }
    }
}
