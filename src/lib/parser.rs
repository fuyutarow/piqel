use nom::{
    branch::alt,
    bytes::complete::{tag, take_till, take_while, take_while_m_n},
    character::complete::{char, digit0, digit1, multispace0, multispace1, one_of},
    character::is_alphabetic,
    combinator::{cut, map, map_res, opt},
    error::{context, ContextError, ParseError, VerboseError},
    multi::{many0, many_m_n, separated_list0},
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
        preceded(space, array),
    ))(input)?;
    dbg!(res);

    Ok((input, ()))
}

pub fn array<'a>(input: &'a str) -> IResult<&'a str, Vec<u64>> {
    let (input, res) = context(
        "array",
        preceded(
            char('['),
            cut(terminated(
                separated_list0(preceded(space, char(',')), digit1),
                preceded(space, char(']')),
            )),
        ),
    )(input)?;

    let r = res
        .into_iter()
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    Ok((input, r))
}
