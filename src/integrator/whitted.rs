//! Defines the Whitted integrator which implements Whitted recursive ray tracing
//! See [Whitted, An improved illumination model for shaded display](http://dl.acm.org/citation.cfm?id=358882)
//!
//! # Scene Usage Example
//! The Whitted integrator just needs a maximum ray depth to terminate specular reflection
//! and transmission rays.
//!
//! ```json
//! "integrator": {
//!     "type": "whitted",
//!     "max_depth": 8
//! }
//! ```

use crate::{
    bxdf::BxDFType,
    film::Colorf,
    geometry::{Emitter, Instance, Intersection},
    integrator::{Integrator, Integrators},
    light::Light,
    linalg::{self, Ray},
    material::Material,
    sampler::{Sampler, Samplers},
    scene::Scene,
};
use light_arena::Allocator;
use rand::StdRng;
use std::f32;

/// The Whitted integrator implementing the Whitted recursive ray tracing algorithm
#[derive(Clone, Copy, Debug)]
pub struct Whitted {
    /// The maximum recursion depth for rays
    max_depth: u32,
}

impl Whitted {
    /// Create a new Whitted integrator with the desired maximum recursion depth for rays
    pub fn new_integrator(max_depth: u32) -> Integrators {
        Integrators::Whitted(Self { max_depth })
    }
}

impl Integrator for Whitted {
    fn illumination(
        &self,
        scene: &Scene,
        light_list: &[&Emitter],
        ray: &Ray,
        hit: &Intersection,
        sampler: &mut Samplers,
        rng: &mut StdRng,
        alloc: &Allocator,
    ) -> Colorf {
        let bsdf = hit.material.bsdf(hit, alloc);
        let w_o = -ray.d;
        let mut sample_2d = [(0.0, 0.0)];
        sampler.get_samples_2d(&mut sample_2d[..], rng);
        let mut illum = Colorf::broadcast(0.0);
        if ray.depth == 0 {
            if let Instance::Emitter(ref e) = *hit.instance {
                let w = -ray.d;
                illum = illum + e.radiance(&w, &hit.dg.p, &hit.dg.ng, ray.time);
            }
        }

        for light in light_list {
            let (li, w_i, pdf, occlusion) =
                light.sample_incident(&hit.dg.p, &sample_2d[0], ray.time);
            let f = bsdf.eval(&w_o, &w_i, BxDFType::all());
            if !li.is_black() && !f.is_black() && !occlusion.occluded(scene) {
                illum = illum + f * li * f32::abs(linalg::dot(&w_i, &bsdf.n)) / pdf;
            }
        }
        if ray.depth < self.max_depth {
            illum = illum
                + self.specular_reflection(scene, light_list, ray, &bsdf, sampler, rng, alloc);
            illum = illum
                + self.specular_transmission(scene, light_list, ray, &bsdf, sampler, rng, alloc);
        }
        illum
    }
}
