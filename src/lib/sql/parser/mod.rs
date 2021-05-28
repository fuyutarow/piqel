use indexmap::IndexMap;

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

use crate::sql::DPath;
use crate::sql::DWhereCond;
use crate::sql::Expr;
use crate::sql::Field;
use crate::sql::Proj;
use crate::sql::Sql;

pub mod math;

pub fn parse_path<'a>(input: &'a str) -> IResult<&'a str, DPath> {
    let (input, vec_path) = separated_list1(char('.'), string_allowed_in_field)(input)?;
    let res = DPath::from(vec_path.join(".").as_str());

    Ok((input, res))
}

pub fn parse_path_as_expr<'a>(input: &'a str) -> IResult<&'a str, Expr> {
    map(parse_path, |path| Expr::Path(path))(input)
}

pub fn parse_expr(input: &str) -> IResult<&str, Expr> {
    alt((parse_path_as_expr, math::parse))(input)
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

pub fn parse_sql<'a>(input: &'a str) -> IResult<&'a str, Sql> {
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

pub fn parse_proj<'a>(input: &'a str) -> IResult<&'a str, Proj> {
    let (input, (expr, alias)) = tuple((
        alt((
            parse_star_as_expr,
            preceded(
                preceded(whitespace, char('(')),
                cut(terminated(
                    preceded(whitespace, parse_sql_as_expr),
                    preceded(whitespace, char(')')),
                )),
            ),
            // delimited(
            //     char('('),
            //     delimited(whitespace, parse_sql_as_expr, whitespace),
            //     char(')'),
            // ),
            parse_path_as_expr,
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

pub fn parse_where<'a>(input: &'a str) -> IResult<&'a str, DWhereCond> {
    let (input, (path, op, value)) = preceded(
        tag("WHERE"),
        preceded(
            whitespace,
            tuple((
                parse_path,
                preceded(whitespace, alt((tag("="), tag("LIKE")))),
                preceded(whitespace, string),
            )),
        ),
    )(input)?;

    // dbg!(&field, &op, &value);

    // let cond = DWhereCond::default();
    let field = Field { path, alias: None };
    let right = value.to_string();
    let cond = match op {
        "=" => DWhereCond::Eq { field, right },
        "LIKE" => DWhereCond::Like { field, right },
        _ => unreachable!(),
    };

    Ok((input, cond))
}

pub fn _parse_where<'a>(input: &'a str) -> IResult<&'a str, DWhereCond> {
    let (input, (field, op, value)) = preceded(
        alt((tag("WHERE"), tag("where"))),
        preceded(
            whitespace,
            tuple((
                parse_field,
                preceded(whitespace, alt((tag("="), tag("LIKE"), tag("like")))),
                preceded(whitespace, string),
            )),
        ),
    )(input)?;

    let cond = match op {
        "=" => DWhereCond::Eq {
            field,
            right: value.to_string(),
        },
        "LIKE" | "like" => DWhereCond::Like {
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
