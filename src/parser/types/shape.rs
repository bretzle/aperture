use super::Spectrum;
use crate::{
    math::{Matrix, Point3, Vector2, Vector3},
    parser::{value::Value, NamedToken, Texture},
    remove_default,
};
use std::path::Path;

/// Mesh representation
#[derive(Debug, Clone)]
pub enum Shape {
    TriMesh {
        indices: Vec<Vector3<usize>>,
        points: Vec<Point3<f32>>,
        normals: Option<Vec<Vector3<f32>>>,
        uv: Option<Vec<Vector2<f32>>>,
    },
    Ply {
        filename: String,
        alpha: Option<Texture>,
        shadowalpha: Option<Texture>,
    },
    Sphere {
        radius: f32, // 1.0
        z_min: Option<f32>,
        z_max: Option<f32>,
        phi_max: f32, //< 360 (in degree)
    },
    Disk {
        height: f32,       // 0.0
        radius: f32,       // 1.0
        inner_radius: f32, // 0.0
        phi_max: f32,      // 360 (in degree)
    },
}

impl Shape {
    pub fn new(mut named_token: NamedToken, wk: &Path) -> Option<Self> {
        match &named_token.internal_type[..] {
            "trianglemesh" => {
                let points = named_token
                    .values
                    .remove("P")
                    .expect(&format!("P is required {:?}", named_token))
                    .into_vector3();
                let points = points.into_iter().map(|v| Point3::from_vec(v)).collect();
                let indices = named_token
                    .values
                    .remove("indices")
                    .expect(&format!("indice is required {:?}", named_token))
                    .into_integer();
                if indices.len() % 3 != 0 {
                    panic!("Support only 3 indices list {:?}", named_token);
                }
                let indices = indices
                    .chunks(3)
                    .map(|v| Vector3::new(v[0] as usize, v[1] as usize, v[2] as usize))
                    .collect();
                let normals = if let Some(v) = named_token.values.remove("N") {
                    Some(v.into_vector3())
                } else {
                    None
                };
                let uv = if let Some(v) = named_token.values.remove("uv") {
                    let v = v.into_floats();
                    assert_eq!(v.len() % 2, 0);
                    let v = v.chunks(2).map(|v| Vector2::new(v[0], v[1])).collect();
                    Some(v)
                } else {
                    None
                };
                Some(Shape::TriMesh {
                    indices,
                    points,
                    normals,
                    uv,
                })
            }
            "plymesh" => {
                let filename = named_token
                    .values
                    .remove("filename")
                    .expect("filename is required")
                    .into_string();
                let filename = wk.join(filename).to_str().unwrap().to_owned();
                Some(Shape::Ply {
                    filename,
                    alpha: None,       // FIXME
                    shadowalpha: None, // FIXME
                })
            }
            "sphere" => {
                let radius = remove_default!(named_token.values, "radius", Value::Float(vec![1.0]))
                    .into_float();
                let z_min = named_token.values.remove("zmin").map(|v| v.into_float());
                let z_max = named_token.values.remove("zmax").map(|v| v.into_float());
                let phi_max =
                    remove_default!(named_token.values, "phimax", Value::Float(vec![360.0]))
                        .into_float();

                Some(Shape::Sphere {
                    radius,
                    z_min,
                    z_max,
                    phi_max,
                })
            }
            "disk" => {
                let radius = remove_default!(named_token.values, "radius", Value::Float(vec![1.0]))
                    .into_float();
                let phi_max =
                    remove_default!(named_token.values, "phimax", Value::Float(vec![360.0]))
                        .into_float();
                let height = remove_default!(named_token.values, "height", Value::Float(vec![0.0]))
                    .into_float();
                let inner_radius =
                    remove_default!(named_token.values, "innerradius", Value::Float(vec![0.0]))
                        .into_float();

                Some(Shape::Disk {
                    radius,
                    phi_max,
                    height,
                    inner_radius,
                })
            }

            _ => {
                warn!("Shape case with {} is not cover", named_token.internal_type);
                None
            }
        }
    }
}

/// Scene representation
#[derive(Debug, Clone)]
pub struct ShapeInfo {
    pub data: Shape,
    pub material_name: Option<String>,
    pub matrix: Matrix,
    pub reverse_orientation: bool,
    pub emission: Option<Spectrum>,
}

impl ShapeInfo {
    pub fn new(shape: Shape, matrix: Matrix) -> Self {
        Self {
            data: shape,
            material_name: None,
            matrix,
            reverse_orientation: false,
            emission: None,
        }
    }
}
