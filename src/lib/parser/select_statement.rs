use nom::bytes::complete::tag_no_case;
use nom::bytes::complete::take_while1;
use nom::character::complete::alphanumeric1;
use nom::character::complete::char;
use nom::character::complete::multispace0;
use nom::combinator::map;
use nom::combinator::not;
use nom::combinator::peek;
use nom::sequence::delimited;
use nom::sequence::preceded;
use nom::sequence::tuple;
use nom::{branch::alt, combinator::opt, IResult};

use crate::parser::clauses;
use crate::parser::elements;
use crate::sql::Expr;
use crate::sql::Field;
use crate::sql::Sql;

pub fn from_str(input: &str) -> anyhow::Result<Sql> {
    match parse_sql(input) {
        Ok((_, sql)) => Ok(sql),
        Err(nom::Err::Incomplete(_needed)) => {
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

pub fn parse_sql_as_field(input: &str) -> IResult<&str, Field> {
    map(
        tuple((
            delimited(
                preceded(multispace0, char('(')),
                preceded(multispace0, parse_sql),
                preceded(multispace0, char(')')),
            ),
            preceded(multispace0, tag_no_case("AS")),
            preceded(multispace0, elements::sql_identifier),
        )),
        |(sql, _, alias)| Field {
            expr: Expr::Sql(sql),
            alias: Some(alias.to_string()),
        },
    )(input)
}

pub fn parse_sql(input: &str) -> IResult<&str, Sql> {
    alt((parse_sql21, parse_sql22))(input)
}

pub fn parse_sql21(input: &str) -> IResult<&str, Sql> {
    let (
        input,
        (
            opt_select_clause,
            opt_from_clause,
            opt_left_join_clause,
            opt_where_clause,
            opt_order_by,
            opt_limit,
        ),
    ) = tuple((
        opt(preceded(multispace0, clauses::select)),
        opt(preceded(multispace0, clauses::from)),
        opt(preceded(multispace0, clauses::left_join)),
        opt(preceded(multispace0, clauses::parse_where)),
        opt(preceded(multispace0, clauses::orderby)),
        opt(preceded(multispace0, clauses::limit)),
    ))(input)?;

    let sql = Sql {
        select_clause: opt_select_clause.unwrap_or_default(),
        from_clause: opt_from_clause.unwrap_or_default(),
        left_join_clause: opt_left_join_clause.unwrap_or_default(),
        where_clause: opt_where_clause.map(Box::new),
        orderby: opt_order_by,
        limit: opt_limit,
    };
    Ok((input, sql))
}

pub fn parse_sql22(input: &str) -> IResult<&str, Sql> {
    let (
        input,
        (
            opt_from_clause,
            opt_left_join_clause,
            opt_where_clause,
            opt_select_clause,
            opt_order_by,
            opt_limit,
        ),
    ) = tuple((
        opt(preceded(multispace0, clauses::from)),
        opt(preceded(multispace0, clauses::left_join)),
        opt(preceded(multispace0, clauses::parse_where)),
        opt(preceded(multispace0, clauses::select)),
        opt(preceded(multispace0, clauses::orderby)),
        opt(preceded(multispace0, clauses::limit)),
    ))(input)?;

    let sql = Sql {
        select_clause: opt_select_clause.unwrap_or_default(),
        from_clause: opt_from_clause.unwrap_or_default(),
        left_join_clause: opt_left_join_clause.unwrap_or_default(),
        where_clause: opt_where_clause.map(Box::new),
        orderby: opt_order_by,
        limit: opt_limit,
    };
    Ok((input, sql))
}
