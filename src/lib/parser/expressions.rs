use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::bytes::complete::take_while;
use nom::character::complete::alphanumeric1;
use nom::character::complete::char;
use nom::character::complete::digit1;
use nom::combinator::cut;
use nom::combinator::map;
use nom::combinator::opt;
use nom::error::{ErrorKind, ParseError};
use nom::multi::many1;
use nom::number::complete::recognize_float;
use nom::sequence::{preceded, terminated, tuple};
use nom::{IResult, InputLength};

use crate::sql::DPath;
use crate::sql::Expr;
use crate::sql::Proj;

pub use crate::parser;
pub use crate::parser::elements::integer;
pub use crate::parser::string_allowed_in_field;
pub use crate::parser::whitespace;
pub use crate::sql::clause;

pub fn parse_proj<'a>(input: &'a str) -> IResult<&'a str, Proj> {
    let (input, (expr, opt_alias)) = tuple((
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
        opt(tuple((
            preceded(whitespace, alt((tag("AS"), tag("as")))),
            preceded(whitespace, string_allowed_in_field),
        ))),
    ))(input)?;

    let res = Proj {
        expr,
        alias: opt_alias.map(|e| e.1),
    };
    Ok((input, res))
}

pub fn parse_expr(input: &str) -> IResult<&str, Expr> {
    // The math::parse must be placed after the parse_path_as_expr to prevent the inf keyword from being parsed.
    alt((
        parse_star_as_expr,
        parser::math::parse,
        parser::elements::float_number,
        parser::func::count,
    ))(input)
}

pub fn parse_star_as_expr(input: &str) -> IResult<&str, Expr> {
    map(tag("*"), |_| Expr::Path(DPath::from("*")))(input)
}

pub fn parse_sql_as_expr(input: &str) -> IResult<&str, Expr> {
    map(parser::parse_sql, |sql| Expr::Sql(sql))(input)
}
