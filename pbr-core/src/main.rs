#![allow(clippy::many_single_char_names, non_snake_case)]

use std::mem::size_of;

use efloat::EFloat;

mod blockedarray;
mod bsdf;
mod bvh;
mod bounds;
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
    let c = dbg!(a / b);
}
