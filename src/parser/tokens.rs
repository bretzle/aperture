use super::{
    keyword::{parse_keyword, Keyword},
    named::{parse_named_token, NamedToken},
    value::{parse_value, Value},
};
use crate::{parser::utils::*, math::Vector3};
use nom::{
    bytes::complete::tag,
    character::complete::char,
    error::ParseError,
    number::complete::float,
    sequence::{delimited, preceded},
    IResult,
};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum Token {
    Transform(Vec<f32>),
    ConcatTransform(Vec<f32>),
    Texture {
        name: String,
        t: String,
        class: String,
        values: HashMap<String, Value>,
    },
    NamedToken(NamedToken),
    Keyword(Keyword),
    MediumInterface {
        inside: String,
        outside: String,
    },
    LookAt {
        eye: Vector3<f32>,
        look: Vector3<f32>,
        up: Vector3<f32>,
    },
    Scale(Vector3<f32>),
    Translate(Vector3<f32>),
    Rotate {
        angle: f32,
        axis: Vector3<f32>,
    },
    ActiveTransform(String),
}

pub fn transform<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, Token, E> {
    let (i, v) = preceded(
        preceded(tag("Transform"), sp),
        delimited(
            preceded(char('['), sp),
            nom::multi::many0(preceded(sp, float)),
            preceded(sp, char(']')),
        ),
    )(i)?;

    assert_eq!(v.len(), 16);

    Ok((i, Token::Transform(v)))
}

pub fn concat_transform<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, Token, E> {
    let (i, v) = preceded(
        preceded(tag("ConcatTransform"), sp),
        delimited(
            preceded(char('['), sp),
            nom::multi::many0(preceded(sp, float)),
            preceded(sp, char(']')),
        ),
    )(i)?;

    assert_eq!(v.len(), 16);

    Ok((i, Token::ConcatTransform(v)))
}

pub fn active_transform<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, Token, E> {
    let (i, v) = preceded(
        preceded(tag("ActiveTransform"), sp),
        nom::character::complete::alpha1,
    )(i)?;

    Ok((i, Token::ActiveTransform(v.to_owned())))
}

pub fn texture<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, Token, E> {
    let (i, _) = preceded(tag("Texture"), sp)(i)?;

    let (i, (name, t, class)) = nom::sequence::tuple((
        delimited(char('"'), parse_string, char('"')),
        preceded(sp, delimited(char('"'), parse_string, char('"'))),
        preceded(sp, delimited(char('"'), parse_string, char('"'))),
    ))(i)?;

    // Contains all the info
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
        Token::Texture {
            name: name.to_owned(),
            t: t.to_owned(),
            class: class.to_owned(),
            values,
        },
    ))
}

pub fn look_at<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, Token, E> {
    let (i, _) = preceded(tag("LookAt"), sp)(i)?;

    let (i, v) = nom::multi::many1(preceded(sp, nom::number::complete::float))(i)?;

    assert_eq!(v.len(), 9);

    Ok((
        i,
        Token::LookAt {
            eye: Vector3::new(v[0], v[1], v[2]),
            look: Vector3::new(v[3], v[4], v[5]),
            up: Vector3::new(v[6], v[7], v[8]),
        },
    ))
}

pub fn scale<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, Token, E> {
    let (i, _) = preceded(tag("Scale"), sp)(i)?;

    let (i, (v0, v1, v2)) = nom::sequence::tuple((
        nom::number::complete::float,
        preceded(sp, nom::number::complete::float),
        preceded(sp, nom::number::complete::float),
    ))(i)?;

    Ok((i, Token::Scale(Vector3::new(v0, v1, v2))))
}

pub fn translate<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, Token, E> {
    let (i, _) = preceded(tag("Translate"), sp)(i)?;

    let (i, (v0, v1, v2)) = nom::sequence::tuple((
        nom::number::complete::float,
        preceded(sp, nom::number::complete::float),
        preceded(sp, nom::number::complete::float),
    ))(i)?;

    Ok((i, Token::Translate(Vector3::new(v0, v1, v2))))
}

pub fn rotate<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, Token, E> {
    let (i, _) = preceded(tag("Rotate"), sp)(i)?;

    let (i, (angle, v0, v1, v2)) = nom::sequence::tuple((
        nom::number::complete::float,
        preceded(sp, nom::number::complete::float),
        preceded(sp, nom::number::complete::float),
        preceded(sp, nom::number::complete::float),
    ))(i)?;

    Ok((
        i,
        Token::Rotate {
            angle,
            axis: Vector3::new(v0, v1, v2),
        },
    ))
}

pub fn medium_interface<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, Token, E> {
    let (i, _) = preceded(tag("MediumInterface"), sp)(i)?;

    let (i, (inside, outside)) = preceded(
        sp,
        nom::sequence::tuple((
            delimited(char('"'), parse_string_empty, char('"')),
            preceded(sp, delimited(char('"'), parse_string_empty, char('"'))),
        )),
    )(i)?;

    Ok((
        i,
        Token::MediumInterface {
            inside: inside.to_owned(),
            outside: outside.to_owned(),
        },
    ))
}

pub fn parse_token<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, Token, E> {
    nom::branch::alt((
        nom::combinator::map(parse_named_token, |v| Token::NamedToken(v)),
        nom::combinator::map(parse_keyword, |v| Token::Keyword(v)),
        transform,
        texture,
        medium_interface,
        look_at,
        scale,
        concat_transform,
        translate,
        rotate,
        active_transform,
    ))(i)
}
