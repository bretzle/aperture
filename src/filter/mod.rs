use std::f32::consts::PI;

use crate::geometry::{Point2f, Vector2f};

pub trait Filter: Send + Sync {
    fn evaluate(&self, p: &Point2f) -> f32;
    fn radius(&self) -> &Vector2f;
    fn inv_radius(&self) -> &Vector2f;
}

pub struct BoxFilter {
    radius: Vector2f,
    inv_radius: Vector2f,
}

impl BoxFilter {
    pub fn new(vec: Vector2f) -> Self {
        Self {
            inv_radius: vec.inverse(),
            radius: vec,
        }
    }
}

impl Filter for BoxFilter {
    fn evaluate(&self, _: &Point2f) -> f32 {
        1.0
    }

    fn radius(&self) -> &Vector2f {
        &self.radius
    }

    fn inv_radius(&self) -> &Vector2f {
        &self.inv_radius
    }
}

pub struct TriangleFilter {
    radius: Vector2f,
    inv_radius: Vector2f,
}

impl TriangleFilter {
    pub fn new(radius: Vector2f) -> Self {
        Self {
            inv_radius: radius.inverse(),
            radius,
        }
    }
}

impl Filter for TriangleFilter {
    fn evaluate(&self, p: &Point2f) -> f32 {
        (self.radius.x - p.x.abs()).max(0.0) * (self.radius.y - p.y.abs()).max(0.0)
    }

    fn radius(&self) -> &Vector2f {
        &self.radius
    }

    fn inv_radius(&self) -> &Vector2f {
        &self.inv_radius
    }
}

pub struct GaussianFilter {
    radius: Vector2f,
    inv_radius: Vector2f,
    alpha: f32,
    expx: f32,
    expy: f32,
}

impl GaussianFilter {
    pub fn new(radius: Vector2f, alpha: f32) -> GaussianFilter {
        Self {
            inv_radius: radius.inverse(),
            expx: (-alpha * radius.x * radius.x).exp(),
            expy: (-alpha * radius.y * radius.y).exp(),
            alpha,
            radius,
        }
    }

    fn gaussian(&self, d: f32, expv: f32) -> f32 {
        ((-self.alpha * d * d).exp() - expv).max(0.0)
    }
}

impl Filter for GaussianFilter {
    fn evaluate(&self, p: &Point2f) -> f32 {
        self.gaussian(p.x, self.expx) * self.gaussian(p.y, self.expy)
    }

    fn radius(&self) -> &Vector2f {
        &self.radius
    }

    fn inv_radius(&self) -> &Vector2f {
        &self.inv_radius
    }
}

pub struct MitchellNetravali {
    radius: Vector2f,
    inv_radius: Vector2f,
    b: f32,
    c: f32,
}

impl MitchellNetravali {
    pub fn new(radius: Vector2f, b: f32, c: f32) -> Self {
        Self {
            inv_radius: radius.inverse(),
            radius,
            b,
            c,
        }
    }

    fn mitchell_1d(&self, x: f32) -> f32 {
        let x = x.abs() * 2.0;
        if x > 1.0 {
            ((-self.b - 6.0 * self.c) * x * x * x
                + (6.0 * self.b + 30.0 * self.c) * x * x
                + (-12.0 * self.b - 48.0 * self.c) * x
                + (8.0 * self.b + 24.0 * self.c))
                * (1.0 / 6.0)
        } else {
            ((12.0 - 9.0 * self.b - 6.0 * self.c) * x * x * x
                + (-18.0 + 12.0 * self.b + 6.0 * self.c) * x * x
                + (6.0 - 2.0 * self.b))
                * (1.0 / 6.0)
        }
    }
}

impl Filter for MitchellNetravali {
    fn evaluate(&self, p: &Point2f) -> f32 {
        self.mitchell_1d(p.x * self.inv_radius.x) * self.mitchell_1d(p.y * self.inv_radius.y)
    }

    fn radius(&self) -> &Vector2f {
        &self.radius
    }

    fn inv_radius(&self) -> &Vector2f {
        &self.inv_radius
    }
}

impl Default for MitchellNetravali {
    fn default() -> Self {
        Self::new(Vector2f::new(2.0, 2.0), 1.0 / 3.0, 1.0 / 3.0)
    }
}

pub struct LanczosSincFilter {
    radius: Vector2f,
    inv_radius: Vector2f,
    tau: f32,
}

impl LanczosSincFilter {
    pub fn new(radius: Vector2f, tau: f32) -> Self {
        Self {
            inv_radius: radius.inverse(),
            radius,
            tau,
        }
    }

    fn sinc(x: f32) -> f32 {
        let x = x.abs();
        if x < 1E-5 {
            1.0
        } else {
            (PI * x).sin() / (PI * x)
        }
    }

    fn window_sinc(&self, x: f32, radius: f32) -> f32 {
        let x = x.abs();
        if x > radius {
            0.0
        } else {
            Self::sinc(x) * Self::sinc(x / self.tau)
        }
    }
}

impl Filter for LanczosSincFilter {
    fn evaluate(&self, p: &Point2f) -> f32 {
        self.window_sinc(p.x, self.radius.x) * self.window_sinc(p.y, self.radius.y)
    }

    fn radius(&self) -> &Vector2f {
        &self.radius
    }

    fn inv_radius(&self) -> &Vector2f {
        &self.inv_radius
    }
}
