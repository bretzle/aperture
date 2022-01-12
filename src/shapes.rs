use crate::geometry::{Boundable, Geometry, Sampleable};

pub struct Sphere {
    radius: f32,
}

impl Sphere {
    pub fn new(radius: f32) -> Self {
        Self { radius }
    }
}

impl Geometry for Sphere {}
impl Boundable for Sphere {}
impl Sampleable for Sphere {}
