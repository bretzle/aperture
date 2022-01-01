use super::{
    utils::{parse_string, parse_string_sp, sp},
    BSDFFloat, Blackbody, Spectrum, RGB,
};
use crate::math::{Vector2, Vector3};
use nom::{
    character::complete::char,
    error::ParseError,
    number::complete::float,
    sequence::{delimited, preceded},
    IResult,
};

// Contain the list of parameter type
// some type are on the same one to avoid unecessary
// repetition in the code below
#[derive(Debug, Clone)]
pub enum Value {
    Integer(Vec<i32>),
    Float(Vec<f32>),
    Vector3(Vec<Vector3<f32>>),
    Vector2(Vec<Vector2<f32>>),
    String(String),
    Texture(String),
    Spectrum(String),
    RGB(RGB),
    Blackbody(Blackbody),
    Boolean(bool),
}

impl Value {
    pub fn into_integer(self) -> Vec<i32> {
        match self {
            Value::Integer(v) => v,
            _ => panic!("into_integer failed: {:?}", self),
        }
    }

    pub fn into_floats(self) -> Vec<f32> {
        match self {
            Value::Float(v) => v,
            _ => panic!("into_float failed: {:?}", self),
        }
    }

    pub fn into_float(self) -> f32 {
        match self {
            Value::Float(v) => {
                assert!(v.len() == 1);
                v[0]
            }
            _ => panic!("into_float failed: {:?}", self),
        }
    }

    pub fn into_vector3(self) -> Vec<Vector3<f32>> {
        match self {
            Value::Vector3(v) => v,
            _ => panic!("into_vector3 failed: {:?}", self),
        }
    }

    // pub fn into_vector2(self) -> Vec<Vector2<f32>> {
    //     match self {
    //         Value::Vector2(v) => v,
    //         _ => panic!("into_vector2 failed: {:?}", self),
    //     }
    // }

    pub fn into_string(self) -> String {
        match self {
            Value::String(v) => v,
            _ => panic!("into_string failed: {:?}", self),
        }
    }

    pub fn into_bool(self) -> bool {
        match self {
            Value::Boolean(v) => v,
            _ => panic!("into_bool failed: {:?}", self),
        }
    }

    pub fn into_rgb(self) -> RGB {
        match self {
            Value::RGB(v) => v,
            _ => panic!("into_rgb failed: {:?}", self),
        }
    }

    pub fn into_spectrum(self) -> Spectrum {
        match self {
            Value::RGB(v) => Spectrum::RGB(v),
            Value::Blackbody(v) => Spectrum::Blackbody(v),
            Value::Texture(v) => Spectrum::Texture(v),
            Value::Spectrum(v) => Spectrum::Spectrum(v),
            _ => panic!("into_spectrum failed: {:?}", self),
        }
    }

    pub fn into_bsdf_float(self) -> BSDFFloat {
        match self {
            Value::Texture(v) => BSDFFloat::Texture(v),
            Value::Float(v) => {
                assert_eq!(v.len(), 1);
                BSDFFloat::Float(v[0])
            }
            _ => panic!("into_spectrum failed: {:?}", self),
        }
    }
}

pub fn parse_value_helper<'a, E: ParseError<&'a str>, O, F1, F2>(
    f1: F1,
    f2: F2,
) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
where
    F1: nom::Parser<&'a str, O, E>,
    F2: nom::Parser<&'a str, O, E>,
{
    nom::branch::alt((
        delimited(preceded(char('['), sp), f1, preceded(sp, char(']'))),
        f2,
    ))
}

pub fn parse_value<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, (String, Value), E> {
    let (i, (t, n)) = delimited(
        char('"'),
        nom::sequence::tuple((parse_string_sp, preceded(sp, parse_string))),
        char('"'),
    )(i)?;
    // dbg!(t, n);

    let (i, v) = match t {
        "integer" => {
            let (i, v) = preceded(
                sp,
                parse_value_helper(
                    nom::multi::many0(preceded(sp, nom::character::complete::digit1)),
                    nom::multi::many0(preceded(sp, nom::character::complete::digit1)),
                ),
            )(i)?;
            let v = v.into_iter().map(|v| v.parse::<i32>().unwrap()).collect();
            (i, Value::Integer(v))
        }
        "bool" | "boolean" => {
            let (i, v) = preceded(
                sp,
                parse_value_helper(
                    nom::branch::alt((
                        delimited(char('"'), nom::character::complete::alpha1, char('"')),
                        nom::character::complete::alpha1,
                    )),
                    nom::branch::alt((
                        delimited(char('"'), nom::character::complete::alpha1, char('"')),
                        nom::character::complete::alpha1,
                    )),
                ),
            )(i)?;

            let v = match v {
                "false" => false,
                "true" => true,
                _ => panic!("Wrong bool type: {}", v),
            };

            (i, Value::Boolean(v))
        }
        "point" | "normal" | "vector" | "vector3" | "point3" => {
            let (i, v) = preceded(
                sp,
                delimited(
                    preceded(char('['), sp),
                    nom::multi::many0(preceded(sp, float)),
                    preceded(sp, char(']')),
                ),
            )(i)?;
            assert_eq!(v.len() % 3, 0);
            let v = v
                .chunks_exact(3)
                .map(|v| Vector3::new(v[0], v[1], v[2]))
                .collect();
            (i, Value::Vector3(v))
        }
        "vector2" | "point2" => {
            let (i, v) = preceded(
                sp,
                delimited(
                    preceded(char('['), sp),
                    nom::multi::many0(preceded(sp, float)),
                    preceded(sp, char(']')),
                ),
            )(i)?;
            assert_eq!(v.len() % 2, 0);
            let v = v
                .chunks_exact(2)
                .map(|v| Vector2::new(v[0], v[1]))
                .collect();
            (i, Value::Vector2(v))
        }
        "float" => {
            let (i, v) = preceded(
                sp,
                parse_value_helper(
                    nom::multi::many0(preceded(sp, float)),
                    nom::multi::many0(preceded(sp, float)),
                ),
            )(i)?;
            (i, Value::Float(v))
        }
        "rgb" | "color" => {
            let (i, (r, g, b)) = preceded(
                sp,
                delimited(
                    preceded(char('['), sp),
                    nom::sequence::tuple((float, preceded(sp, float), preceded(sp, float))),
                    preceded(sp, char(']')),
                ),
            )(i)?;
            (i, Value::RGB(RGB { r, g, b }))
        }
        "blackbody" => {
            let (i, (temperature, scale)) = preceded(
                sp,
                delimited(
                    preceded(char('['), sp),
                    nom::branch::alt((
                        nom::sequence::tuple((float, preceded(sp, float))),
                        nom::combinator::map(float, |f| (f, 1.0)),
                    )),
                    preceded(sp, char(']')),
                ),
            )(i)?;
            (i, Value::Blackbody(Blackbody { temperature, scale }))
        }
        "string" | "texture" | "spectrum" => {
            let (i, v) = preceded(
                sp,
                parse_value_helper(
                    delimited(char('"'), parse_string, char('"')),
                    delimited(char('"'), parse_string, char('"')),
                ),
            )(i)?;
            match t {
                "string" => (i, Value::String(v.to_owned())),
                "texture" => (i, Value::Texture(v.to_owned())),
                "spectrum" => (i, Value::Spectrum(v.to_owned())),
                _ => panic!("Impossible to convert str to type"),
            }
        }
        _ => panic!("{:?} not valid type", t),
    };

    Ok((i, (n.to_owned(), v)))
}
