#![allow(clippy::enum_variant_names)]
#![forbid(unsafe_code)]

#[macro_use]
extern crate log;

pub mod math;
pub mod parser;
pub mod quaternion;
pub mod transform;
pub mod interaction;
pub mod medium;
pub mod shapes;