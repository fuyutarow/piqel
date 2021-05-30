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

pub mod func;
pub mod math;

pub fn parse_path<'a>(input: &'a str) -> IResult<&'a str, DPath> {
    let (input, vec_path) = separated_list1(char('.'), string_allowed_in_field)(input)?;
    let res = DPath::from(vec_path.join(".").as_str());

    Ok((input, res))
}

pub fn parse_path_as_expr<'a>(input: &'a str) -> IResult<&'a str, Expr> {
    map(parse_path, |path| Expr::Path(path))(input)
}

pub fn whitespace<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, &'a str, E> {
    let chars = " \t\r\n";
    take_while(move |c| chars.contains(c))(input)
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
    let (input, (select_clause, vec_from_clause, vec_left_join_clause, vec_where_clause)) =
        tuple((
            preceded(whitespace, parse_select_clause),
            many_m_n(0, 1, preceded(whitespace, parse_from_clause)),
            many_m_n(0, 1, preceded(whitespace, parse_left_join)),
            many_m_n(0, 1, preceded(whitespace, parse_where)),
        ))(input)?;
    dbg!(&vec_where_clause);

    let sql = Sql {
        select_clause,
        from_clause: vec_from_clause.first().unwrap_or(&vec![]).to_owned(),
        left_join_clause: vec_left_join_clause.first().unwrap_or(&vec![]).to_owned(),
        where_clause: if let Some(cond) = vec_where_clause.first() {
            Some(Box::new(cond.to_owned()))
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

pub fn string_allowed_in_field<'a>(input: &'a str) -> IResult<&'a str, String> {
    let (input, ss) = many1(alt((alphanumeric1, tag("_"))))(input)?;

    Ok((input, ss.into_iter().collect::<String>()))
}

pub fn parse_field<'a>(input: &'a str) -> IResult<&'a str, Field> {
    let (input, (path, alias)) = tuple((parse_path, parse_alias_in_from_clause))(input)?;
    let res = Field { path, alias };
    Ok((input, res))
}

pub fn parse_alias_in_select_clause(input: &str) -> IResult<&str, Option<String>> {
    let (input, vec) = many_m_n(
        0,
        1,
        tuple((
            preceded(whitespace, alt((tag("AS"), tag("as")))),
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

pub fn parse_sql_as_expr(input: &str) -> IResult<&str, Expr> {
    map(parse_sql, |sql| Expr::Sql(sql))(input)
}

pub fn parse_star_as_expr(input: &str) -> IResult<&str, Expr> {
    map(tag("*"), |_| Expr::Path(DPath::from("*")))(input)
}

pub fn parse_number(input: &str) -> IResult<&str, Expr> {
    map(double, |f| Expr::Num(f as f64))(input)
}

pub fn parse_expr(input: &str) -> IResult<&str, Expr> {
    alt((
        math::parse,
        parse_star_as_expr,
        parse_number,
        func::count,
        parse_path_as_expr,
    ))(input)
}

pub fn parse_proj<'a>(input: &'a str) -> IResult<&'a str, Proj> {
    let (input, (expr, alias)) = tuple((
        alt((
            parse_expr,
            preceded(
                preceded(whitespace, char('(')),
                cut(terminated(
                    preceded(whitespace, parse_sql_as_expr),
                    preceded(whitespace, char(')')),
                )),
            ),
        )),
        parse_alias_in_select_clause,
    ))(input)?;
    let res = Proj { expr, alias };
    Ok((input, res))
}

pub fn parse_select_clause<'a>(input: &'a str) -> IResult<&'a str, Vec<Proj>> {
    let (input, vec_proj) = preceded(
        alt((tag("SELECT"), tag("select"))),
        preceded(
            whitespace,
            separated_list0(char(','), preceded(whitespace, many1(parse_proj))),
        ),
    )(input)?;

    let res = vec_proj.into_iter().flatten().collect::<Vec<_>>();
    Ok((input, res))
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

pub fn string<'a>(input: &'a str) -> IResult<&'a str, &'a str> {
    alt((
        preceded(char('"'), cut(terminated(parse_str, char('"')))),
        preceded(char('\''), cut(terminated(parse_str, char('\'')))),
    ))(input)
}

pub fn parse_value(input: &str) -> IResult<&str, PqlValue> {
    alt((
        map(string, |s| PqlValue::Str(s.to_string())),
        map(double, |f| PqlValue::Float(OrderedFloat(f as f64))),
    ))(input)
}

pub fn parse_where_like(input: &str) -> IResult<&str, WhereCond> {
    let (input, (expr, _, s)) = preceded(
        whitespace,
        tuple((
            parse_expr,
            preceded(whitespace, tag("LIKE")),
            preceded(whitespace, string),
        )),
    )(input)?;

    let res = WhereCond::Like {
        expr,
        right: s.to_string(),
    };

    Ok((input, res))
}

pub fn parse_where_eq(input: &str) -> IResult<&str, WhereCond> {
    let (input, (expr, _, right)) = preceded(
        whitespace,
        tuple((
            parse_expr,
            preceded(whitespace, tag("=")),
            preceded(whitespace, parse_value),
        )),
    )(input)?;

    let res = WhereCond::Eq { expr, right };

    Ok((input, res))
}

pub fn parse_where<'a>(input: &'a str) -> IResult<&'a str, WhereCond> {
    preceded(tag("WHERE"), alt((parse_where_eq, parse_where_like)))(input)
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
