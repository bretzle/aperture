//! The integrator module defines the Integrator trait implemented by
//! the various surface integrators used to render the scene with different
//! integration methods, eg. path tracing, photon mapping etc.
//!
//! # Scene Usage Example
//! The integrator will be specified within the root object of the scene. A type
//! for the integrator along with any additional parameters is required within
//! the integrator object.
//!
//! ```json
//! "integrator": {
//!     "type": "The_Integrator_Type",
//!     ...
//! }
//! ```

use enum_set::EnumSet;
use light_arena::Allocator;
use rand::StdRng;
use std::cmp;
use std::f32;

use crate::bxdf::{BxDFType, BSDF};
use crate::film::Colorf;
use crate::geometry::{Emitter, Instance, Intersection};
use crate::light::Light;
use crate::linalg::{self, Point, Ray, Vector};
use crate::mc;
use crate::sampler::{Sample, Sampler};
use crate::scene::Scene;

pub use self::normals_debug::NormalsDebug;
pub use self::path::Path;
pub use self::whitted::Whitted;

pub mod normals_debug;
pub mod path;
pub mod whitted;

/// Trait implemented by the various integration methods that can be used to render
/// the scene. For scene usage information see whitted and path to get information
/// on how to specify them.
pub trait Integrator {
    /// Compute the illumination at the intersection in the scene
    fn illumination(
        &self,
        scene: &Scene,
        light_list: &[&Emitter],
        ray: &Ray,
        hit: &Intersection,
        sampler: &mut dyn Sampler,
        rng: &mut StdRng,
        alloc: &Allocator,
    ) -> Colorf;

    /// Compute the color of specularly reflecting light off the intersection
    fn specular_reflection(
        &self,
        scene: &Scene,
        light_list: &[&Emitter],
        ray: &Ray,
        bsdf: &BSDF,
        sampler: &mut dyn Sampler,
        rng: &mut StdRng,
        alloc: &Allocator,
    ) -> Colorf {
        let w_o = -ray.d;
        let mut spec_refl = EnumSet::new();
        spec_refl.insert(BxDFType::Specular);
        spec_refl.insert(BxDFType::Reflection);
        let mut sample_2d = [(0.0, 0.0)];
        let mut sample_1d = [0.0];
        sampler.get_samples_2d(&mut sample_2d[..], rng);
        sampler.get_samples_1d(&mut sample_1d[..], rng);
        let sample = Sample::new(&sample_2d[0], sample_1d[0]);
        let (f, w_i, pdf, _) = bsdf.sample(&w_o, spec_refl, &sample);
        let mut refl = Colorf::broadcast(0.0);
        if pdf > 0.0 && !f.is_black() && f32::abs(linalg::dot(&w_i, &bsdf.n)) != 0.0 {
            let mut refl_ray = ray.child(&bsdf.p, &w_i);
            refl_ray.min_t = 0.001;
            if let Some(hit) = scene.intersect(&mut refl_ray) {
                let li = self.illumination(scene, light_list, &refl_ray, &hit, sampler, rng, alloc);
                refl = f * li * f32::abs(linalg::dot(&w_i, &bsdf.n)) / pdf;
            }
        }
        refl
    }

    /// Compute the color of specularly transmitted light through the intersection
    fn specular_transmission(
        &self,
        scene: &Scene,
        light_list: &[&Emitter],
        ray: &Ray,
        bsdf: &BSDF,
        sampler: &mut dyn Sampler,
        rng: &mut StdRng,
        alloc: &Allocator,
    ) -> Colorf {
        let w_o = -ray.d;
        let mut spec_trans = EnumSet::new();
        spec_trans.insert(BxDFType::Specular);
        spec_trans.insert(BxDFType::Transmission);
        let mut sample_2d = [(0.0, 0.0)];
        let mut sample_1d = [0.0];
        sampler.get_samples_2d(&mut sample_2d[..], rng);
        sampler.get_samples_1d(&mut sample_1d[..], rng);
        let sample = Sample::new(&sample_2d[0], sample_1d[0]);
        let (f, w_i, pdf, _) = bsdf.sample(&w_o, spec_trans, &sample);
        let mut transmit = Colorf::broadcast(0.0);
        if pdf > 0.0 && !f.is_black() && f32::abs(linalg::dot(&w_i, &bsdf.n)) != 0.0 {
            let mut trans_ray = ray.child(&bsdf.p, &w_i);
            trans_ray.min_t = 0.001;
            if let Some(hit) = scene.intersect(&mut trans_ray) {
                let li =
                    self.illumination(scene, light_list, &trans_ray, &hit, sampler, rng, alloc);
                transmit = f * li * f32::abs(linalg::dot(&w_i, &bsdf.n)) / pdf;
            }
        }
        transmit
    }

