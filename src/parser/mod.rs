use self::keyword::*;
use self::named::*;
use self::scene::{Scene, State};
use self::tokens::*;
use self::types::*;
use self::utils::*;
use self::value::*;
use crate::math::*;
use crate::transform::Transform;
use nom::{error::VerboseError, sequence::preceded, IResult};
use std::{collections::HashMap, fs::read_to_string, path::Path, time::Instant};

mod keyword;
mod named;
mod scene;
mod tokens;
mod types;
mod utils;
mod value;

#[macro_export]
macro_rules! remove_default {
    ($map: expr, $name:expr, $default:expr) => {{
        if let Some(v) = $map.remove($name) {
            v
        } else {
            $default
        }
    }};
}

pub struct PbrtParser<'a> {
    scene: Scene,
    state: State,
    working_dir: &'a Path,
}

impl<'a> PbrtParser<'a> {
    pub fn load_pbrt(path: &'a str) -> Scene {
        let path: &'a Path = path.as_ref();

        let mut parser = Self {
            scene: Scene::default(),
            state: State::default(),
            working_dir: path.parent().unwrap(),
        };

        parser.read_pbrt_file(path);

        parser.scene
    }

    fn read_pbrt_file(&mut self, path: &Path) {
        info!("Loading: {}", path.display());

        let now = Instant::now();
        let contents = read_to_string(path).expect("Failed to read file");
        let tokens = self.tokenize(&contents);
        self.parse(tokens);

        info!("Time for parsing file: {:?}", now.elapsed());
    }

    fn tokenize(&self, scene_string: &'a str) -> Vec<Token> {
        fn parse<'a>(i: &'a str) -> IResult<&'a str, Vec<Token>, VerboseError<&str>> {
            let (i, v) = preceded(
                sp,
                nom::multi::fold_many0(
                    preceded(sp, parse_token),
                    Vec::new,
                    |mut acc: Vec<Token>, item: Token| {
                        acc.push(item);
                        acc
                    },
                ),
            )(i)?;

            let (i, _) = sp(i)?;

