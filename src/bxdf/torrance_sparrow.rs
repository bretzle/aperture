//! This module provides the Torrance Sparrow microfacet BRDF, see
//! [Walter et al. 07](https://www.cs.cornell.edu/~srm/publications/EGSR07-btdf.pdf)
//! for details.

use crate::{
    bxdf::{
        self,
        fresnel::{Fresnel, Fresnels},
        microfacet::{MicrofacetDistribution, MicrofacetDistributions},
        BxDF, BxDFType, BxDFs,
    },
    film::Colorf,
    linalg::{self, Vector},
};
use enum_set::EnumSet;
use std::f32;

/// Struct providing the Torrance Sparrow BRDF, implemented as described in
/// [Walter et al. 07](https://www.cs.cornell.edu/~srm/publications/EGSR07-btdf.pdf)
#[derive(Copy, Clone)]
pub struct TorranceSparrow<'a> {
    reflectance: Colorf,
    fresnel: &'a Fresnels,
    /// Microfacet distribution describing the structure of the microfacets of
    /// the material
    microfacet: &'a MicrofacetDistributions,
}

impl<'a> TorranceSparrow<'a> {
    /// Create a new Torrance Sparrow microfacet BRDF
    pub fn new(c: &Colorf, fresnel: &'a Fresnels, microfacet: &'a MicrofacetDistributions) -> Self {
        Self {
            reflectance: *c,
            fresnel,
            microfacet,
        }
    }

    pub fn new_bxdf(
        c: &Colorf,
        fresnel: &'a Fresnels,
        microfacet: &'a MicrofacetDistributions,
    ) -> BxDFs<'a> {
        BxDFs::TorranceSparrow(Self::new(c, fresnel, microfacet))
    }
}

impl<'a> BxDF for TorranceSparrow<'a> {
    fn bxdf_type(&self) -> EnumSet<BxDFType> {
        let mut e = EnumSet::new();
        e.insert(BxDFType::Glossy);
        e.insert(BxDFType::Reflection);
        e
    }

    fn eval(&self, w_o: &Vector, w_i: &Vector) -> Colorf {
        let cos_to = f32::abs(bxdf::cos_theta(w_o));
        let cos_ti = f32::abs(bxdf::cos_theta(w_i));
        if cos_to == 0.0 || cos_ti == 0.0 {
            return Colorf::new(0.0, 0.0, 0.0);
        }
        let mut w_h = *w_i + *w_o;
        if w_h == Vector::broadcast(0.0) {
            return Colorf::new(0.0, 0.0, 0.0);
        }
        w_h = w_h.normalized();
        let d = self.microfacet.normal_distribution(&w_h);
        let f = self.fresnel.fresnel(linalg::dot(w_i, &w_h));
        let g = self.microfacet.shadowing_masking(w_i, w_o, &w_h);
        self.reflectance * f * d * g / (4.0 * cos_ti * cos_to)
    }

    fn sample(&self, w_o: &Vector, samples: &(f32, f32)) -> (Colorf, Vector, f32) {
        if w_o.z == 0.0 {
            return (Colorf::black(), Vector::broadcast(0.0), 0.0);
        }
        let mut w_h = self.microfacet.sample(w_o, samples);
        if !bxdf::same_hemisphere(w_o, &w_h) {
            w_h = -w_h;
        }
        let w_i = linalg::reflect(w_o, &w_h);
        if !bxdf::same_hemisphere(w_o, &w_i) {
            (Colorf::black(), Vector::broadcast(0.0), 0.0)
        } else {
            (self.eval(w_o, &w_i), w_i, self.pdf(w_o, &w_i))
        }
    }

    fn pdf(&self, w_o: &Vector, w_i: &Vector) -> f32 {
        if !bxdf::same_hemisphere(w_o, w_i) {
            0.0
        } else {
            let w_h = (*w_o + *w_i).normalized();
            // This term is p_o(o) in eq. 38 of Walter et al's 07 paper and is for reflection so
            // we use the Jacobian for reflection, eq. 14
            let jacobian = 1.0 / (4.0 * f32::abs(linalg::dot(w_o, &w_h)));
            self.microfacet.pdf(&w_h) * jacobian
        }
    }
}
