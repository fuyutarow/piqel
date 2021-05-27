use std::str::FromStr;

use partiql::lang::Lang;
use partiql::sql;
use partiql::sql::parser::math;
use partiql::sql::Expr;

fn main() -> anyhow::Result<()> {
    // let input = "a - b - c";
    // let r = sql::parser::parse_expr(&input);
    // dbg!(&r);

    let input = "1 - 2 - 3";
    let r = sql::parser::math::parse(&input);
    dbg!(&r);

    let input = r#"
{
    "a": 1,
    "b": 2,
    "c": 3
}
"#;

    let r = Lang::from_str(&input)?;
    dbg!(&r);
    // dbg!(eval(expr));
    // dbg!(eval(r));

    // let input = "3+5*(3+3)";
    // let r = math::expr(input)?;
    // dbg!(r);
    Ok(())
}
