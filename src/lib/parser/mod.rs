use nom::character::complete::multispace0;
use nom::character::complete::multispace1;
use nom::error::Error;
use nom::Err;
use nom::{
    branch::alt,
    bytes::complete::{escaped, tag, take_till, take_while, take_while_m_n},
    character::complete::{alphanumeric1, char, digit0, digit1, one_of, space0, space1},
    character::is_alphabetic,
    combinator::{cut, map, map_res, opt, value},
    error::{context, ContextError, ParseError},
    multi::{many0, many1, many_m_n, separated_list0, separated_list1},
    number::complete::{double, float, i64 as parse_i64},
    sequence::{delimited, preceded, separated_pair, terminated, tuple},
    IResult, Parser,
};
use ordered_float::OrderedFloat;

use crate::sql::DPath;
use crate::sql::Expr;
use crate::sql::Field;
use crate::sql::Proj;
use crate::sql::Sql;
use crate::sql::WhereCond;
use crate::value::PqlValue;

pub mod clauses;
pub mod elements;
pub mod expressions;
pub mod func;
pub mod keywords;
pub mod math;

pub use elements::{float_number, string_allowed_in_field, whitespace};
pub use expressions::parse_expr;
pub use expressions::parse_field;
pub use expressions::parse_path_as_expr;

pub fn sql(input: &str) -> anyhow::Result<Sql> {
    match parse_sql(input) {
        Ok((_, sql)) => Ok(sql),
        Err(nom::Err::Incomplete(needed)) => {
            anyhow::bail!("needed")
        }
        Err(nom::Err::Error(err)) => {
            dbg!(&err);
            eprintln!("{}", err);
            anyhow::bail!("parser error")
        }
        Err(nom::Err::Failure(err)) => {
            dbg!(&err);
            eprintln!("{}", err);
            anyhow::bail!("parse failed")
        }
    }
}

pub fn parse_sql(input: &str) -> IResult<&str, Sql> {
    let (
        input,
        (
            select_clause,
            opt_from_clause,
            opt_left_join_clause,
            opt_where_clause,
            opt_order_by,
            opt_limit,
        ),
    ) = tuple((
        preceded(multispace0, clauses::select),
        opt(preceded(multispace0, clauses::from)),
        opt(preceded(multispace0, clauses::left_join)),
        opt(preceded(multispace0, clauses::parse_where)),
        opt(preceded(multispace0, clauses::orderby)),
        opt(preceded(multispace0, clauses::limit)),
    ))(input)?;

    let sql = Sql {
        select_clause,
        from_clause: opt_from_clause.unwrap_or_default(),
        left_join_clause: opt_left_join_clause.unwrap_or_default(),
        where_clause: opt_where_clause.map(Box::new),
        orderby: opt_order_by,
        limit: opt_limit,
    };
    Ok((input, sql))
}

pub fn parse_value(input: &str) -> IResult<&str, PqlValue> {
    alt((
        map(elements::string, |s| PqlValue::Str(s.to_string())),
        map(double, |f| PqlValue::Float(OrderedFloat(f as f64))),
    ))(input)
}

pub fn array<'a>(input: &'a str) -> IResult<&'a str, Vec<u64>> {
    let (input, res) = context(
        "array",
        preceded(
            char('['),
            preceded(
                whitespace,
                cut(terminated(
                    separated_list0(char(','), preceded(whitespace, digit1)),
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
