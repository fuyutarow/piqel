use nom::character::complete::multispace0;
use nom::{
    branch::alt,
    combinator::opt,
    sequence::{preceded, tuple},
    IResult,
};

use crate::parser::clauses;
use crate::planner;

pub fn from_str(input: &str) -> anyhow::Result<planner::Sql> {
    match parse_planner_sql(input) {
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

pub fn parse_planner_sql(input: &str) -> IResult<&str, planner::Sql> {
    alt((parse_sql21, parse_sql22))(input)
}

pub fn parse_sql21(input: &str) -> IResult<&str, planner::Sql> {
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
        opt(preceded(multispace0, clauses::select2)),
        opt(preceded(multispace0, clauses::from)),
        opt(preceded(multispace0, clauses::left_join)),
        opt(preceded(multispace0, clauses::parse_where)),
        opt(preceded(multispace0, clauses::orderby)),
        opt(preceded(multispace0, clauses::limit)),
    ))(input)?;

    let sql = planner::Sql {
        select_clause: opt_select_clause.unwrap_or_default(),
        from_clause: opt_from_clause.unwrap_or_default(),
        left_join_clause: opt_left_join_clause.unwrap_or_default(),
        where_clause: opt_where_clause.map(Box::new),
        orderby: opt_order_by,
        limit: opt_limit,
    };
    Ok((input, sql))
}

pub fn parse_sql22(input: &str) -> IResult<&str, planner::Sql> {
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
        opt(preceded(multispace0, clauses::select2)),
        opt(preceded(multispace0, clauses::orderby)),
        opt(preceded(multispace0, clauses::limit)),
    ))(input)?;

    let sql = planner::Sql {
        select_clause: opt_select_clause.unwrap_or_default(),
        from_clause: opt_from_clause.unwrap_or_default(),
        left_join_clause: opt_left_join_clause.unwrap_or_default(),
        where_clause: opt_where_clause.map(Box::new),
        orderby: opt_order_by,
        limit: opt_limit,
    };
    Ok((input, sql))
}
