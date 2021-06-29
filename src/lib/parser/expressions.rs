use collect_mac::collect;
use indexmap::IndexMap as Map;
use nom::branch::alt;
use nom::bytes::complete::{tag, tag_no_case};
use nom::character::complete::char;
use nom::character::complete::multispace0;
use nom::character::complete::{alphanumeric0, alphanumeric1};
use nom::combinator::map;
use nom::combinator::opt;
use nom::multi::separated_list1;
use nom::sequence::delimited;
use nom::sequence::preceded;
use nom::sequence::tuple;
use nom::IResult;

pub use crate::parser;
pub use crate::parser::elements;
pub use crate::parser::elements::string_allowed_in_field;
use crate::parser::select_statement::parse_sql;
pub use crate::parser::whitespace;
use crate::pqlir_parser;
pub use crate::sql::clause;
use crate::sql::Expr;
use crate::sql::Field;
use crate::sql::Proj;
use crate::sql::Selector;
use crate::sql::SelectorNode;
use crate::sql::SourceField;
use crate::value::PqlValue;

pub fn pqlvalue_with_alias(input: &str) -> IResult<&str, PqlValue> {
    let (input, (value, opt_alias)) = tuple((
        pqlir_parser::root,
        opt(preceded(
            opt(preceded(multispace0, tag_no_case("AS"))),
            preceded(multispace0, alphanumeric1),
        )),
    ))(input)?;

    let val = if let Some(alias) = opt_alias {
        PqlValue::Object(collect! {
            as Map::<String , PqlValue>:
            alias.to_string() => value
        })
    } else {
        value
    };
    Ok((input, val))
}
pub fn pqlvalue_with_alias_as_souceField(input: &str) -> IResult<&str, SourceField> {
    let (input, val) = pqlvalue_with_alias(input)?;
    Ok((input, SourceField::Value(val)))
}

pub fn selector_with_alias(input: &str) -> IResult<&str, SourceField> {
    let (input, (selector, opt_alias)) = tuple((
        parse_selector,
        opt(preceded(
            opt(preceded(multispace0, tag_no_case("AS"))),
            preceded(multispace0, alphanumeric1),
        )),
    ))(input)?;

    dbg!(&selector, opt_alias);

    let source = SourceField::Selector((selector, opt_alias.map(|e| e.to_owned())));

    Ok((input, source))
}

pub fn parse_sourcefield(input: &str) -> IResult<&str, SourceField> {
    dbg!("#1");
    alt((pqlvalue_with_alias_as_souceField, selector_with_alias))(input)
}

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
    map(tag("*"), |_| Expr::Star)(input)
}

pub fn parse_sql_as_expr(input: &str) -> IResult<&str, Expr> {
    map(parse_sql, |sql| Expr::Sql(sql))(input)
}

pub fn parse_field<'a>(input: &'a str) -> IResult<&'a str, Field> {
    let (input, (path, alias)) = tuple((parse_selector, opt(parse_alias_in_from_clause)))(input)?;
    let res = Field { path, alias };
    Ok((input, res))
}

pub fn parse_alias_in_from_clause(input: &str) -> IResult<&str, String> {
    let (input, (_, alias)) = tuple((
        opt(preceded(whitespace, tag_no_case("AS"))),
        preceded(whitespace, string_allowed_in_field),
    ))(input)?;
    Ok((input, alias))
}

pub fn parse_selector(input: &str) -> IResult<&str, Selector> {
    pub fn selecotrnode_with_index<'a>(input: &str) -> IResult<&str, Vec<SelectorNode>> {
        let (input, (s, opt_i)) = tuple((
            string_allowed_in_field,
            opt(delimited(char('['), elements::integer, char(']'))),
        ))(input)?;

        let mut nodes = vec![];
        nodes.push(SelectorNode::String(s));
        if let Some(i) = opt_i {
            nodes.push(SelectorNode::Number(i as i64));
        };

        Ok((input, nodes))
    }

    let (input, vec_nodes) = separated_list1(char('.'), selecotrnode_with_index)(input)?;
    let nodes = vec_nodes.into_iter().flatten().collect::<Vec<_>>();
    let res = Selector::from(nodes.as_slice());
    Ok((input, res))
}

pub fn parse_path_as_expr<'a>(input: &'a str) -> IResult<&'a str, Expr> {
    map(parse_selector, |path| Expr::Path(path))(input)
}

#[cfg(test)]
mod tests {
    use super::parse_selector;
    use crate::parser;
    use crate::sql::Selector;
    use crate::sql::SelectorNode;
    use crate::value::PqlValue;
    use std::str::FromStr;

    #[test]
    fn selector_xyz() -> anyhow::Result<()> {
        let (_, selector) = parse_selector("x.y.z")?;

        let expected = Selector::from(
            vec![
                SelectorNode::from("x"),
                SelectorNode::from("y"),
                SelectorNode::from("z"),
            ]
            .as_slice(),
        );

        assert_eq!(selector, expected);
        Ok(())
    }

    #[test]
    fn selector_xy2z() -> anyhow::Result<()> {
        let (_, selector) = parse_selector("x.y[2].z")?;

        let expected = Selector::from(
            vec![
                SelectorNode::from("x"),
                SelectorNode::from("y"),
                SelectorNode::from(2),
                SelectorNode::from("z"),
            ]
            .as_slice(),
        );

        assert_eq!(selector, expected);
        Ok(())
    }

    #[test]
    fn alias_pql_vlaue_to_pql_value() -> anyhow::Result<()> {
        let value = parser::expressions::pqlvalue_with_alias(r#"[1,2,3] AS arr"#)?.1;
        let expected = PqlValue::from_str(r#"{ "arr" : [1,2,3] }"#)?;
        assert_eq!(value, expected);
        Ok(())
    }
}
