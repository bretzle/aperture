use crate::{
    geometry::{Bounds3f, Point2f, Ray, Vector3f},
    interaction::{Interaction, SurfaceInteraction},
    utils,
};
use std::fmt::Debug;

mod cone;
mod cylinder;
mod disk;
mod mesh;
mod paraboloid;
mod sphere;

pub trait Shape: Debug + Send + Sync {
    fn intersect(&self, ray: &Ray) -> Option<(SurfaceInteraction, f32)>;

    fn intersect_p(&self, ray: &Ray) -> bool {
        self.intersect(ray).is_some()
    }

    fn area(&self) -> f32;

    fn object_bounds(&self) -> Bounds3f;

    fn world_bounds(&self) -> Bounds3f;

    fn sample(&self, u: Point2f) -> (Interaction, f32);

    fn sample_si(&self, si: &Interaction, u: Point2f) -> (Interaction, f32) {
        let (intr, mut pdf) = self.sample(u);
        let mut wi = intr.p - si.p;
        if wi.length_sq() == 0.0 {
            pdf = 0.0;
        } else {
            wi = wi.normalize();
            pdf *= utils::distance_squared(&si.p, &intr.p) / intr.n.dot(&-wi).abs();
            if pdf.is_infinite() {
                pdf = 0.0;
            }
        }

        (intr, pdf)
    }

    fn pdf(&self, _si: &Interaction) -> f32 {
        1.0 / self.area()
    }

    fn pdf_wi(&self, si: &Interaction, wi: &Vector3f) -> f32 {
        let ray = si.spawn_ray(wi);

        if let Some((isect_light, _t_hit)) = self.intersect(&ray) {
            utils::distance_squared(&si.p, &isect_light.hit.p)
                / (isect_light.hit.n.dot(&(-(*wi))).abs() * self.area())
        } else {
            0.0
        }
    }

    fn reverse_orientation(&self) -> bool;

    fn transform_swaps_handedness(&self) -> bool;
}
