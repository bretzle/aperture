//! Defines a rough glass material
//!
//! # Scene Usage Example
//! The rough glass material describes a thin glass surface material,
//! not a solid block of glass (there is no absorption of light). The glass requires
//! a reflective and emissive color along with a refrective index, eta and roughness.
//!
//! ```json
//! "materials": [
//!     {
//!         "name": "foggy_glass",
//!         "type": "rough_glass",
//!         "reflect": [1, 1, 1],
//!         "transmit": [1, 1, 1],
//!         "eta": 1.52,
//!         "roughness": 0.5,
//!     },
//!     ...
//! ]
//! ```

use std::sync::Arc;

use light_arena::Allocator;

use crate::bxdf::fresnel::Dielectric;
use crate::bxdf::microfacet::Beckmann;
use crate::bxdf::{BxDF, MicrofacetTransmission, TorranceSparrow, BSDF};
use crate::geometry::Intersection;
use crate::material::Material;
use crate::texture::Texture;

/// The `RoughGlass` material describes specularly transmissive and reflective glass material
pub struct RoughGlass {
    reflect: Arc<dyn Texture + Send + Sync>,
    transmit: Arc<dyn Texture + Send + Sync>,
    eta: Arc<dyn Texture + Send + Sync>,
    roughness: Arc<dyn Texture + Send + Sync>,
}

impl RoughGlass {
    /// Create the `RoughGlass` material with the desired color and index of refraction
    /// `reflect`: color of reflected light
    /// `transmit`: color of transmitted light
    /// `eta`: refractive index of the material
    /// `roughness`: roughness of the material
    pub fn new(
        reflect: Arc<dyn Texture + Send + Sync>,
        transmit: Arc<dyn Texture + Send + Sync>,
        eta: Arc<dyn Texture + Send + Sync>,
        roughness: Arc<dyn Texture + Send + Sync>,
    ) -> Self {
        Self {
            reflect,
            transmit,
            eta,
            roughness,
        }
    }
}

impl Material for RoughGlass {
    fn bsdf<'a, 'b, 'c>(&self, hit: &Intersection<'a, 'b>, alloc: &'c Allocator) -> BSDF<'c>
    where
        'a: 'c,
    {
        let reflect = self.reflect.sample_color(hit.dg.u, hit.dg.v, hit.dg.time);
        let transmit = self.transmit.sample_color(hit.dg.u, hit.dg.v, hit.dg.time);
        let eta = self.eta.sample_f32(hit.dg.u, hit.dg.v, hit.dg.time);
        let roughness = self.roughness.sample_f32(hit.dg.u, hit.dg.v, hit.dg.time);

        let mut num_bxdfs = 0;
        if !reflect.is_black() {
            num_bxdfs += 1;
        }
        if !transmit.is_black() {
            num_bxdfs += 1;
        }

        let bxdfs = alloc.alloc_slice::<&dyn BxDF>(num_bxdfs);
        let mut i = 0;
        let fresnel = alloc.alloc(Dielectric::new(1.0, eta));
        let microfacet = alloc.alloc(Beckmann::new(roughness));
        if !reflect.is_black() {
            bxdfs[i] = alloc.alloc(TorranceSparrow::new(&reflect, fresnel, microfacet));
            i += 1;
        }
        if !transmit.is_black() {
            bxdfs[i] = alloc.alloc(MicrofacetTransmission::new(&transmit, fresnel, microfacet));
        }
        BSDF::new(bxdfs, eta, &hit.dg)
    }
}
