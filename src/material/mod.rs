//! Defines the trait implemented by all materials and exports various
//! supported material types. Materials are used to define how BxDFs are
//! composed to create the desired appearance
//!
//! # Scene Usage Example
//! The material will be specified within the materials list of the scene object. A type
//! and name for the material along with any additional parameters is required to specify one.
//! The name is used when specifying which material should be used by an object in the scene.
//!
//! ```json
//! "materials": [
//!     {
//!         "name": "my_material",
//!         "type": "The_Material_Type",
//!          ...
//!     }
//!     ...
//! ]
//! ```

pub use self::{
    glass::Glass, matte::Matte, merl::Merl, metal::Metal, plastic::Plastic,
    rough_glass::RoughGlass, specular_metal::SpecularMetal,
};
use crate::{bxdf::BSDF, geometry::Intersection};
use light_arena::Allocator;

pub mod glass;
pub mod matte;
pub mod merl;
pub mod metal;
pub mod plastic;
pub mod rough_glass;
pub mod specular_metal;

/// Trait implemented by materials. Provides method to get the BSDF describing
/// the material properties at the intersection
#[enum_dispatch(Materials)]
pub trait Material {
    /// Get the BSDF for the material which defines its properties at the hit point.
    ///
    /// We have the lifetime constraint on the returned BSDF to enforce it does not
    /// outlive the material which produced it. This allows us to borrow things from
    /// the parent material in the BxDFs making up the BSDF.
    fn bsdf<'a, 'b, 'c>(&'a self, hit: &Intersection<'a, 'b>, alloc: &'c Allocator) -> BSDF<'c>
    where
        'a: 'c;
}

#[enum_dispatch]
pub enum Materials {
    Glass,
    Matte,
    Merl,
    Metal,
    Plastic,
    RoughGlass,
    SpecularMetal,
}
