use nom::bytes::complete::take_while;
use nom::character::complete::digit1;
use nom::error::{ErrorKind, ParseError};
use nom::number::complete::recognize_float;
use nom::IResult;

use crate::sql::Expr;

pub fn whitespace<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, &'a str, E> {
    let chars = " \t\r\n";
    take_while(move |c| chars.contains(c))(input)
}

pub fn integer<'a>(input: &'a str) -> IResult<&'a str, u64> {
    let (input, s) = digit1(input)?;
    match s.parse::<u64>() {
        Ok(i) => Ok((input, i)),
        Err(_) => Err(nom::Err::Error(ParseError::from_error_kind(
            input,
            ErrorKind::Float,
        ))),
    }
}

// Unlike nom::complete::{float, double}, this function does not parse `inf` keyword
pub fn float_number<'a>(input: &'a str) -> IResult<&'a str, Expr> {
    let (input, s) = recognize_float(input)?;
    match s.parse::<f64>() {
        Ok(f) => Ok((input, Expr::Num(f))),
        Err(_) => Err(nom::Err::Error(ParseError::from_error_kind(
            input,
            ErrorKind::Float,
        ))),
    }
}

#[cfg(test)]
mod tests {
    use super::float_number;
    use crate::sql::Expr;

    fn float(input: &str) -> anyhow::Result<Expr> {
        match float_number(input) {
            Ok((_, f)) => Ok(f),
            Err(err) => anyhow::bail!("fail"),
        }
    }

    #[test]
    fn parse_float_number() -> anyhow::Result<()> {
        assert_eq!(float("3.4E3")?, Expr::Num(3.4e3));

        Ok(())
    }
}