            Ok((i, v))
        }

        let (scene_string, tokens) = parse(scene_string).expect("Error during parsing");

        if !scene_string.is_empty() {
            panic!("Parsing is not complete: {:?}", scene_string)
        }

        tokens
    }

    fn parse(&mut self, tokens: Vec<Token>) {
        for t in tokens {
            match t {
                Token::Transform(values) => {
                    let m00 = values[0];
                    let m01 = values[1];
                    let m02 = values[2];
                    let m03 = values[3];
                    let m10 = values[4];
                    let m11 = values[5];
                    let m12 = values[6];
                    let m13 = values[7];
                    let m20 = values[8];
                    let m21 = values[9];
                    let m22 = values[10];
                    let m23 = values[11];
                    let m30 = values[12];
                    let m31 = values[13];
                    let m32 = values[14];
                    let m33 = values[15];
                    #[rustfmt::skip]
                    let matrix = Matrix::new(
                        [m00, m01, m02, m03],
                        [m10, m11, m12, m13],
                        [m20, m21, m22, m23],
                        [m30, m31, m32, m33],
                    );
                    self.state.replace_matrix(matrix);
                },
                Token::ConcatTransform(values) => {
                    let m00 = values[0];
                    let m01 = values[1];
                    let m02 = values[2];
                    let m03 = values[3];
                    let m10 = values[4];
                    let m11 = values[5];
                    let m12 = values[6];
                    let m13 = values[7];
                    let m20 = values[8];
                    let m21 = values[9];
                    let m22 = values[10];
                    let m23 = values[11];
                    let m30 = values[12];
                    let m31 = values[13];
                    let m32 = values[14];
                    let m33 = values[15];

                    #[rustfmt::skip]
                    let matrix = self.state.matrix() * Matrix::new(
                        [m00, m01, m02, m03],
                        [m10, m11, m12, m13],
                        [m20, m21, m22, m23],
                        [m30, m31, m32, m33],
                    );
                    self.state.replace_matrix(matrix);
                },
                Token::Scale(values) => {
                    let matrix = self.state.matrix()
                        * Matrix::from_diagonal(
                            values
                        );
                    self.state.replace_matrix(matrix);
                },
                Token::LookAt {
                    eye, look, up
                } => {
                    let dir = (look - eye).normalize();
                    let left = -dir.cross(up.normalize()).normalize();
                    let new_up = dir.cross(left);
                    #[rustfmt::skip]
                    let matrix = self.state.matrix() *  Matrix::new(
                        [left.x, left.y, left.z, 0.0],
                        [new_up.x, new_up.y, new_up.z, 0.0],
                        [dir.x, dir.y, dir.z, 0.0],
                        [eye.x, eye.y, eye.z, 1.0],
                    ).inverse().unwrap();
                    self.state.replace_matrix(matrix);
                },
                Token::Translate(values) => {
                    let matrix = self.state.matrix()
                            * Transform::translate(values).to_matrix();
                    self.state.replace_matrix(matrix);
                },
                Token::Rotate {
                    angle,
                    axis
                } => {
                    let matrix = self.state.matrix() * Matrix::from_axis_angle(axis, Angle::deg(angle));
                    self.state.replace_matrix(matrix);
                },
                Token::Keyword(key) => {
                    match key {
                        Keyword::AttributeBegin | Keyword::TransformBegin => {
                            self.state.save();
                        }
                        Keyword::AttributeEnd | Keyword::TransformEnd => {
                            self.state.restore();
                        }
                        Keyword::Identity => {
                            self.state.replace_matrix(Matrix::IDENTITY);
                        }
                        Keyword::WorldBegin => {
                             // Reinit the transformation matrix
                            self.state.replace_matrix(Matrix::IDENTITY);
                        }
                        Keyword::ObjectEnd => {
                            let object = self.state.finish_object();
                            self.scene
                            .objects
                            .insert(object.name.clone(), object);
                        }
                        Keyword::WorldEnd => {
                            // Nothing?
                        }
                        Keyword::ReverseOrientation => {
                            self.state.reverse_orientation = !self.state.reverse_orientation;
                        }
                    }
                },
                Token::ActiveTransform(_) => todo!(),
                Token::MediumInterface { .. } => todo!(),
                Token::Texture {
                    name, class, mut values, .. // t not read
                } => {
                        // TODO: WK
                        // Check type as Well... {spectrum or float} -> Two lists    
                        // TODO: A lot of parameters...
                        match &class[..] {
                            "imagemap" => {
                                let filename = self.working_dir.join(values.remove("filename").unwrap().into_string()).to_str().unwrap().to_owned();
                                self.scene.textures.insert(name, Texture {
                                    filename,
                                    trilinear: remove_default!(values, "trilinear", Value::Boolean(false)).into_bool(),
                                });
                            }
                            _ => warn!("texture type {} is ignored", class)
                        }
                    },
                Token::NamedToken(mut named_token) => {
                    match named_token.object_type {
                        NamedTokenType::Accelerator | NamedTokenType::Integrator | NamedTokenType::Sampler | NamedTokenType::PixelFilter | NamedTokenType::SurfaceIntegrator | NamedTokenType::VolumeIntegrator => {
                            // Nothing...
                        },
                        NamedTokenType::Camera => {
                            if let Some(c) = Camera::new(named_token, self.state.matrix()) {
                                self.scene.cameras.push(c);
                                self.scene
                                    .transforms
                                    .insert("camera".to_string(), self.state.matrix().inverse().unwrap());
                            }
                        },
                        NamedTokenType::MakeNamedMaterial => {
                            let name = named_token.internal_type.clone();
                            if let Some(bsdf) = BSDF::new(named_token, false) {
                                self.scene.materials.insert(name, bsdf);
                            }
                        }
                        NamedTokenType::NamedMaterial => {
                            assert!(named_token.values.is_empty());
                            self.state.set_named_matrial(named_token.internal_type);
                        }
                        NamedTokenType::Material => {
                            if let Some(bsdf) = BSDF::new(named_token, true) {
                                // Create a fake name...
                                let name = format!(
                                    "unamed_material_{}",
                                    self.scene.number_unamed_materials
                                );
                                self.scene.number_unamed_materials += 1;
                                self.scene.materials.insert(name.to_string(), bsdf);
                                self.state.set_named_matrial(name);
                            }
                        }
                        NamedTokenType::Shape => {
                            if let Some(shape) = Shape::new(named_token, self.working_dir) {
                                let mut shape = ShapeInfo::new(shape, self.state.matrix());
                                shape.material_name = self.state.named_material();
                                shape.emission = self.state.emission();
                                shape.reverse_orientation = self.state.reverse_orientation;
                                match &mut self.state.object {
                                    Some(o) => {
                                        // info!("Added inside an object: {}", o.name);
                                        o.shapes.push(shape)
                                    }
                                    None => {
                                        // info!("Put inside scene_info");
                                        self.scene.shapes.push(shape)
                                    }
                                };
                            }
                        }
                        NamedTokenType::Film => {
                            self.scene.image_size = Vector2::new(
                                named_token.values.remove("xresolution").unwrap().into_integer()[0] as u32,
                                named_token.values.remove("yresolution").unwrap().into_integer()[0] as u32,
                            );
                        }
                        NamedTokenType::AreaLightSource => {
                            match &named_token.internal_type[..] {
                                "diffuse" => {
                                    if let Some(e) = named_token.values.remove("L") {
                                        self.state.set_emission(e.into_spectrum());
                                    }
                                }
                                _ => warn!("Unsuppored area light: {}", named_token.internal_type),
                            }
                        }
                        NamedTokenType::LightSource => {
                            if let Some(light) = Light::new(named_token, self.state.matrix()) {
                                self.scene.lights.push(light);
                            }
                        }
                        NamedTokenType::CoordSysTransform => {
                            self.state.replace_matrix(*self.scene.transforms.get(&named_token.internal_type).unwrap());
                        }
                        NamedTokenType::CoordSys => {
                            self.scene
                            .transforms
                            .insert(named_token.internal_type, *self.state.matrix.last().unwrap());
                        }
                        NamedTokenType::Include => {
                            info!("Include found: {}", named_token.internal_type);
                            let filename = self.working_dir.join(named_token.internal_type);
                            self.read_pbrt_file(
                                filename.to_str().unwrap().as_ref(),
                            );
                        }
                        NamedTokenType::ObjectBegin => {
                            self.state.new_object(named_token.internal_type);
                        }
                        NamedTokenType::ObjectInstance => {
                            self.scene.instances.push(
                                InstanceInfo {
                                    matrix: self.state.matrix(),
                                    name: named_token.internal_type
                                }
                            )
                        }
                        _ => warn!("{:?} not implemented", named_token.object_type),
                    }
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct Texture {
    pub filename: String,
    pub trilinear: bool,
}

#[derive(Debug)]
pub struct InstanceInfo {
    pub matrix: Matrix,
    pub name: String,
}

#[derive(Debug)]
pub struct ObjectInfo {
    pub name: String,
    pub shapes: Vec<ShapeInfo>,
    pub matrix: Matrix,
}
