use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::char;
use nom::character::complete::digit1;
use nom::combinator::map;
use nom::error::ParseError;
use nom::multi::many_m_n;
use nom::multi::separated_list0;
use nom::sequence::{preceded, tuple};
use nom::IResult;

pub use crate::parser::elements::integer;
pub use crate::parser::string_allowed_in_field;
pub use crate::parser::whitespace;
pub use crate::sql::clause;

pub fn orderby(input: &str) -> IResult<&str, clause::OrderBy> {
    let (input, (_, field_name, asc_or_desc)) = tuple((
        alt((tag("ORDER BY"), tag("order by"))),
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
    let (input, (_, limit, v_offset)) = tuple((
        alt((tag("LIMIT"), tag("limit"))),
        preceded(whitespace, integer),
        many_m_n(0, 1, preceded(whitespace, offset)),
    ))(input)?;

    let offset = v_offset.first().unwrap_or(&0).to_owned();
    Ok((input, clause::Limit { limit, offset }))
}

pub fn offset(input: &str) -> IResult<&str, u64> {
    preceded(
        alt((tag("OFFSET"), tag("offset"))),
        preceded(whitespace, integer),
    )(input)
}
