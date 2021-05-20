use nom::{
    branch::alt,
    bytes::complete::{tag, take_till, take_while, take_while_m_n},
    character::complete::{digit0, digit1, multispace0, multispace1, one_of},
    character::is_alphabetic,
    combinator::{cut, map, map_res, opt},
    error::{context, ParseError, VerboseError},
    multi::{many0, many_m_n},
    number::complete::float,
    number::complete::i64 as parse_i64,
    sequence::{delimited, preceded, terminated, tuple},
    IResult, Parser,
};

pub fn space<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, &'a str, E> {
    let chars = " \t\r\n ";
    take_while(move |c| chars.contains(c))(input)
}

pub fn sql<'a>(input: &'a str) -> IResult<&'a str, ()> {
    let (input, res) = tuple((
        preceded(space, tag("SELECT")),
        preceded(space, tag("*")),
        preceded(space, tag("FROM")),
        preceded(space, tag("[1,2,3]")),
    ))(input)?;
    dbg!(res);

    Ok((input, ()))
}
