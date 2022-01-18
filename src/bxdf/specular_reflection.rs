//! Defines a BRDF that describes specular reflection

use crate::{
    bxdf::{
        self,
        fresnel::{Fresnel, Fresnels},
        BxDF, BxDFType, BxDFs,
    },
    film::Colorf,
    linalg::Vector,
};
use enum_set::EnumSet;
use std::f32;

/// Specular reflection BRDF that implements a specularly reflective material model
#[derive(Copy, Clone)]
pub struct SpecularReflection<'a> {
    /// Color of the reflective material
    reflectance: Colorf,
    /// Fresnel term for the reflection model
    fresnel: &'a Fresnels,
}

impl<'a> SpecularReflection<'a> {
    /// Create a specularly reflective BRDF with the reflective color and Fresnel term
    pub fn new(c: &Colorf, fresnel: &'a Fresnels) -> Self {
        Self {
            reflectance: *c,
            fresnel,
        }
    }

    pub fn new_bxdf(c: &Colorf, fresnel: &'a Fresnels) -> BxDFs<'a> {
        BxDFs::SpecularReflection(Self::new(c, fresnel))
    }
}

impl<'a> BxDF for SpecularReflection<'a> {
    fn bxdf_type(&self) -> EnumSet<BxDFType> {
        let mut e = EnumSet::new();
        e.insert(BxDFType::Specular);
        e.insert(BxDFType::Reflection);
        e
    }

    /// We'll never exactly hit the specular reflection direction with some pair
    /// so this just returns black. Use `sample` instead
    fn eval(&self, _: &Vector, _: &Vector) -> Colorf {
        Colorf::broadcast(0.0)
    }

    /// Sampling the specular BRDF just returns the specular reflection direction
    /// for the light leaving along `w_o`
    fn sample(&self, w_o: &Vector, _: &(f32, f32)) -> (Colorf, Vector, f32) {
        let w_i = Vector::new(-w_o.x, -w_o.y, w_o.z);
        // TODO: is this an expected but super rare case? or does it imply some error
        // in the sphere intersection? Such a glancing angle shouldn't really be counted right?
        if w_i.z != 0.0 {
            let c = self.fresnel.fresnel(bxdf::cos_theta(w_o)) * self.reflectance
                / f32::abs(bxdf::cos_theta(&w_i));
            (c, w_i, 1.0)
        } else {
            (Colorf::black(), w_i, 0.0)
        }
    }
}
