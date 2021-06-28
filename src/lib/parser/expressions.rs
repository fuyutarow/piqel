use nom::branch::alt;
use nom::bytes::complete::{tag, tag_no_case};
use nom::character::complete::char;
use nom::combinator::map;
use nom::combinator::opt;
use nom::multi::separated_list1;
use nom::sequence::delimited;
use nom::sequence::{preceded, tuple};
use nom::{IResult};

use crate::sql::DPath;
use crate::sql::Expr;
use crate::sql::Field;
use crate::sql::Proj;

pub use crate::parser;
pub use crate::parser::elements;
pub use crate::parser::elements::string_allowed_in_field;
use crate::parser::select_statement::parse_sql;
pub use crate::parser::whitespace;
pub use crate::sql::clause;

pub fn parse_proj<'a>(input: &'a str) -> IResult<&'a str, Proj> {
    let (input, (expr, opt_alias)) = tuple((
        alt((
            parse_expr,
            delimited(
                preceded(whitespace, char('(')),
                preceded(whitespace, parse_sql_as_expr),
                preceded(whitespace, char(')')),
            ),
            // preceded(
            //     preceded(whitespace, char('(')),
            //     cut(terminated(
            //         preceded(whitespace, parse_sql_as_expr),
            //         preceded(whitespace, char(')')),
            //     )),
            // ),
        )),
        opt(tuple((
            preceded(whitespace, tag_no_case("AS")),
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
    map(parse_sql, |sql| Expr::Sql(sql))(input)
}

pub fn parse_field<'a>(input: &'a str) -> IResult<&'a str, Field> {
    let (input, (path, alias)) = tuple((parse_path, parse_alias_in_from_clause))(input)?;
    let res = Field { path, alias };
    Ok((input, res))
}

pub fn parse_alias_in_from_clause(input: &str) -> IResult<&str, Option<String>> {
    let (input, as_alias) = opt(tuple((
        opt(preceded(whitespace, tag_no_case("AS"))),
        preceded(whitespace, string_allowed_in_field),
    )))(input)?;

    let alias = as_alias.map(|e| e.1);
    Ok((input, alias))
}

pub fn parse_path<'a>(input: &'a str) -> IResult<&'a str, DPath> {
    let (input, vec_path) = separated_list1(char('.'), string_allowed_in_field)(input)?;
    let res = DPath::from(vec_path.join(".").as_str());
    Ok((input, res))
}

pub fn parse_path_as_expr<'a>(input: &'a str) -> IResult<&'a str, Expr> {
    map(parse_path, |path| Expr::Path(path))(input)
}
