use std::collections::HashMap;

pub use nom::error::convert_error;
pub use nom::error::VerboseError;
use nom::{
    branch::alt,
    bytes::complete::{escaped, tag, take_till, take_while, take_while_m_n},
    character::complete::{
        alphanumeric1, char, digit0, digit1, multispace0, multispace1, one_of, space1,
    },
    character::is_alphabetic,
    combinator::{cut, map, map_res, opt, value},
    error::{context, ContextError, ParseError},
    multi::{many0, many_m_n, separated_list0},
    number::complete::{double, float, i64 as parse_i64},
    sequence::{delimited, preceded, separated_pair, terminated, tuple},
    IResult, Parser,
};

use crate::models::JsonValue;

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
    context("null", tag("null"))(i)
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

fn array<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
    i: &'a str,
) -> IResult<&'a str, Vec<JsonValue>, E> {
    context(
        "array",
        preceded(
            tag("<<"),
            cut(terminated(
                separated_list0(preceded(whitespace, char(',')), json_value),
                preceded(whitespace, tag(">>")),
            )),
        ),
    )(i)
}

fn key_value<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
    i: &'a str,
) -> IResult<&'a str, (&'a str, JsonValue), E> {
    separated_pair(
        preceded(whitespace, string),
        cut(preceded(whitespace, char(':'))),
        json_value,
    )(i)
}

fn hash<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
    i: &'a str,
) -> IResult<&'a str, HashMap<String, JsonValue>, E> {
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
) -> IResult<&'a str, JsonValue, E> {
    preceded(
        whitespace,
        alt((
            map(null, |s| JsonValue::Null),
            map(hash, JsonValue::Object),
            map(array, JsonValue::Array),
            map(string, |s| JsonValue::Str(String::from(s))),
            map(double, JsonValue::Num),
            map(boolean, JsonValue::Boolean),
        )),
    )(i)
}

pub fn root<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
    i: &'a str,
) -> IResult<&'a str, JsonValue, E> {
    delimited(
        whitespace,
        alt((map(hash, JsonValue::Object), map(array, JsonValue::Array))),
        opt(whitespace),
    )(i)
}

pub fn pql_model(input: &str) -> anyhow::Result<JsonValue> {
    let re = regex::Regex::new(r"(^|\n)\s*--[\w\s\.{}]*\n").unwrap();
    let input = re.replace_all(input, "");

    match root::<VerboseError<&str>>(&input) {
        Ok((_, r)) => Ok(r),
        Err(err) => {
            dbg!(err);
            anyhow::bail!("failed")
        }
    }
}
