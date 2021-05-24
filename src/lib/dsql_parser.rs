use std::collections::HashMap;

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

use crate::sql::DField as Field;
use crate::sql::DSql as Sql;
use crate::sql::DWhereCond as WhereCond;
use crate::sql::Dpath;

pub fn whitespace<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, &'a str, E> {
    let chars = " \t\r\n";
    take_while(move |c| chars.contains(c))(input)
}

pub fn sql(input: &str) -> anyhow::Result<Sql> {
    match parse_sql(input) {
        Ok((_, sql)) => Ok(sql),
        Err(err) => {
            dbg!(err);
            anyhow::bail!("failed")
        }
    }
}

pub fn parse_sql<'a>(input: &'a str) -> IResult<&'a str, Sql> {
    let (input, (select_clause, from_clause, vec_left_join_clause, vec_where_clause)) =
        tuple((
            preceded(whitespace, parse_select),
            preceded(whitespace, parse_from),
            many_m_n(0, 1, preceded(whitespace, parse_left_join)),
            many_m_n(0, 1, preceded(whitespace, parse_where)),
        ))(input)?;

    let sql = Sql {
        select_clause,
        from_clause,
        left_join_clause: vec_left_join_clause.first().unwrap_or(&vec![]).to_owned(),
        where_clause: if let Some(cond) = vec_where_clause.first() {
            Some(cond.to_owned())
        } else {
            None
        },
    };
    Ok((input, sql))
}

fn parse_str<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &'a str, E> {
    escaped(
        alt((alphanumeric1, space1, tag("%"))),
        '\\',
        one_of("\"n\\"),
    )(i)
}

pub fn parse_field<'a>(input: &'a str) -> IResult<&'a str, Field> {
    let (input, (p1, _, p2)) = tuple((alphanumeric1, char('.'), alphanumeric1))(input)?;

    let f = Field {
        path: Dpath::from(format!("{}.{}", p1, p2).as_str()),
        alias: None,
    };
    Ok((input, f))
}

pub fn field_with<'a>(input: &'a str) -> IResult<&'a str, Field> {
    let (input, (p1, _, p2, vec_as_alias)) = tuple((
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

    let alias = {
        if let Some(as_alias) = vec_as_alias.first() {
            let (_, alias) = as_alias;
            Some(alias.to_string())
        } else {
            None
        }
    };

    let f = Field {
        path: Dpath::from(format!("{}.{}", p1, p2).as_str()),
        alias,
    };
    Ok((input, f))
}

pub fn parse_select<'a>(input: &'a str) -> IResult<&'a str, Vec<Field>> {
    let (input, vec_fields) = preceded(
        tag("SELECT"),
        preceded(
            whitespace,
            separated_list0(char(','), preceded(whitespace, many1(field_with))),
        ),
    )(input)?;

    let fields = vec_fields.into_iter().flatten().collect::<Vec<_>>();
    Ok((input, fields))
}

pub fn parse_from<'a>(input: &'a str) -> IResult<&'a str, Vec<Field>> {
    let (input, vec_fields) = preceded(
        tag("FROM"),
        preceded(
            whitespace,
            separated_list0(char(','), preceded(whitespace, many1(field_with))),
        ),
    )(input)?;

    let fields = vec_fields.into_iter().flatten().collect::<Vec<_>>();
    Ok((input, fields))
}

pub fn parse_left_join<'a>(input: &'a str) -> IResult<&'a str, Vec<Field>> {
    let (input, vec_fields) = preceded(
        tag("LEFT JOIN"),
        preceded(
            whitespace,
            separated_list0(char(','), preceded(whitespace, many1(field_with))),
        ),
    )(input)?;

    let fields = vec_fields.into_iter().flatten().collect::<Vec<_>>();
    Ok((input, fields))
}

pub fn string<'a>(input: &'a str) -> IResult<&'a str, &'a str> {
    alt((
        preceded(char('"'), cut(terminated(parse_str, char('"')))),
        preceded(char('\''), cut(terminated(parse_str, char('\'')))),
    ))(input)
}

pub fn parse_where<'a>(input: &'a str) -> IResult<&'a str, WhereCond> {
    let (input, (field, op, value)) = preceded(
        tag("WHERE"),
        preceded(
            whitespace,
            tuple((
                parse_field,
                preceded(whitespace, alt((tag("="), tag("LIKE")))),
                preceded(whitespace, string),
            )),
        ),
    )(input)?;

    let cond = match op {
        "=" => WhereCond::Eq {
            field,
            right: value.to_string(),
        },
        "LIKE" => WhereCond::Like {
            field,
            right: value.to_string(),
        },
        _ => unreachable!(),
    };

    Ok((input, cond))
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
        .iter()
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    Ok((input, r))
}
