use nom::branch::alt;
use nom::bytes::complete::{tag, tag_no_case};
use nom::character::complete::char;
use nom::character::complete::digit1;
use nom::combinator::opt;
use nom::error::ParseError;
use nom::multi::many1;
use nom::multi::separated_list0;
use nom::sequence::{preceded, tuple};
use nom::IResult;

use crate::sql::Proj;
use crate::sql::WhereCond;

pub use crate::parser::elements;
pub use crate::parser::expressions;
pub use crate::parser::parse_expr;
pub use crate::parser::parse_value;
pub use crate::parser::string_allowed_in_field;
pub use crate::parser::whitespace;
pub use crate::sql::clause;

pub fn select<'a>(input: &'a str) -> IResult<&'a str, Vec<Proj>> {
    let (input, vec_proj) = preceded(
        alt((tag("SELECT"), tag("select"))),
        preceded(
            whitespace,
            separated_list0(
                char(','),
                preceded(whitespace, many1(expressions::parse_proj)),
            ),
        ),
    )(input)?;

    let res = vec_proj.into_iter().flatten().collect::<Vec<_>>();
    Ok((input, res))
}

pub fn parse_where(input: &str) -> IResult<&str, WhereCond> {
    preceded(
        tag_no_case("WHERE"),
        alt((parse_where_eq, parse_where_like)),
    )(input)
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

pub fn parse_where_like(input: &str) -> IResult<&str, WhereCond> {
    let (input, (expr, _, s)) = preceded(
        whitespace,
        tuple((
            parse_expr,
            preceded(whitespace, tag_no_case("LIKE")),
            preceded(whitespace, elements::string),
        )),
    )(input)?;
    let res = WhereCond::Like {
        expr,
        right: s.to_string(),
    };
    Ok((input, res))
}

pub fn orderby(input: &str) -> IResult<&str, clause::OrderBy> {
    let (input, (_, field_name, asc_or_desc)) = tuple((
        tag_no_case("GROUP BY"),
        preceded(whitespace, string_allowed_in_field),
        alt((tag("ASC"), tag("asc"), tag("DESC"), tag("desc"))),
    ))(input)?;

    let is_asc = asc_or_desc.to_lowercase() == "asc";
    Ok((
        input,
        clause::OrderBy {
            label: field_name,
            is_asc,
        },
    ))
}

pub fn limit(input: &str) -> IResult<&str, clause::Limit> {
    let (input, (_, limit, opt_offset)) = tuple((
        tag_no_case("LIMIT"),
        preceded(whitespace, elements::integer),
        opt(preceded(whitespace, offset)),
    ))(input)?;

    let offset = opt_offset.unwrap_or(0);
    Ok((input, clause::Limit { limit, offset }))
}

pub fn offset(input: &str) -> IResult<&str, u64> {
    preceded(
        tag_no_case("OFFSET"),
        preceded(whitespace, elements::integer),
    )(input)
}
