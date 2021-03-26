use pbr_core::efloat::EFloat;

fn main() {
    println!("Hello, world!");

    let a = dbg!(EFloat::from(1.0));
    let b = dbg!(EFloat::from(5.0));
    let _c = dbg!(a / b);
}
