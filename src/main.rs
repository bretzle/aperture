#![allow(clippy::many_single_char_names, non_snake_case)]

use efloat::EFloat;

mod bsdf;
mod bvh;
mod camera;
mod cie;
mod efloat;
mod film;
mod geometry;
mod interaction;
mod material;
mod paramset;
mod primitive;
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
