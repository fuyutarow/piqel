use nom::character::complete::multispace0;
use nom::{
    branch::alt,
    combinator::opt,
    sequence::{preceded, tuple},
    IResult,
};

use crate::sql::Sql;

use crate::parser::clauses;

pub fn sql(input: &str) -> anyhow::Result<Sql> {
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

pub fn parse_sql(input: &str) -> IResult<&str, Sql> {
    alt((parse_sql1, parse_sql2))(input)
}

pub fn parse_sql1(input: &str) -> IResult<&str, Sql> {
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

pub fn parse_sql2(input: &str) -> IResult<&str, Sql> {
    let (
        input,
        (
            opt_from_clause,
            opt_left_join_clause,
            opt_where_clause,
            select_clause,
            opt_order_by,
            opt_limit,
        ),
    ) = tuple((
        opt(preceded(multispace0, clauses::from)),
        opt(preceded(multispace0, clauses::left_join)),
        opt(preceded(multispace0, clauses::parse_where)),
        preceded(multispace0, clauses::select),
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

use crate::sql::Proj;
use crate::value::PqlValue;

#[derive(Debug, Default)]
pub struct LogicalPlan {
    pub select: Vec<Proj>,
    pub from: PqlValue,
}

pub fn parse_sql3(input: &str) -> IResult<&str, LogicalPlan> {
    let (input, (select_clause, opt_from_clause)) = tuple((
        preceded(multispace0, clauses::select),
        opt(preceded(multispace0, clauses::from_pql_value)),
    ))(input)?;

    let sql = LogicalPlan {
        select: select_clause,
        from: opt_from_clause.unwrap_or_default(),
    };
    Ok((input, sql))
}
