use super::{quaternion, Quaternion, Transform, Vector};

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

    fn decompose(_transform: &Transform) -> (Vector, Quaternion, Vector) {
        todo!()
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
