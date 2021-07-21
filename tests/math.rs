use std::convert::TryFrom;

use piqel::parser;
use piqel::sql::Env;
use piqel::value::PqlValue;

#[test]
fn calc() -> anyhow::Result<()> {
    let input = "1 - 2 - 3";
    let (_, expr) = parser::math::parse(&input)?;
    assert_eq!(expr.eval(&Env::default()), PqlValue::from(-4));

    let input = "12 - 34 + 15 - 9";
    let (_, expr) = parser::math::parse(&input)?;
    assert_eq!(expr.eval(&Env::default()), PqlValue::from(-16));

    let input = "1 * 2 + 3 / 4 ^ 6";
    let (_, expr) = parser::math::parse(&input)?;
    assert_eq!(i64::try_from(expr.eval(&Env::default()))?, 2);

    let input = "(1 + 2) * 3";
    let (_, expr) = parser::math::parse(&input)?;
    assert_eq!(expr.eval(&Env::default()), PqlValue::from(9));

    Ok(())
}
