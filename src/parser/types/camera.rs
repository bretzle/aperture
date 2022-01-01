use crate::{math::Matrix, parser::NamedToken};

/// Camera representations
#[derive(Debug)]
pub enum Camera {
    Perspective { fov: f32, world_to_camera: Matrix },
}

impl Camera {
    pub fn new(mut named_token: NamedToken, mat: Matrix) -> Option<Self> {
        match &named_token.internal_type[..] {
            "perspective" => {
                let fov = named_token
                    .values
                    .remove("fov")
                    .expect("fov is not given")
                    .into_floats()[0];
                Some(Camera::Perspective {
                    fov,
                    world_to_camera: mat,
                })
            }
            _ => {
                warn!(
                    "Camera case with {:?} is not cover",
                    named_token.internal_type
                );
                None
            }
        }
    }
}
