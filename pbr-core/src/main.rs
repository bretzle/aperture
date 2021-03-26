#![allow(
    clippy::many_single_char_names,
    non_snake_case,
    dead_code,
    clippy::excessive_precision,
    clippy::too_many_arguments,
    clippy::suspicious_operation_groupings // <-- maybe bugs?
)]

use efloat::EFloat;

mod blockedarray;
mod bounds;
mod bsdf;
mod bvh;
mod camera;
mod cie;
mod efloat;
mod film;
mod filter;
mod interaction;
mod interpolation;
mod material;
mod mipmap;
mod paramset;
mod primitive;
mod ray;
mod sampler;
mod sampling;
mod shapes;
mod spectrum;
mod texture;
mod transform;
mod utils;

fn main() {
    println!("Hello, world!");

    let a = dbg!(EFloat::from(1.0));
    let b = dbg!(EFloat::from(5.0));
    let _c = dbg!(a / b);
}
