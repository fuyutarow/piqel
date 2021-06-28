use indexmap::IndexMap;
use ordered_float::OrderedFloat;

pub use nom::error::convert_error;
pub use nom::error::VerboseError;
use nom::{
    branch::alt,
    bytes::complete::{escaped, tag, take_while},
    character::complete::{
        alphanumeric1, char, one_of, space1,
    },
    combinator::{cut, map, opt, value},
    error::{context, ContextError, ParseError},
    multi::{separated_list0},
    number::complete::{double},
    sequence::{delimited, preceded, separated_pair, terminated},
    IResult, Parser,
};

use crate::value::PqlValue;

pub fn whitespace<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, &'a str, E> {
    let chars = " \t\r\n";
    take_while(move |c| chars.contains(c))(input)
}

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
                separated_list0(preceded(whitespace, char(',')), json_value),
                preceded(whitespace, tag(">>")),
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
                separated_list0(preceded(whitespace, char(',')), json_value),
                preceded(whitespace, tag("]")),
            )),
        ),
    )(i)
}

fn key_value<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
    i: &'a str,
) -> IResult<&'a str, (&'a str, PqlValue), E> {
    separated_pair(
        preceded(whitespace, string),
        cut(preceded(whitespace, char(':'))),
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
                    separated_list0(preceded(whitespace, char(',')), key_value),
                    |tuple_vec| {
                        tuple_vec
                            .into_iter()
                            .map(|(k, v)| (String::from(k), v))
                            .collect()
                    },
                ),
                preceded(whitespace, char('}')),
            )),
        ),
    )(i)
}

fn json_value<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
    i: &'a str,
) -> IResult<&'a str, PqlValue, E> {
    preceded(
        whitespace,
        alt((
            map(null, |_s| PqlValue::Null),
            map(hash, PqlValue::Object),
            map(array, PqlValue::Array),
            map(bag, PqlValue::Array),
            map(string, |s| PqlValue::Str(String::from(s))),
            map(double, |f| PqlValue::Float(OrderedFloat(f as f64))),
            map(boolean, PqlValue::Boolean),
        )),
    )(i)
}

pub fn root<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
    i: &'a str,
) -> IResult<&'a str, PqlValue, E> {
    delimited(whitespace, json_value, opt(whitespace))(i)
}

pub fn pql_model(input: &str) -> anyhow::Result<PqlValue> {
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
