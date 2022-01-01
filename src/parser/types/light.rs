use super::*;
use crate::{
    math::{Matrix, Point3, Vector3},
    parser::{value::Value, NamedToken},
    remove_default,
};

/// Lights
#[derive(Debug)]
pub enum Light {
    Distant {
        luminance: Spectrum,
        from: Point3<f32>,
        to: Point3<f32>,
        scale: Rgb,
    },
    Infinite {
        luminance: Spectrum,
        samples: u32,
        scale: Rgb,
    },
    Point {
        intensity: Spectrum,
        from: Point3<f32>,
        scale: Rgb,
    },
}

impl Light {
    pub fn new(mut named_token: NamedToken, mat: Matrix) -> Option<Self> {
        let scale = if let Some(scale) = named_token.values.remove("scale") {
            scale.into_rgb()
        } else {
            Rgb::color(1.0)
        };

        match &named_token.internal_type[..] {
            "infinite" => {
                let samples =
                    remove_default!(named_token.values, "samples", Value::Integer(vec![1]))
                        .into_integer()[0] as u32;
                let luminance =
                    remove_default!(named_token.values, "L", Value::Rgb(Rgb::color(1.0)))
                        .into_spectrum();

                // In case the map name is provide, we will replace the luminance
                let luminance = if let Some(mapname) = named_token.values.remove("mapname") {
                    Spectrum::Mapname(mapname.into_string())
                } else {
                    luminance
                };

                Some(Light::Infinite {
                    luminance,
                    samples,
                    scale,
                })
            }
            "point" => {
                let intensity =
                    remove_default!(named_token.values, "I", Value::Rgb(Rgb::color(1.0)))
                        .into_spectrum();
                let from = remove_default!(
                    named_token.values,
                    "from",
                    Value::Vector3(vec![Vector3::new(0.0, 0.0, 0.0)])
                )
                .into_vector3()[0];
                let from = Point3::from_vec(from);
                let from = mat.to_transform().transform_point(from);
                Some(Light::Point {
                    intensity,
                    from,
                    scale,
                })
            }
            "distant" => {
                let luminance =
                    remove_default!(named_token.values, "L", Value::Rgb(Rgb::color(1.0)))
                        .into_spectrum();
                let from = remove_default!(
                    named_token.values,
                    "from",
                    Value::Vector3(vec![Vector3::new(0.0, 0.0, 0.0)])
                )
                .into_vector3()[0];
                let to = remove_default!(
                    named_token.values,
                    "to",
                    Value::Vector3(vec![Vector3::new(0.0, 0.0, 0.0)])
                )
                .into_vector3()[0];
                let from = Point3::from_vec(from);
                let to = Point3::from_vec(to);
                let from = mat.to_transform().transform_point(from);
                let to = mat.to_transform().transform_point(to);
                Some(Light::Distant {
                    luminance,
                    from,
                    to,
                    scale,
                })
            }
            _ => {
                warn!("Light case with {} is not cover", named_token.internal_type);
                None
            }
        }
    }
}
