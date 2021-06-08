use nom::{
    branch::alt,
    bytes::complete::{escaped, tag, take_till, take_while, take_while_m_n},
    character::complete::{
        alphanumeric1, char, digit0, digit1, multispace0, multispace1, one_of, space0, space1,
    },
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

pub fn parse_path<'a>(input: &'a str) -> IResult<&'a str, DPath> {
    let (input, vec_path) = separated_list1(char('.'), string_allowed_in_field)(input)?;
    let res = DPath::from(vec_path.join(".").as_str());

    Ok((input, res))
}

pub fn parse_path_as_expr<'a>(input: &'a str) -> IResult<&'a str, Expr> {
    map(parse_path, |path| Expr::Path(path))(input)
}

pub fn sql(input: &str) -> anyhow::Result<Sql> {
    match parse_sql(input) {
        Ok((_, sql)) => Ok(sql),
        Err(err) => {
            eprintln!("{}", err);
            anyhow::bail!("failed")
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
        preceded(whitespace, clauses::select),
        opt(preceded(whitespace, parse_from_clause)),
        opt(preceded(whitespace, parse_left_join)),
        opt(preceded(whitespace, clauses::parse_where)),
        opt(preceded(whitespace, clauses::orderby)),
        opt(preceded(whitespace, clauses::limit)),
    ))(input)?;

    let sql = Sql {
        select_clause,
        from_clause: opt_from_clause.unwrap_or_default(),
        left_join_clause: opt_left_join_clause.unwrap_or_default(),
        where_clause: opt_where_clause.map(Box::new),
        orderby: opt_order_by,
        limit: opt_limit,
    };
    dbg!(&input);
    Ok((input, sql))
}

pub fn parse_field<'a>(input: &'a str) -> IResult<&'a str, Field> {
    let (input, (path, alias)) = tuple((parse_path, parse_alias_in_from_clause))(input)?;
    let res = Field { path, alias };
    Ok((input, res))
}

pub fn parse_alias_in_from_clause(input: &str) -> IResult<&str, Option<String>> {
    let (input, vec) = many_m_n(
        0,
        1,
        tuple((
            preceded(whitespace, many_m_n(0, 1, alt((tag("AS"), tag("as"))))),
            preceded(whitespace, string_allowed_in_field),
        )),
    )(input)?;

    let alias = if let Some(as_alias) = vec.first() {
        let (_, alias) = as_alias;
        Some(alias.to_string())
    } else {
        None
    };

    Ok((input, alias))
}

pub fn parse_from_clause<'a>(input: &'a str) -> IResult<&'a str, Vec<Field>> {
    let (input, vec_fields) = preceded(
        alt((tag("FROM"), tag("from"))),
        preceded(
            whitespace,
            separated_list0(char(','), preceded(whitespace, many1(parse_field))),
        ),
    )(input)?;

    let res = vec_fields.into_iter().flatten().collect::<Vec<_>>();
    Ok((input, res))
}

pub fn parse_left_join<'a>(input: &'a str) -> IResult<&'a str, Vec<Field>> {
    let (input, vec_fields) = preceded(
        tag("LEFT JOIN"),
        preceded(
            whitespace,
            separated_list0(char(','), preceded(whitespace, many1(parse_field))),
        ),
    )(input)?;

    let fields = vec_fields.into_iter().flatten().collect::<Vec<_>>();
    Ok((input, fields))
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
