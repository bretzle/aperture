use super::{lerp, Number, Point2, Point3, Vector2, Vector3};
use num_traits::Float;

#[derive(Debug, Default, Copy, Clone)]
pub struct Bounds2<T> {
    pub p_min: Point2<T>,
    pub p_max: Point2<T>,
}

impl<T: Number> Bounds2<T> {
    pub fn new(p_min: Point2<T>, p_max: Point2<T>) -> Self {
        Self { p_min, p_max }
    }

    pub fn diagonal(self) -> Vector2<T> {
        self.p_max - self.p_min
    }

    pub fn area(self) -> T {
        let d = self.p_max - self.p_min;
        d.x * d.y
    }

    pub fn lerp(&self, t: Point2<T>) -> Point2<T>
    where
        T: Float,
    {
        Point2 {
            x: lerp(t.x, self.p_min.x, self.p_max.x),
            y: lerp(t.y, self.p_min.y, self.p_max.y),
        }
    }

    pub fn offset(&self, p: Point2<T>) -> Vector2<T> {
        let mut o = p - self.p_min;

        if self.p_max.x > self.p_min.x {
            o.x /= self.p_max.x - self.p_min.x;
        }

        if self.p_max.y > self.p_min.y {
            o.y /= self.p_max.y - self.p_min.y;
        }

        o
    }
}

#[derive(Debug, Default, Copy, Clone)]
pub struct Bounds3<T> {
    pub p_min: Point3<T>,
    pub p_max: Point3<T>,
}

impl<T: Number> Bounds3<T> {
    pub fn new(p_min: Point3<T>, p_max: Point3<T>) -> Self {
		// TODO: validate input data
        Self { p_min, p_max }
    }

    pub fn corner(self, corner: u8) -> Point3<T> {
        assert!(corner < 8);
        Point3 {
            x: if corner & 1 == 0 {
                self.p_min.x
            } else {
                self.p_max.x
            },
            y: if corner & 2 == 0 {
                self.p_min.y
            } else {
                self.p_max.y
            },
            z: if corner & 4 == 0 {
                self.p_min.z
            } else {
                self.p_max.z
            },
        }
    }

    pub fn diagonal(self) -> Vector3<T> {
        self.p_max - self.p_min
    }

    pub fn surface_area(self) -> T {
        let d = self.diagonal();
        let r = d.x * d.y + d.x * d.z + d.y * d.z;
        r + r
    }

	/// The dimension that the bounds extends into the most
    pub fn maximum_extent(self) -> u8 {
        let d = self.diagonal();
        if d.x > d.y && d.x > d.z {
            0
        } else if d.y > d.z {
            1
        } else {
            2
        }
    }

    pub fn offset(self, p: Point3<T>) -> Vector3<T> {
        let mut o = p - self.p_min;
        if self.p_max.x > self.p_min.x {
            o.x /= self.p_max.x - self.p_min.x;
        }
        if self.p_max.y > self.p_min.y {
            o.y /= self.p_max.y - self.p_min.y;
        }
        if self.p_max.z > self.p_min.z {
            o.z /= self.p_max.z - self.p_min.z;
        }
        o
    }

    // pub fn bounding_sphere(self, center: &mut Point3<T>, radius: &mut T) {
    //     todo!()
    // }

    pub fn lerp(&self, t: &Point3<T>) -> Point3<T>
    where
        T: Float,
    {
        Point3 {
            x: lerp(t.x, self.p_min.x, self.p_max.x),
            y: lerp(t.y, self.p_min.y, self.p_max.y),
            z: lerp(t.z, self.p_min.z, self.p_max.z),
        }
    }

    // pub fn intersect_b(&self, ray: &Ray, hitt0: &mut Float, hitt1: &mut Float) -> bool {
    //     todo!()
    // }

    // pub fn intersect_p(&self, ray: &Ray, inv_dir: &Vector3f, dir_is_neg: &[u8; 3]) -> bool {
    //     todo!()
    // }
}
