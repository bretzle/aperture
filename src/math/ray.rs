use super::{Point3, Vector3};
use crate::medium::Medium;
use std::{cell::Cell, sync::Arc};

#[derive(Default, Clone)]
pub struct Ray {
    pub origin: Point3<f32>,
    pub direction: Vector3<f32>,
    pub t_max: Cell<f32>,
    pub time: f32,
    pub medium: Option<Arc<Medium>>,
    pub differential: Option<RayDifferential>,
}

impl Ray {
    // Point3f operator()(Float t) const { return o + d * t; }
    pub fn position(&self, t: f32) -> Point3<f32> {
        self.origin + self.direction * t
    }

    // from class RayDifferential
    pub fn scale_differentials(&mut self, s: f32) {
        if let Some(d) = self.differential.iter_mut().next() {
            d.rx_origin = self.origin + (d.rx_origin - self.origin) * s;
            d.ry_origin = self.origin + (d.ry_origin - self.origin) * s;
            d.rx_direction = self.direction + (d.rx_direction - self.direction) * s;
            d.ry_direction = self.direction + (d.ry_direction - self.direction) * s;
        }
    }
}

#[derive(Debug, Default, Copy, Clone)]
pub struct RayDifferential {
    pub rx_origin: Point3<f32>,
    pub ry_origin: Point3<f32>,
    pub rx_direction: Vector3<f32>,
    pub ry_direction: Vector3<f32>,
}
