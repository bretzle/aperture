//! Defines a matte material used to describe diffuse materials
//!
//! # Scene Usage Example
//! The matte material requires a diffuse color and a roughness for the material. A roughness of 0
//! will select a [Lambertian](https://en.wikipedia.org/wiki/Lambertian_reflectance) model
//! while a roughness > 0 will select an
//! [Oren-Nayar](https://en.wikipedia.org/wiki/Oren%E2%80%93Nayar_reflectance_model)
//! reflectance model.
//!
//! ```json
//! "materials": [
//!     {
//!         "name": "purple_matte",
//!         "type": "matte",
//!         "diffuse": [1, 0, 1],
//!         "roughness" 0.5
//!     },
//!     ...
//! ]
//! ```

use crate::{
    bxdf::{BxDFs, Lambertian, OrenNayar, BSDF},
    geometry::Intersection,
    material::{Material, Materials},
    texture::{Texture, Textures},
};
use light_arena::Allocator;
use std::sync::Arc;

/// The Matte material describes diffuse materials with either a Lambertian or
/// Oren-Nayar BRDF. The Lambertian BRDF is used for materials with no roughness
/// while Oren-Nayar is used for those with some roughness.
pub struct Matte {
    diffuse: Arc<Textures>,
    roughness: Arc<Textures>,
}

impl Matte {
    /// Create a new Matte material with the desired diffuse color and roughness
    pub fn new_material(diffuse: Arc<Textures>, roughness: Arc<Textures>) -> Materials {
        Materials::Matte(Matte { diffuse, roughness })
    }
}

impl Material for Matte {
    fn bsdf<'a, 'b, 'c>(&'a self, hit: &Intersection<'a, 'b>, alloc: &'c Allocator) -> BSDF<'c>
    where
        'a: 'c,
    {
        let diffuse = self.diffuse.sample_color(hit.dg.u, hit.dg.v, hit.dg.time);
        let roughness = self.roughness.sample_f32(hit.dg.u, hit.dg.v, hit.dg.time);

        let bsdfs = alloc.alloc_slice::<&'c BxDFs>(1);
        if roughness == 0.0 {
            bsdfs[0] = alloc.alloc(Lambertian::new_bxdf(diffuse));
        } else {
            bsdfs[0] = alloc.alloc(OrenNayar::new_bxdf(diffuse, roughness));
        }
        BSDF::new(bsdfs, 1.0, &hit.dg)
    }
}
