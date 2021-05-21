use nom::{
    branch::alt,
    bytes::complete::{escaped, tag, take_till, take_while, take_while_m_n},
    character::complete::{
        alphanumeric1, char, digit0, digit1, multispace0, multispace1, one_of, space1,
    },
    character::is_alphabetic,
    combinator::{cut, map, map_res, opt, value},
    error::{context, ContextError, ParseError},
    multi::{many0, many1, many_m_n, separated_list0, separated_list1},
    number::complete::{double, float, i64 as parse_i64},
    sequence::{delimited, preceded, separated_pair, terminated, tuple},
    IResult, Parser,
};

pub fn whitespace<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, &'a str, E> {
    let chars = " \t\r\n";
    take_while(move |c| chars.contains(c))(input)
}

pub fn sql<'a>(input: &'a str) -> IResult<&'a str, ()> {
    let (input, res) = tuple((
        preceded(whitespace, parse_select),
        preceded(whitespace, parse_from),
        preceded(whitespace, parse_where),
    ))(input)?;
    dbg!(res);

    Ok((input, ()))
}

fn parse_str<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &'a str, E> {
    escaped(
        alt((alphanumeric1, space1, tag("%"))),
        '\\',
        one_of("\"n\\"),
    )(i)
}

pub fn field<'a>(input: &'a str) -> IResult<&'a str, ()> {
    let (input, res) = tuple((alphanumeric1, char('.'), alphanumeric1))(input)?;

    dbg!(res);

    Ok((input, ()))
}

pub fn field_with<'a>(input: &'a str) -> IResult<&'a str, ()> {
    let (input, res) = tuple((
        alphanumeric1,
        char('.'),
        alphanumeric1,
        many_m_n(
            0,
            1,
            tuple((
                preceded(whitespace, many_m_n(0, 1, tag("AS"))),
                preceded(whitespace, alphanumeric1),
            )),
        ),
    ))(input)?;

    dbg!(res);

    Ok((input, ()))
}

pub fn parse_select<'a>(input: &'a str) -> IResult<&'a str, ()> {
    let (input, res) = preceded(
        tag("SELECT"),
        preceded(
            whitespace,
            separated_list0(char(','), preceded(whitespace, many1(field_with))),
        ),
    )(input)?;
    dbg!(res);

    Ok((input, ()))
}

pub fn parse_from<'a>(input: &'a str) -> IResult<&'a str, ()> {
    let (input, res) = preceded(
        tag("FROM"),
        preceded(
            whitespace,
            separated_list0(char(','), preceded(whitespace, many1(field_with))),
        ),
    )(input)?;
    dbg!(res);

    Ok((input, ()))
}

pub fn string<'a>(input: &'a str) -> IResult<&'a str, ()> {
    let (input, res) = alt((
        preceded(char('"'), cut(terminated(parse_str, char('"')))),
        preceded(char('\''), cut(terminated(parse_str, char('\'')))),
    ))(input)?;
    dbg!(res);

    Ok((input, ()))
}

pub fn parse_where<'a>(input: &'a str) -> IResult<&'a str, ()> {
    let (input, res) = preceded(
        tag("WHERE"),
        preceded(
            whitespace,
            tuple((
                field,
                preceded(whitespace, alt((tag("="), tag("LIKE")))),
                preceded(whitespace, string),
            )),
        ),
    )(input)?;
    dbg!(res);

    Ok((input, ()))
}

pub fn array<'a>(input: &'a str) -> IResult<&'a str, Vec<u64>> {
    let (input, res) = context(
        "array",
        preceded(
            char('['),
            preceded(
                whitespace,
                cut(terminated(
                    separated_list0(
                        // preceded(whitespace, char(',')),
                        char(','),
                        preceded(whitespace, digit1),
                    ),
                    preceded(whitespace, char(']')),
                )),
            ),
        ),
    )(input)?;

    let r = res
        .into_iter()
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    Ok((input, r))
}
