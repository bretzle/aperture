use super::{Keyframe, Transform};
use bspline::BSpline;
use std::ops::Mul;

#[derive(Clone, Debug)]
pub struct AnimatedTransform {
    keyframes: Vec<BSpline<Keyframe, f32>>,
}

impl AnimatedTransform {
    /// Create an animated transformation blending between the passed keyframes
    pub fn with_keyframes(mut _keyframes: Vec<Keyframe>, _knots: Vec<f32>, _degree: usize) -> Self {
        todo!()
    }

    pub fn unanimated(transform: &Transform) -> Self {
        let key = Keyframe::new(transform);
        Self {
            keyframes: vec![BSpline::new(0, vec![key], vec![0.0, 1.0])],
        }
    }

    /// Compute the transformation matrix for the animation at some time point using B-Spline
    /// interpolation.
    pub fn transform(&self, time: f32) -> Transform {
        let mut transform = Transform::IDENTITY;
        // Step through the transform stack, applying each animation transform at this
        // time as we move up
        for spline in &self.keyframes {
            let domain = spline.knot_domain();
            let t = if spline.control_points().count() == 1 {
                spline.control_points().next().unwrap().transform()
            } else {
                let t_val = super::clamp(time, domain.0, domain.1);
                spline.point(t_val).transform()
            };
            transform = t * transform;
        }

        transform
    }

    /// Check if the transform is actually animated
    pub fn is_animated(&self) -> bool {
        self.keyframes.is_empty()
            || self
                .keyframes
                .iter()
                .all(|spline| spline.control_points().count() > 1)
    }
}

impl Mul for AnimatedTransform {
    type Output = AnimatedTransform;

    fn mul(self, mut rhs: AnimatedTransform) -> AnimatedTransform {
        for l in &self.keyframes[..] {
            rhs.keyframes.push(l.clone());
        }
        rhs
    }
}
