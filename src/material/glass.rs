//! Defines a specular glass material
//!
//! # Scene Usage Example
//! The specular glass material describes a thin glass surface type of material,
//! not a solid block of glass (there is no absorption of light). The glass requires
//! a reflective and emissive color along with a refrective index, eta.
//!
//! ```json
//! "materials": [
//!     {
//!         "name": "clear_glass",
//!         "type": "glass",
//!         "reflect": [1, 1, 1],
//!         "transmit": [1, 1, 1],
//!         "eta": 1.52
//!     },
//!     ...
//! ]
//! ```

use crate::{
    bxdf::{fresnel::Dielectric, BxDFs, SpecularReflection, SpecularTransmission, BSDF},
    geometry::Intersection,
    material::{Material, Materials},
    texture::{Texture, Textures},
};
use light_arena::Allocator;
use std::sync::Arc;

/// The Glass material describes specularly transmissive and reflective glass material
pub struct Glass {
    reflect: Arc<Textures>,
    transmit: Arc<Textures>,
    eta: Arc<Textures>,
}

impl Glass {
    /// Create the glass material with the desired color and index of refraction
    /// `reflect`: color of reflected light
    /// `transmit`: color of transmitted light
    /// `eta`: refractive index of the material
    pub fn new_material(
        reflect: Arc<Textures>,
        transmit: Arc<Textures>,
        eta: Arc<Textures>,
    ) -> Materials {
        Materials::Glass(Glass {
            reflect,
            transmit,
            eta,
        })
    }
}

impl Material for Glass {
    fn bsdf<'a, 'b, 'c>(&'a self, hit: &Intersection<'a, 'b>, alloc: &'c Allocator) -> BSDF<'c>
    where
        'a: 'c,
    {
        // TODO: I don't like this counting and junk we have to do to figure out
        // the slice size and then the indices. Is there a better way?
        let reflect = self.reflect.sample_color(hit.dg.u, hit.dg.v, hit.dg.time);
        let transmit = self.transmit.sample_color(hit.dg.u, hit.dg.v, hit.dg.time);
        let eta = self.eta.sample_f32(hit.dg.u, hit.dg.v, hit.dg.time);

        let mut num_bxdfs = 0;
        if !reflect.is_black() {
            num_bxdfs += 1;
        }
        if !transmit.is_black() {
            num_bxdfs += 1;
        }
        let bxdfs = alloc.alloc_slice::<&BxDFs>(num_bxdfs);

        let mut i = 0;
        let fresnel = alloc.alloc(Dielectric::new(1.0, eta).into());
        if !reflect.is_black() {
            bxdfs[i] = alloc.alloc(SpecularReflection::new_bxdf(&reflect, fresnel));
            i += 1;
        }
        if !transmit.is_black() {
            bxdfs[i] = alloc.alloc(SpecularTransmission::new_bxdf(&transmit, fresnel));
        }
        BSDF::new(bxdfs, eta, &hit.dg)
    }
}
