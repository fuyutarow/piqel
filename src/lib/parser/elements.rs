use nom::branch::alt;
use nom::bytes::complete::escaped;
use nom::bytes::complete::tag;
use nom::bytes::complete::take_while;
use nom::bytes::complete::take_while1;
use nom::character::complete::alphanumeric1;
use nom::character::complete::char;
use nom::character::complete::digit1;
use nom::character::complete::multispace0;
use nom::character::complete::one_of;
use nom::character::complete::space1;
use nom::character::is_alphanumeric;
use nom::combinator::cut;
use nom::combinator::map;
use nom::combinator::map_res;
use nom::combinator::not;
use nom::combinator::peek;
use nom::error::{context, ContextError, ErrorKind, ParseError};
use nom::multi::many1;
use nom::number::complete::recognize_float;
use nom::sequence::delimited;
use nom::sequence::tuple;
use nom::sequence::{preceded, terminated};
use nom::{IResult, InputLength};

use crate::parser::keywords::sql_keyword;
use crate::sql::Expr;

pub fn eof<I: Copy + InputLength, E: ParseError<I>>(input: I) -> IResult<I, I, E> {
    if input.input_len() == 0 {
        Ok((input, input))
    } else {
        Err(nom::Err::Error(E::from_error_kind(input, ErrorKind::Eof)))
    }
}

pub fn comma(input: &str) -> IResult<&str, &str> {
    delimited(multispace0, tag(","), multispace0)(input)
}

pub fn is_sql_identifier(chr: char) -> bool {
    is_alphanumeric(chr as u8) || chr == '_' || chr == '@'
}

/// Parses a SQL identifier (alphanumeric1 and "_").
pub fn sql_identifier(input: &str) -> IResult<&str, String> {
    map(
        alt((
            preceded(not(peek(sql_keyword)), take_while1(is_sql_identifier)),
            delimited(tag("`"), take_while1(is_sql_identifier), tag("`")),
            delimited(tag("["), take_while1(is_sql_identifier), tag("]")),
        )),
        String::from,
    )(input)
}

pub fn unsinged_integer<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
    input: &'a str,
) -> IResult<&'a str, u64, E> {
    map(digit1, |s: &str| s.parse::<u64>().unwrap())(input)
}

pub fn integer<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
    input: &'a str,
) -> IResult<&'a str, i64, E> {
    alt((
        map(digit1, |s: &str| s.parse::<i64>().unwrap()),
        map(
            preceded(tuple((char('-'), multispace0)), digit1),
            |s: &str| -s.parse::<i64>().expect("int"),
        ),
    ))(input)
}

pub fn integer_as_expr(input: &str) -> IResult<&str, Expr> {
    map(integer, |i| Expr::from(i as i64))(input)
}

/// Unlike nom::complete::{float, double}, this function does not parse `inf` keyword
pub fn float_number<'a>(input: &'a str) -> IResult<&'a str, Expr> {
    let (input, s) = recognize_float(input)?;
    match s.parse::<f64>() {
        Ok(f) => Ok((input, Expr::from(f))),
        Err(_) => Err(nom::Err::Error(ParseError::from_error_kind(
            input,
            ErrorKind::Float,
        ))),
    }
}

pub fn string<'a>(input: &'a str) -> IResult<&'a str, &'a str> {
    alt((
        preceded(char('"'), cut(terminated(parse_str, char('"')))),
        preceded(char('\''), cut(terminated(parse_str, char('\'')))),
    ))(input)
}

fn parse_str<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &'a str, E> {
    escaped(
        alt((alphanumeric1, space1, tag("_"), tag("%"))),
        '\\',
        one_of("\"n\\"),
    )(i)
}

#[cfg(test)]
mod tests {
    use super::float_number;
    use crate::sql::Expr;
    use crate::value::PqlValue;

    fn float(input: &str) -> anyhow::Result<Expr> {
        match float_number(input) {
            Ok((_, f)) => Ok(f),
            Err(_err) => anyhow::bail!("fail"),
        }
    }

    #[test]
    fn parse_float_number() -> anyhow::Result<()> {
        assert_eq!(float("3.4E3")?, Expr::Value(PqlValue::from(3.4e3)));

        Ok(())
    }
}
