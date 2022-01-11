use super::{Point, Vector};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Ray {
    /// Origin of the ray
    pub o: Point,
    /// Direction the ray is heading
    pub d: Vector,
    /// Point along the ray that the actual ray starts at, `p = o + min_t * d`
    pub min_t: f32,
    /// Point along the ray at which it stops, will be inf if the ray is infinite
    pub max_t: f32,
    /// Recursion depth of the ray
    pub depth: u32,
    /// Time point sampled by this ray
    pub time: f32,
}

impl Ray {
    /// Create a new ray from `o` heading in `d` with infinite length
    pub const fn new(o: Point, d: Vector, time: f32) -> Self {
        Self {
            o,
            d,
            min_t: 0f32,
            max_t: f32::INFINITY,
            depth: 0,
            time,
        }
    }

    /// Create a new segment ray from `o + min_t * d` to `o + max_t * d`
    pub const fn segment(o: Point, d: Vector, min_t: f32, max_t: f32, time: f32) -> Self {
        Self {
            o,
            d,
            min_t,
            max_t,
            depth: 0,
            time,
        }
    }

    /// Create a child ray from the parent starting at `o` and heading in `d`
    pub fn child(&self, o: Point, d: Vector) -> Self {
        Self {
            o,
            d,
            min_t: 0f32,
            max_t: f32::INFINITY,
            depth: self.depth + 1,
            time: self.time,
        }
    }

    /// Create a child ray segment from `o + min_t * d` to `o + max_t * d`
    pub fn child_segment(&self, o: Point, d: Vector, min_t: f32, max_t: f32) -> Self {
        Self {
            o,
            d,
            min_t,
            max_t,
            depth: self.depth + 1,
            time: self.time,
        }
    }

    /// Evaulate the ray equation at some t value and return the point
    /// returns result of `self.o + t * self.d`
    pub fn at(&self, t: f32) -> Point {
        self.o + self.d * t
    }
}
