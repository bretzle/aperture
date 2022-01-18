//! Defines the BxDF interface implemented by BRDF/BTDFs to describe
//! material properties. Also provides the BSDF type which composes
//! various BRDF/BTDFs to describe materials

use enum_set::{CLike, EnumSet};
use std::{f32, mem};

use crate::{
    film::Colorf,
    linalg::{self, Vector},
    mc,
};

pub use self::{
    bsdf::BSDF, lambertian::Lambertian, merl::Merl,
    microfacet_transmission::MicrofacetTransmission, oren_nayar::OrenNayar,
    specular_reflection::SpecularReflection, specular_transmission::SpecularTransmission,
    torrance_sparrow::TorranceSparrow,
};

pub mod bsdf;
pub mod fresnel;
pub mod lambertian;
pub mod merl;
pub mod microfacet;
pub mod microfacet_transmission;
pub mod oren_nayar;
pub mod specular_reflection;
pub mod specular_transmission;
pub mod torrance_sparrow;

/// Various types of BxDFs that can be selected to specify which
/// types of surface functions should be evaluated
#[repr(u32)]
#[derive(Clone, Copy, Debug)]
pub enum BxDFType {
    Reflection,
    Transmission,
    Diffuse,
    Glossy,
    Specular,
}

impl BxDFType {
    /// Get an EnumSet containing all flags for the different types of
    /// BxDFs: Diffuse, Glossy, Specular
    pub fn all_types() -> EnumSet<BxDFType> {
        let mut e = EnumSet::new();
        e.insert(BxDFType::Diffuse);
        e.insert(BxDFType::Glossy);
        e.insert(BxDFType::Specular);
        e
    }
    /// Get an EnumSet containing all flags for reflective BxDFs (eg. BRDFs)
    pub fn all_brdf() -> EnumSet<BxDFType> {
        let mut e = BxDFType::all_types();
        e.insert(BxDFType::Reflection);
        e
    }
    /// Get an EnumSet containing all flags for transmissive BxDFs (eg. BTDFs)
    pub fn all_btdf() -> EnumSet<BxDFType> {
        let mut e = BxDFType::all_types();
        e.insert(BxDFType::Transmission);
        e
    }
    /// Get an EnumSet containing all flags for all BxDFs. This would be all
    /// types of BRDFs and BTDFs
    pub fn all() -> EnumSet<BxDFType> {
        BxDFType::all_brdf().union(BxDFType::all_btdf())
    }
    /// Get an EnumSet containing flags for all types of specular BxDFs
    pub fn specular() -> EnumSet<BxDFType> {
        let mut e = EnumSet::new();
        e.insert(BxDFType::Specular);
        e.insert(BxDFType::Reflection);
        e.insert(BxDFType::Transmission);
        e
    }
    /// Get an EnumSet containing flags for all non-specular BxDFs
    pub fn non_specular() -> EnumSet<BxDFType> {
        let mut e = EnumSet::new();
        e.insert(BxDFType::Diffuse);
        e.insert(BxDFType::Glossy);
        e.insert(BxDFType::Reflection);
        e.insert(BxDFType::Transmission);
        e
    }
}

impl CLike for BxDFType {
    fn to_u32(&self) -> u32 {
        *self as u32
    }
    unsafe fn from_u32(v: u32) -> BxDFType {
        mem::transmute(v)
    }
}

/// Trait implemented by BRDF/BTDFs in `tray_rust`. Provides methods for
/// evaluating and sampling the function
#[enum_dispatch(BxDFs)]
pub trait BxDF {
    /// Get the type of this BxDF
    fn bxdf_type(&self) -> EnumSet<BxDFType>;
    /// Evaluate the BxDF for the pair of incident and outgoing light directions,
    /// `w_i` and `w_o`.
    fn eval(&self, w_o: &Vector, w_i: &Vector) -> Colorf;
    /// Sample an incident light direction for an outgoing light direction `w_o`.
    /// `samples` will be used to randomly sample a direction for the outgoing light
    /// Returns the color of the material for the pair of directions, the incident
    /// light direction and pdf
    fn sample(&self, w_o: &Vector, samples: &(f32, f32)) -> (Colorf, Vector, f32) {
        let mut w_i = mc::cos_sample_hemisphere(samples);
        // We may need to flip the sampled direction to be on the same hemisphere as w_o
        if w_o.z < 0.0 {
            w_i.z *= -1.0;
        }
        (self.eval(w_o, &w_i), w_i, self.pdf(w_o, &w_i))
    }
    /// Check if this BxDF matches the type flags passed
    fn matches(&self, flags: EnumSet<BxDFType>) -> bool {
        self.bxdf_type().is_subset(&flags)
    }
    /// Compute the pdf of sampling the pair of directions passed for this BxDF
    fn pdf(&self, w_o: &Vector, w_i: &Vector) -> f32 {
        if same_hemisphere(w_o, w_i) {
            f32::abs(cos_theta(w_i)) * f32::consts::FRAC_1_PI
        } else {
            0.0
        }
    }
}

#[enum_dispatch]
#[derive(Copy, Clone)]
pub enum BxDFs<'a> {
    Merl(Merl<'a>),
    Lambertian,
    OrenNayar,
    MicrofacetTransmission(MicrofacetTransmission<'a>),
    SpecularReflection(SpecularReflection<'a>),
    SpecularTransmission(SpecularTransmission<'a>),
    TorranceSparrow(TorranceSparrow<'a>),
}

/// Compute the value of cosine theta for a vector in shading space
pub fn cos_theta(v: &Vector) -> f32 {
    v.z
}
/// Compute the value of cosine^2 theta for a vector in shading space
pub fn cos_theta_sqr(v: &Vector) -> f32 {
    v.z * v.z
}
/// Compute the value of (sine theta)^2  for a vector in shading space
pub fn sin_theta_sqr(v: &Vector) -> f32 {
    f32::max(0.0, 1.0 - v.z * v.z)
}
/// Compute the value of sine theta for a vector in shading space
pub fn sin_theta(v: &Vector) -> f32 {
    f32::sqrt(sin_theta_sqr(v))
}
/// Compute the value of tan theta for a vector in shading space
pub fn tan_theta(v: &Vector) -> f32 {
    let sin_theta_2 = sin_theta_sqr(v);
    if sin_theta_2 <= 0.0 {
        0.0
    } else {
        f32::sqrt(sin_theta_2) / cos_theta(v)
    }
}
/// Compute the value of tan theta^2 for a vector in shading space
pub fn tan_theta_sqr(v: &Vector) -> f32 {
    sin_theta_sqr(v) / cos_theta_sqr(v)
}
/// Compute the value of arctan theta for a vector in shading space
pub fn arctan_theta(v: &Vector) -> f32 {
    cos_theta(v) / sin_theta(v)
}
/// Compute the value of cosine phi for a vector in shading space
pub fn cos_phi(v: &Vector) -> f32 {
    let sin_theta = sin_theta(v);
    if sin_theta == 0.0 {
        1.0
    } else {
        linalg::clamp(v.x / sin_theta, -1.0, 1.0)
    }
}
/// Compute the value of sine phi for a vector in shading space
pub fn sin_phi(v: &Vector) -> f32 {
    let sin_theta = sin_theta(v);
    if sin_theta == 0.0 {
        0.0
    } else {
        linalg::clamp(v.y / sin_theta, -1.0, 1.0)
    }
}
/// Check if two vectors are in the same hemisphere in shading space
pub fn same_hemisphere(a: &Vector, b: &Vector) -> bool {
    a.z * b.z > 0.0
}
