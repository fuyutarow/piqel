use nom::branch::alt;
use nom::character::complete::{char, digit1, space0};
use nom::combinator::map;
use nom::multi::many0;
use nom::sequence::{delimited, tuple};
use nom::IResult;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub enum Expr {
    Num(f64),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Exp(Box<Expr>, Box<Expr>),
}

pub fn parse(input: &str) -> IResult<&str, Expr> {
    parse_basic_expr(input)
}

fn parse_basic_expr(input: &str) -> IResult<&str, Expr> {
    parse_math_expr(input)
}

fn parse_parens(input: &str) -> IResult<&str, Expr> {
    delimited(
        space0,
        delimited(char('('), parse_math_expr, char(')')),
        space0,
    )(input)
}

fn parse_operation(input: &str) -> IResult<&str, Expr> {
    alt((parse_parens, parse_number))(input)
}

fn parse_factor(input: &str) -> IResult<&str, Expr> {
    let (input, num1) = parse_operation(input)?;
    let (input, exprs) = many0(tuple((char('^'), parse_factor)))(input)?;
    Ok((input, parse_expr(num1, exprs)))
}

fn parse_term(input: &str) -> IResult<&str, Expr> {
    let (input, num1) = parse_factor(input)?;
    let (input, exprs) = many0(tuple((alt((char('/'), char('*'))), parse_factor)))(input)?;
    Ok((input, parse_expr(num1, exprs)))
}

fn parse_math_expr(input: &str) -> IResult<&str, Expr> {
    let (input, num1) = parse_term(input)?;
    let (input, exprs) = many0(tuple((alt((char('+'), char('-'))), parse_term)))(input)?;
    Ok((input, parse_expr(num1, exprs)))
}

fn parse_expr(expr: Expr, rem: Vec<(char, Expr)>) -> Expr {
    rem.into_iter().fold(expr, |acc, val| parse_op(val, acc))
}

fn parse_op(tup: (char, Expr), expr1: Expr) -> Expr {
    let (op, expr2) = tup;
    match op {
        '+' => Expr::Add(Box::new(expr1), Box::new(expr2)),
        '-' => Expr::Sub(Box::new(expr1), Box::new(expr2)),
        '*' => Expr::Mul(Box::new(expr1), Box::new(expr2)),
        '/' => Expr::Div(Box::new(expr1), Box::new(expr2)),
        '^' => Expr::Exp(Box::new(expr1), Box::new(expr2)),
        _ => unreachable!(),
    }
}

fn parse_enum(parsed_num: &str) -> Expr {
    let num = f64::from_str(parsed_num).unwrap();
    Expr::Num(num)
}

fn parse_number(input: &str) -> IResult<&str, Expr> {
    map(delimited(space0, digit1, space0), parse_enum)(input)
}

#[cfg(test)]
mod tests {
    use super::parse;
    use super::Expr;

    #[test]
    fn parse_sub_sub() {
        let parsed = parse("1-2-3");
        assert_eq!(
            parsed,
            Ok((
                "",
                Expr::Sub(
                    Box::new(Expr::Sub(
                        Box::new(Expr::Num(1.0)),
                        Box::new(Expr::Num(2.0)),
                    )),
                    Box::new(Expr::Num(3.0)),
                )
            ))
        );
    }

    #[test]
    fn parse_add_statement() {
        let parsed = parse("12 + 34");
        assert_eq!(
            parsed,
            Ok((
                "",
                Expr::Add(Box::new(Expr::Num(12.0)), Box::new(Expr::Num(34.0)))
            ))
        );
    }

    #[test]
    fn parse_subtract_statement() {
        let parsed = parse("12 - 34");
        assert_eq!(
            parsed,
            Ok((
                "",
                Expr::Sub(Box::new(Expr::Num(12.0)), Box::new(Expr::Num(34.0)))
            ))
        );
    }

    #[test]
    fn parse_nested_add_sub_statements() {
        let parsed = parse("12 - 34 + 15 - 9");
        assert_eq!(
            parsed,
            Ok((
                "",
                Expr::Sub(
                    Box::new(Expr::Add(
                        Box::new(Expr::Sub(
                            Box::new(Expr::Num(12.0)),
                            Box::new(Expr::Num(34.0))
                        )),
                        Box::new(Expr::Num(15.0))
                    )),
                    Box::new(Expr::Num(9.0))
                )
            ))
        );
    }

    #[test]
    fn test_parse_multi_level_expression() {
        let parsed = parse("1 * 2 + 3 / 4 ^ 6");
        let expected = Expr::Add(
            Box::new(Expr::Mul(
                Box::new(Expr::Num(1.0)),
                Box::new(Expr::Num(2.0)),
            )),
            Box::new(Expr::Div(
                Box::new(Expr::Num(3.0)),
                Box::new(Expr::Exp(
                    Box::new(Expr::Num(4.0)),
                    Box::new(Expr::Num(6.0)),
                )),
            )),
        );
        assert_eq!(parsed, Ok(("", expected)));
    }

    #[test]
    fn test_parse_expression_with_parantheses() {
        let parsed = parse("(1 + 2) * 3");
        let expected = Expr::Mul(
            Box::new(Expr::Add(
                Box::new(Expr::Num(1.0)),
                Box::new(Expr::Num(2.0)),
            )),
            Box::new(Expr::Num(3.0)),
        );
        assert_eq!(parsed, Ok(("", expected)));
    }
}
