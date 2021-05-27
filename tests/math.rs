use partiql::sql;

#[test]
fn calc() -> anyhow::Result<()> {
    let input = "1 - 2 - 3";
    let (_, expr) = sql::parser::math::parse(&input)?;
    assert_eq!(expr.eval(), -4.);

    let input = "12 - 34 + 15 - 9";
    let (_, expr) = sql::parser::math::parse(&input)?;
    assert_eq!(expr.eval(), -16.);

    let input = "1 * 2 + 3 / 4 ^ 6";
    let (_, expr) = sql::parser::math::parse(&input)?;
    assert_eq!(expr.eval() as u64, 2);

    let input = "(1 + 2) * 3";
    let (_, expr) = sql::parser::math::parse(&input)?;
    assert_eq!(expr.eval(), 9.);

    Ok(())
}