use super::{quaternion, Matrix, Quaternion, Transform, Vector};

#[derive(Debug, Copy, Clone)]
pub struct Keyframe {
    pub translation: Vector,
    pub rotation: Quaternion,
    pub scaling: Vector,
}

impl Keyframe {
    pub fn new(transform: &Transform) -> Self {
        let (translation, rotation, scaling) = Keyframe::decompose(transform);
        Self {
            translation,
            rotation,
            scaling,
        }
    }

    pub fn from_parts(translation: Vector, rotation: Quaternion, scaling: Vector) -> Self {
        Self {
            translation,
            rotation,
            scaling,
        }
    }

    pub fn transform(&self) -> Transform {
        let m = self.rotation.to_matrix();
        Transform::translate(&self.translation)
            * Transform::from_matrix(m)
            * Transform::scale(&self.scaling)
    }

    fn decompose(transform: &Transform) -> (Vector, Quaternion, Vector) {
        let m = transform.mat;
        let translation = Vector::new(*m.at(0, 3), *m.at(1, 3), *m.at(2, 3));
        // Robust matrix decomposition, based on Mitsuba:
        // We use SVD to extract rotation and scaling matrices that properly account for flip
        let la_mat = la::Matrix::<f64>::new(
            3,
            3,
            vec![
                *m.at(0, 0) as f64,
                *m.at(0, 1) as f64,
                *m.at(0, 2) as f64,
                *m.at(1, 0) as f64,
                *m.at(1, 1) as f64,
                *m.at(1, 2) as f64,
                *m.at(2, 0) as f64,
                *m.at(2, 1) as f64,
                *m.at(2, 2) as f64,
            ],
        );
        // TODO: More explanation of the math going on here, why do we choose these matrices for
        // q and p. q is the basis transform of the matrix without scaling so it represents the
        // rotation, while p is the scaling transformed from the basis into the canonical basis
        // for R^3, giving scaling in the canonical basis. Is this intuition correct?
        let svd = la::SVD::<f64>::new(&la_mat);
        let mut q = svd.get_u() * svd.get_v().t();
        let mut p = svd.get_v() * svd.get_s() * svd.get_v().t();
        if q.det() < 0.0 {
            q = -q;
            p = -p;
        }
        let rotation = Quaternion::from_matrix(&Matrix::new([
            q.get(0, 0) as f32,
            q.get(0, 1) as f32,
            q.get(0, 2) as f32,
            0.0,
            q.get(1, 0) as f32,
            q.get(1, 1) as f32,
            q.get(1, 2) as f32,
            0.0,
            q.get(2, 0) as f32,
            q.get(2, 1) as f32,
            q.get(2, 2) as f32,
            0.0,
            0.0,
            0.0,
            0.0,
            1.0,
        ]));
        let scaling = Vector::new(p.get(0, 0) as f32, p.get(1, 1) as f32, p.get(2, 2) as f32);
        (translation, rotation, scaling)
    }
}

impl bspline::Interpolate<f32> for Keyframe {
    fn interpolate(&self, other: &Self, t: f32) -> Self {
        let translation = (1.0 - t) * self.translation + t * other.translation;
        let rotation = quaternion::slerp(t, &self.rotation, &other.rotation);
        let scaling = (1.0 - t) * self.scaling + t * other.scaling;
        Keyframe::from_parts(translation, rotation, scaling)
    }
}
