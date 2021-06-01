use nom::combinator::map;
use nom::error::{Error, ErrorKind, ParseError};
use nom::number::complete::recognize_float;
use nom::IResult;

use partiql::sql::parser;
use partiql::sql::Expr;

fn main() -> anyhow::Result<()> {
    let input = r#"
SELECT t.id AS id,
       x AS even
FROM matrices AS t,
     t.matrix AS y,
     y AS x
WHERE x % 2 = 0
  "#;
    let sql = parser::sql(&input)?;
    dbg!(&sql);

    let input = "x/2";
    let r = parser::parse_expr(input);
    dbg!(r);

    Ok(())
}
