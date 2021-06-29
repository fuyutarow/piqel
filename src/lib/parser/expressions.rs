use nom::branch::alt;
use nom::bytes::complete::{tag, tag_no_case};
use nom::character::complete::char;
use nom::combinator::map;
use nom::combinator::opt;
use nom::multi::separated_list1;
use nom::sequence::delimited;
use nom::sequence::{preceded, tuple};
use nom::IResult;

pub use crate::parser;
pub use crate::parser::elements;
pub use crate::parser::elements::string_allowed_in_field;
use crate::parser::select_statement::parse_sql;
pub use crate::parser::whitespace;
pub use crate::sql::clause;
use crate::sql::Expr;
use crate::sql::Field;
use crate::sql::Proj;
use crate::sql::Selector;
use crate::sql::SelectorNode;

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
    let (input, (path, alias)) = tuple((parse_selector, parse_alias_in_from_clause))(input)?;
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

pub fn parse_selector<'a>(input: &'a str) -> IResult<&'a str, Selector> {
    pub fn selecotrnode_with_index<'a>(input: &'a str) -> IResult<&'a str, Vec<SelectorNode>> {
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
    use crate::sql::Selector;
    use crate::sql::SelectorNode;

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
}
