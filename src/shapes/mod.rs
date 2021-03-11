use std::fmt::Debug;

pub trait Shape: Debug + Send + Sync {
    fn reverse_orientation(&self) -> bool;

    fn transform_swaps_handedness(&self) -> bool;
}
