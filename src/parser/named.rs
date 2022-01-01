use super::{
    utils::*,
    value::{parse_value, Value},
};
use nom::{
    bytes::complete::tag,
    character::complete::char,
    error::ParseError,
    sequence::{delimited, preceded},
    IResult,
};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum NamedTokenType {
    Accelerator,
    AreaLightSource,
    Camera,
    CoordSys,
    CoordSysTransform,
    Film,
    Integrator,
    LightSource,
    MakeNamedMaterial,
    MakeNamedMedium,
    Material,
    NamedMaterial,
    Include,
    PixelFilter,
    Sampler,
    Shape,
    ObjectInstance,
    ObjectBegin,
    SurfaceIntegrator,
    VolumeIntegrator,
}
pub fn parse_named_token_type<'a, E: ParseError<&'a str>>(
    i: &'a str,
) -> IResult<&'a str, NamedTokenType, E> {
    nom::branch::alt((
        nom::combinator::map(tag("Accelerator"), |_| NamedTokenType::Accelerator),
        nom::combinator::map(tag("AreaLightSource"), |_| NamedTokenType::AreaLightSource),
        nom::combinator::map(tag("Camera"), |_| NamedTokenType::Camera),
        nom::combinator::map(tag("CoordSysTransform"), |_| {
            NamedTokenType::CoordSysTransform
        }),
        nom::combinator::map(tag("CoordSys"), |_| NamedTokenType::CoordSys),
        nom::combinator::map(tag("Film"), |_| NamedTokenType::Film),
        nom::combinator::map(tag("Integrator"), |_| NamedTokenType::Integrator),
        nom::combinator::map(tag("LightSource"), |_| NamedTokenType::LightSource),
        nom::combinator::map(tag("MakeNamedMaterial"), |_| {
            NamedTokenType::MakeNamedMaterial
        }),
        nom::combinator::map(tag("MakeNamedMedium"), |_| NamedTokenType::MakeNamedMedium),
        nom::combinator::map(tag("Material"), |_| NamedTokenType::Material),
        nom::combinator::map(tag("NamedMaterial"), |_| NamedTokenType::NamedMaterial),
        nom::combinator::map(tag("Include"), |_| NamedTokenType::Include),
        nom::combinator::map(tag("PixelFilter"), |_| NamedTokenType::PixelFilter),
        nom::combinator::map(tag("Sampler"), |_| NamedTokenType::Sampler),
        nom::combinator::map(tag("Shape"), |_| NamedTokenType::Shape),
        nom::combinator::map(tag("ObjectInstance"), |_| NamedTokenType::ObjectInstance),
        nom::combinator::map(tag("ObjectBegin"), |_| NamedTokenType::ObjectBegin),
        nom::combinator::map(tag("SurfaceIntegrator"), |_| {
            NamedTokenType::SurfaceIntegrator
        }),
        nom::combinator::map(tag("VolumeIntegrator"), |_| {
            NamedTokenType::VolumeIntegrator
        }),
    ))(i)
}

#[derive(Debug, Clone)]
pub struct NamedToken {
    pub internal_type: String,
    pub values: HashMap<String, Value>,
    pub object_type: NamedTokenType,
}

pub fn parse_named_token<'a, E: ParseError<&'a str>>(
    i: &'a str,
) -> IResult<&'a str, NamedToken, E> {
    let (i, object_type) = parse_named_token_type(i)?;
    let (i, internal_type) = nom::combinator::cut(preceded(
        sp,
        delimited(
            preceded(char('"'), sp),
            parse_string_empty, // Can be empty due to Material "" => None
            preceded(sp, char('"')),
        ),
    ))(i)?;

    let (i, values) = nom::combinator::cut(nom::multi::fold_many0(
        preceded(sp, parse_value),
        HashMap::new,
        |mut acc: HashMap<String, Value>, item: (String, Value)| {
            acc.insert(item.0, item.1);
            acc
        },
    ))(i)?;

    Ok((
        i,
        NamedToken {
            internal_type: internal_type.to_owned(),
            values,
            object_type,
        },
    ))
}

// pub fn parse_named_token_many<'a, E: ParseError<&'a str>>(
//     i: &'a str,
// ) -> IResult<&'a str, Vec<NamedToken>, E> {
//     nom::multi::fold_many1(
//         preceded(sp, parse_named_token),
//         Vec::new,
//         |mut acc: Vec<NamedToken>, item: NamedToken| {
//             acc.push(item);
//             acc
//         },
//     )(i)
// }
