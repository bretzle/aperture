#![allow(clippy::too_many_arguments, clippy::type_complexity)]

#[macro_use]
extern crate enum_dispatch;

pub mod bxdf;
pub mod exec;
pub mod film;
pub mod geometry;
pub mod integrator;
pub mod light;
pub mod linalg;
pub mod material;
pub mod mc;
pub mod partition;
pub mod sampler;
pub mod scene;
pub mod texture;
