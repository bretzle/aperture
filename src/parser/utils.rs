use nom::{bytes::complete::tag, error::ParseError, IResult};

/// parser combinators are constructed from the bottom up:
/// first we write parsers for the smallest elements (here a space character),
/// then we'll combine them in larger parsers
pub fn sp<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &'a str, E> {
    let chars = " \t\r\n";

    // nom combinators like `take_while` return a function. That function is the
    // parser,to which we can pass the input
    let (i, v) = nom::bytes::complete::take_while(move |c| chars.contains(c))(i)?;

    // Check if we have seen a #
    match tag::<&str, &str, E>("#")(i) {
        Ok((i, _)) => {
            let (i, _) = nom::bytes::complete::take_while(move |c| c != '\n')(i)?;
            sp(i)
        }
        Err(_) => Ok((i, v)),
    }
}

pub fn parse_string_empty<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &'a str, E> {
    nom::bytes::complete::take_while(move |c| c != '"')(i)
}

pub fn parse_string<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &'a str, E> {
    nom::bytes::complete::take_while1(move |c| c != '"')(i)
}

pub fn parse_string_sp<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &'a str, E> {
    nom::bytes::complete::take_while1(move |c| c != ' ')(i)
}
