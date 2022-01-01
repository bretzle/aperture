use nom::{bytes::complete::tag, error::ParseError, IResult};

#[derive(Debug, Clone)]
pub enum Keyword {
    AttributeBegin,
    AttributeEnd,
    Identity,
    ObjectEnd,
    ReverseOrientation,
    TransformBegin,
    TransformEnd,
    WorldBegin,
    WorldEnd,
}

pub fn parse_keyword<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, Keyword, E> {
    nom::branch::alt((
        nom::combinator::map(tag("AttributeBegin"), |_| Keyword::AttributeBegin),
        nom::combinator::map(tag("AttributeEnd"), |_| Keyword::AttributeEnd),
        nom::combinator::map(tag("Identity"), |_| Keyword::Identity),
        nom::combinator::map(tag("ReverseOrientation"), |_| Keyword::ReverseOrientation),
        nom::combinator::map(tag("TransformBegin"), |_| Keyword::TransformBegin),
        nom::combinator::map(tag("TransformEnd"), |_| Keyword::TransformEnd),
        nom::combinator::map(tag("WorldBegin"), |_| Keyword::WorldBegin),
        nom::combinator::map(tag("WorldEnd"), |_| Keyword::WorldEnd),
        nom::combinator::map(tag("ObjectEnd"), |_| Keyword::ObjectEnd),
    ))(i)
}
