use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::char;
use nom::combinator::{cut, map, map_res, opt, value};
use nom::sequence::{preceded, terminated, tuple};
use nom::IResult;

use crate::sql::Expr;
use crate::sql::Func;
use crate::sql::Proj;

use crate::sql::parser::{parse_expr, whitespace};

pub fn count(input: &str) -> IResult<&str, Expr> {
    let (input, (_, expr)) = tuple((
        preceded(whitespace, tag("COUNT(")),
        cut(terminated(
            preceded(whitespace, parse_expr),
            preceded(whitespace, char(')')),
        )),
    ))(input)?;

    let res = Expr::Func(Box::new(Func::Count(expr)));

    Ok((input, res))
}
