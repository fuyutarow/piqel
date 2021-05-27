use partiql::sql::parser::math;
use partiql::sql::parser::math::Expr;

pub fn eval(expr: Expr) -> f64 {
    match expr {
        Expr::Num(num) => num,
        Expr::Add(expr1, expr2) => eval(*expr1) + eval(*expr2),
        Expr::Sub(expr1, expr2) => eval(*expr1) - eval(*expr2),
        Expr::Mul(expr1, expr2) => eval(*expr1) * eval(*expr2),
        Expr::Div(expr1, expr2) => eval(*expr1) / eval(*expr2),
        Expr::Exp(expr1, expr2) => eval(*expr1).powf(eval(*expr2)),
    }
}

fn main() -> anyhow::Result<()> {
    let input = "3+5";
    let r = math::parse(input)?;
    dbg!(input, r);

    // let input = "3+5/3";
    // let r = math::expr(input)?;
    // dbg!(input, r);

    let input = "1-2-3";
    let (_, expr) = math::parse(input)?;
    dbg!(eval(expr));
    // dbg!(eval(r));

    // let input = "3+5*(3+3)";
    // let r = math::expr(input)?;
    // dbg!(r);
    Ok(())
}