    /// Uniformly sample the contribution of a randomly chosen light in the scene
    /// to the illumination of this BSDF at the point
    ///
    /// - `w_o` outgoing direction of the light that is incident from the light being
    ///         sampled and reflecting off the surface
    /// - `bsdf` surface properties of the surface being illuminated
    /// - `light_sample` 3 random samples for the light
    /// - `bsdf_sample` 3 random samples for the bsdf
    fn sample_one_light(
        &self,
        scene: &Scene,
        light_list: &[&Emitter],
        w_o: &Vector,
        p: &Point,
        bsdf: &BSDF,
        light_sample: &Sample,
        bsdf_sample: &Sample,
        time: f32,
    ) -> Colorf {
        let l = cmp::min(
            (light_sample.one_d * light_list.len() as f32) as usize,
            light_list.len() - 1,
        );
        self.estimate_direct(
            scene,
            w_o,
            p,
            bsdf,
            light_sample,
            bsdf_sample,
            light_list[l],
            BxDFType::non_specular(),
            time,
        )
    }

    /// Estimate the direct light contribution to the surface being shaded by the light
    /// using multiple importance sampling
    ///
    /// - `w_o` outgoing direction of the light that is incident from the light being
    ///         sampled and reflecting off the surface
    /// - `bsdf` surface properties of the surface being illuminated
    /// - `light_sample` 3 random samples for the light
    /// - `bsdf_sample` 3 random samples for the bsdf
    /// - `light` light to sample contribution from
    /// - `flags` flags for which BxDF types to sample
    fn estimate_direct(
        &self,
        scene: &Scene,
        w_o: &Vector,
        p: &Point,
        bsdf: &BSDF,
        light_sample: &Sample,
        bsdf_sample: &Sample,
        light: &dyn Light,
        flags: EnumSet<BxDFType>,
        time: f32,
    ) -> Colorf {
        let mut direct_light = Colorf::black();
        // Sample the light first
        let (li, w_i, pdf_light, occlusion) =
            light.sample_incident(&bsdf.p, &light_sample.two_d, time);
        if pdf_light > 0.0 && !li.is_black() && !occlusion.occluded(scene) {
            let f = bsdf.eval(w_o, &w_i, flags);
            if !f.is_black() {
                if light.delta_light() {
                    direct_light = f * li * f32::abs(linalg::dot(&w_i, &bsdf.n)) / pdf_light;
                } else {
                    let pdf_bsdf = bsdf.pdf(w_o, &w_i, flags);
                    let w = mc::power_heuristic(1.0, pdf_light, 1.0, pdf_bsdf);
                    direct_light = f * li * f32::abs(linalg::dot(&w_i, &bsdf.n)) * w / pdf_light;
                }
            }
        }
        // Now sample the BSDF
        if !light.delta_light() {
            let (f, w_i, pdf_bsdf, sampled_type) = bsdf.sample(w_o, flags, bsdf_sample);
            if pdf_bsdf > 0.0 && !f.is_black() {
                // Handle delta distributions the same way we did for the light
                let w = if !sampled_type.contains(&BxDFType::Specular) {
                    let pdf_light = light.pdf(p, &w_i, time);
                    if pdf_light == 0.0 {
                        return direct_light;
                    }
                    mc::power_heuristic(1.0, pdf_bsdf, 1.0, pdf_light)
                } else {
                    1.0
                };
                // Find out if the ray along w_i actually hits the light source
                let mut ray = Ray::segment(p, &w_i, 0.001, f32::INFINITY, time);
                let mut li = Colorf::black();
                if let Some(h) = scene.intersect(&mut ray) {
                    if let Instance::Emitter(ref e) = *h.instance {
                        // FIXME
                        #[allow(clippy::vtable_address_comparisons)]
                        if std::ptr::eq(e as *const dyn Light, light as *const dyn Light) {
                            li = e.radiance(&-w_i, &h.dg.p, &h.dg.ng, time)
                        }
                    }
                }
                if !li.is_black() {
                    direct_light =
                        direct_light + f * li * f32::abs(linalg::dot(&w_i, &bsdf.n)) * w / pdf_bsdf;
                }
            }
        }
        direct_light
    }
}
