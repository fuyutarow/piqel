use indexmap::IndexMap;
use nom::character::complete::digit1;
use nom::character::complete::multispace0;
use nom::combinator::recognize;
pub use nom::error::convert_error;
pub use nom::error::VerboseError;
use nom::error::{context, ContextError, ErrorKind, ParseError};
use nom::multi::many0;
use nom::number::complete::recognize_float;
use nom::sequence::tuple;
use nom::{
    branch::alt,
    bytes::complete::{escaped, tag},
    character::complete::{alphanumeric1, char, one_of, space1},
    combinator::{cut, map, map_res, opt, value},
    multi::separated_list0,
    number::complete::double,
    sequence::{delimited, preceded, separated_pair, terminated},
    IResult, Parser,
};
use ordered_float::OrderedFloat;

use crate::parser;
use crate::parser::elements;
use crate::value::PqlValue;

fn parse_str<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &'a str, E> {
    escaped(alt((alphanumeric1, space1)), '\\', one_of("\"n\\"))(i)
}

fn boolean<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, bool, E> {
    // This is a parser that returns `true` if it sees the string "true", and
    // an error otherwise
    let parse_true = value(true, tag("true"));

    // This is a parser that returns `false` if it sees the string "false", and
    // an error otherwise
    let parse_false = value(false, tag("false"));

    // `alt` combines the two parsers. It returns the result of the first
    // successful parser, or an error
    alt((parse_true, parse_false))(input)
}

fn null<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
    i: &'a str,
) -> IResult<&'a str, &'a str, E> {
    context("null", alt((tag("null"), tag("NULL"))))(i)
}

fn string<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
    i: &'a str,
) -> IResult<&'a str, &'a str, E> {
    context(
        "string",
        alt((
            preceded(char('"'), cut(terminated(parse_str, char('"')))),
            preceded(char('\''), cut(terminated(parse_str, char('\'')))),
        )),
    )(i)
}

fn bag<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
    i: &'a str,
) -> IResult<&'a str, Vec<PqlValue>, E> {
    context(
        "bag",
        preceded(
            tag("<<"),
            cut(terminated(
                separated_list0(preceded(multispace0, char(',')), json_value),
                preceded(multispace0, tag(">>")),
            )),
        ),
    )(i)
}

fn array<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
    i: &'a str,
) -> IResult<&'a str, Vec<PqlValue>, E> {
    context(
        "array",
        preceded(
            tag("["),
            cut(terminated(
                separated_list0(preceded(multispace0, char(',')), json_value),
                preceded(multispace0, tag("]")),
            )),
        ),
    )(i)
}

fn key_value<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
    i: &'a str,
) -> IResult<&'a str, (&'a str, PqlValue), E> {
    separated_pair(
        preceded(multispace0, string),
        cut(preceded(multispace0, char(':'))),
        json_value,
    )(i)
}

fn hash<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
    i: &'a str,
) -> IResult<&'a str, IndexMap<String, PqlValue>, E> {
    context(
        "map",
        preceded(
            char('{'),
            cut(terminated(
                map(
                    separated_list0(preceded(multispace0, char(',')), key_value),
                    |tuple_vec| {
                        tuple_vec
                            .into_iter()
                            .map(|(k, v)| (String::from(k), v))
                            .collect()
                    },
                ),
                preceded(multispace0, char('}')),
            )),
        ),
    )(i)
}

pub fn decimal1<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
    input: &'a str,
) -> IResult<&'a str, &'a str, E> {
    recognize(tuple((one_of("0123456789"), many0(one_of("_0123456789")))))(input)
}

pub fn float<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
    input: &'a str,
) -> IResult<&'a str, f64, E> {
    alt((
        // .42
        map(
            recognize(tuple((
                char('.'),
                decimal1,
                opt(tuple((one_of("eE"), opt(one_of("+-")), decimal1))),
            ))),
            |s| s.to_string().replace("_", "").parse::<f64>().unwrap(),
        ),
        // 42e42 and 42.42e42
        map(
            recognize(tuple((
                decimal1,
                opt(preceded(char('.'), decimal1)),
                tuple((one_of("eE"), opt(one_of("+-")), decimal1)),
            ))),
            |s| s.to_string().replace("_", "").parse::<f64>().unwrap(),
        ),
        // 42.
        map(
            recognize(tuple((decimal1, char('.'), opt(decimal1)))),
            |s| s.to_string().replace("_", "").parse::<f64>().unwrap(),
        ),
    ))(input)
}

pub fn integer<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
    input: &'a str,
) -> IResult<&'a str, i64, E> {
    map(recognize(tuple((opt(one_of("+-")), decimal1))), |s| {
        s.to_string().replace("_", "").parse::<i64>().unwrap()
    })(input)
}

pub fn json_value<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
    i: &'a str,
) -> IResult<&'a str, PqlValue, E> {
    preceded(
        multispace0,
        alt((
            map(null, |_s| PqlValue::Null),
            map(hash, PqlValue::Object),
            map(array, PqlValue::Array),
            map(bag, PqlValue::Array),
            map(string, |s| PqlValue::Str(String::from(s))),
            map(float, |f| PqlValue::Float(OrderedFloat(f as f64))),
            map(integer, |i| PqlValue::Int(i)),
            map(boolean, PqlValue::Boolean),
        )),
    )(i)
}

pub fn root<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
    i: &'a str,
) -> IResult<&'a str, PqlValue, E> {
    delimited(multispace0, json_value, opt(multispace0))(i)
}

pub fn pql_value(input: &str) -> anyhow::Result<PqlValue> {
    // let re = regex::Regex::new(r"(^|\n)\s*--[\w\s\.{}]*?\n").unwrap();
    let re = regex::Regex::new(r"--[\w\s\.{}]*?\n").unwrap();
    let input = re.replace_all(input, "");

    match root::<VerboseError<&str>>(&input) {
        Ok((_, r)) => Ok(r),
        Err(_err) => {
            anyhow::bail!("failed")
        }
    }
}

pub fn from_str(input: &str) -> anyhow::Result<PqlValue> {
    pql_value(input)
}

#[cfg(test)]
mod tests {
    use crate::value::PqlValue;

    use super::pql_value;
    use nom::number::complete::double;
    use nom::number::complete::float;
    use nom::number::complete::recognize_float;

    #[test]
    fn test_integer() -> anyhow::Result<()> {
        assert_eq!(pql_value("0000_000_00_0")?, PqlValue::from(0));
        assert_eq!(pql_value("12345")?, PqlValue::from(12345));
        assert_eq!(pql_value("+12345")?, PqlValue::from(12345));
        assert_eq!(pql_value("-12345")?, PqlValue::from(-12345));
        Ok(())
    }

    #[test]
    fn test_float() -> anyhow::Result<()> {
        assert_eq!(pql_value(".42")?, PqlValue::from(0.42));
        assert_eq!(pql_value("41.43")?, PqlValue::from(41.43));
        assert_eq!(pql_value("42.")?, PqlValue::from(42.));
        assert_eq!(pql_value("6.0")?, PqlValue::from(6.));
        assert_eq!(pql_value(".42e10")?, PqlValue::from(0.42e10));
        assert_eq!(pql_value(".42e+10")?, PqlValue::from(0.42e10));
        assert_eq!(pql_value(".42e-10")?, PqlValue::from(0.42e-10));
        assert_eq!(pql_value("41.43e10")?, PqlValue::from(41.43e10));
        assert_eq!(pql_value("41.43e+10")?, PqlValue::from(41.43e10));
        assert_eq!(pql_value("41.43e-10")?, PqlValue::from(41.43e-10));
        Ok(())
    }
}
