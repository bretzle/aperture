use maths::*;
use pbr_core::efloat::EFloat;

fn main() {
    println!("Hello, world!");

    let a = dbg!(EFloat::from(1.0));
    let b = dbg!(EFloat::from(5.0));
    let _c = dbg!(a / b);

    let mat = matrix![
        1.0, 2.0;
        3.0, 4.0;
        5.0, 6.0;
    ];

    let mata = dbg!(mat.clone());
    let matb = dbg!(mat.clone());

    dbg!(Matrix::add(&mata, &matb));
}
