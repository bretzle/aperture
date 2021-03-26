#[macro_export]
macro_rules! approx {
    ($a:expr, == $b:expr) => {
        ($a - $b).abs() < f32::EPSILON
    };
    ($a:expr, != $b:expr) => {
        ($a - $b).abs() > f32::EPSILON
    };
}
