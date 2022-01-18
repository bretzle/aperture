//! Defines the Path integrator which implements path tracing with
//! explicit light sampling
//!
//! See [Kajiya, The Rendering Equation](http://dl.acm.org/citation.cfm?id=15902)
//!
//! # Scene Usage Example
//! The pathtracer integrator needs a maximum ray depth to terminate rays at and
//! a minimum ray depth to start applying Russian Roulette to terminate rays early.
//!
//! ```json
//! "integrator": {
//!     "type": "pathtracer",
//!     "min_depth": 3,
//!     "max_depth": 8
//! }
//! ```

use crate::{
    bxdf::BxDFType,
    film::Colorf,
    geometry::{Emitter, Instance, Intersection},
    integrator::Integrator,
    linalg::{self, Ray},
    material::Material,
    sampler::{Sample, Sampler, Samplers},
    scene::Scene,
};
use light_arena::Allocator;
use rand::{Rng, StdRng};
use std::f32;

use super::Integrators;

/// The path integrator implementing Path tracing with explicit light sampling
#[derive(Clone, Copy, Debug)]
pub struct Path {
    min_depth: usize,
    max_depth: usize,
}

impl Path {
    /// Create a new path integrator with the min and max length desired for paths
    pub fn new(min_depth: u32, max_depth: u32) -> Integrators {
        Integrators::Path(Path {
            min_depth: min_depth as usize,
            max_depth: max_depth as usize,
        })
    }
}

impl Integrator for Path {
    fn illumination(
        &self,
        scene: &Scene,
        light_list: &[&Emitter],
        r: &Ray,
        hit: &Intersection,
        sampler: &mut Samplers,
        rng: &mut StdRng,
        alloc: &Allocator,
    ) -> Colorf {
        let num_samples = self.max_depth as usize + 1;
        let l_samples = alloc.alloc_slice::<(f32, f32)>(num_samples);
        let l_samples_comp = alloc.alloc_slice::<f32>(num_samples);
        let bsdf_samples = alloc.alloc_slice::<(f32, f32)>(num_samples);
        let bsdf_samples_comp = alloc.alloc_slice::<f32>(num_samples);
        let path_samples = alloc.alloc_slice::<(f32, f32)>(num_samples);
        let path_samples_comp = alloc.alloc_slice::<f32>(num_samples);
        sampler.get_samples_2d(l_samples, rng);
        sampler.get_samples_2d(bsdf_samples, rng);
        sampler.get_samples_2d(path_samples, rng);
        sampler.get_samples_1d(l_samples_comp, rng);
        sampler.get_samples_1d(bsdf_samples_comp, rng);
        sampler.get_samples_1d(path_samples_comp, rng);

        let mut illum = Colorf::black();
        let mut path_throughput = Colorf::broadcast(1.0);
        // Track if the previous bounce was a specular one
        let mut specular_bounce = false;
        let mut current_hit = *hit;
        let mut ray = *r;
        let mut bounce = 0;
        loop {
            if bounce == 0 || specular_bounce {
                if let Instance::Emitter(ref e) = *current_hit.instance {
                    let w = -ray.d;
                    illum =
                        illum + path_throughput * e.radiance(&w, &hit.dg.p, &hit.dg.ng, ray.time);
                }
            }
            let bsdf = current_hit.material.bsdf(&current_hit, alloc);
            let w_o = -ray.d;
            let light_sample = Sample::new(&l_samples[bounce], l_samples_comp[bounce]);
            let bsdf_sample = Sample::new(&bsdf_samples[bounce], bsdf_samples_comp[bounce]);
            let li = self.sample_one_light(
                scene,
                light_list,
                &w_o,
                &current_hit.dg.p,
                &bsdf,
                &light_sample,
                &bsdf_sample,
                ray.time,
            );
            illum = illum + path_throughput * li;

            // Determine the next direction to take the path by sampling the BSDF
            let path_sample = Sample::new(&path_samples[bounce], path_samples_comp[bounce]);
            let (f, w_i, pdf, sampled_type) = bsdf.sample(&w_o, BxDFType::all(), &path_sample);
            if f.is_black() || pdf == 0.0 {
                break;
            }
            specular_bounce = sampled_type.contains(&BxDFType::Specular);
            path_throughput = path_throughput * f * f32::abs(linalg::dot(&w_i, &bsdf.n)) / pdf;

            // Check if we're beyond the min depth at which point we start trying to
            // terminate rays using Russian Roulette
            // TODO: Am I re-weighting properly? The Russian roulette results don't look quite as
            // nice, eg. damping light in transparent objects and such.
            if bounce > self.min_depth {
                let cont_prob = f32::max(0.5, path_throughput.luminance());
                if rng.next_f32() > cont_prob {
                    break;
                }
                // Re-weight the sum terms accordingly with the Russian roulette weight
                path_throughput = path_throughput / cont_prob;
            }
            if bounce == self.max_depth {
                break;
            }

            ray = ray.child(&bsdf.p, &w_i.normalized());
            ray.min_t = 0.001;
            // Find the next vertex on the path
            match scene.intersect(&mut ray) {
                Some(h) => current_hit = h,
                None => break,
            }
            bounce += 1;
        }
        illum
    }
}
