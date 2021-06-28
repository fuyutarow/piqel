
use nom::bytes::complete::tag;
use nom::character::complete::char;
use nom::combinator::{cut};
use nom::sequence::{preceded, terminated, tuple};
use nom::IResult;

use crate::sql::Expr;
use crate::sql::Func;


use crate::parser::{parse_expr, whitespace};

pub fn count<'a>(input: &'a str) -> IResult<&'a str, Expr> {
    let name = "COUNT";
    let (input, (_, _, expr)) = tuple((
        preceded(whitespace, tag(name)),
        char('('),
        cut(terminated(
            preceded(whitespace, parse_expr),
            preceded(whitespace, char(')')),
        )),
    ))(input)?;

    let res = Expr::Func(Box::new(Func::Count(expr)));

    Ok((input, res))
}

pub fn upper<'a>(input: &'a str) -> IResult<&'a str, Expr> {
    let name = "UPPER";
    let (input, (_, _, expr)) = tuple((
        preceded(whitespace, tag(name)),
        char('('),
        cut(terminated(
            preceded(whitespace, parse_expr),
            preceded(whitespace, char(')')),
        )),
    ))(input)?;

    let res = Expr::Func(Box::new(Func::Upper(expr)));

    Ok((input, res))
}
