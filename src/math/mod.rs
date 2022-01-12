use std::ops::Index;

pub use self::animated_transform::AnimatedTransform;
pub use self::keyframe::Keyframe;
pub use self::matrix::Matrix;
pub use self::normal::Normal;
pub use self::point::Point;
pub use self::quaternion::Quaternion;
pub use self::ray::Ray;
pub use self::transform::Transform;
pub use self::vector::Vector;

mod animated_transform;
mod keyframe;
mod matrix;
mod normal;
mod point;
mod quaternion;
mod ray;
mod transform;
mod vector;

#[derive(Debug, Clone, Copy)]
pub enum Axis {
    X,
    Y,
    Z,
}

pub fn cross<A, B>(a: &A, b: &B) -> Vector
where
    A: Index<usize, Output = f32>,
    B: Index<usize, Output = f32>,
{
    Vector::new(
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    )
}

pub fn dot<A, B>(a: &A, b: &B) -> f32
where
    A: Index<usize, Output = f32>,
    B: Index<usize, Output = f32>,
{
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

pub fn clamp<T>(x: T, min: T, max: T) -> T
where
    T: PartialOrd,
{
    if x < min {
        min
    } else if x > max {
        max
    } else {
        x
    }
}

pub fn solve_quadratic(a: f32, b: f32, c: f32) -> Option<(f32, f32)> {
    let discrim_sqr = b * b - 4.0 * a * c;
    if discrim_sqr < 0.0 {
        None
    } else {
        let discrim = f32::sqrt(discrim_sqr);
        let q = if b < 0.0 {
            -0.5 * (b - discrim)
        } else {
            -0.5 * (b + discrim)
        };
        match (q / a, c / q) {
            (x, y) if x > y => Some((y, x)),
            (x, y) => Some((x, y)),
        }
    }
}
