#![allow(
    clippy::many_single_char_names,
    non_snake_case,
    clippy::excessive_precision,
    clippy::too_many_arguments,
    clippy::suspicious_operation_groupings // <-- maybe bugs?
)]

pub mod blockedarray;
pub mod bounds;
pub mod bsdf;
pub mod bvh;
pub mod camera;
pub mod cie;
pub mod efloat;
pub mod film;
pub mod filter;
pub mod interaction;
pub mod interpolation;
pub mod material;
pub mod mipmap;
pub mod paramset;
pub mod primitive;
pub mod ray;
pub mod sampler;
pub mod sampling;
pub mod shapes;
pub mod spectrum;
pub mod texture;
pub mod transform;
pub mod utils;
