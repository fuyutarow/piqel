
use partiql::sql::parser;

fn main() -> anyhow::Result<()> {
  let input =r#"
SELECT t.id AS id,
       x AS even
FROM matrices AS t,
     t.matrix AS y,
     y AS x
WHERE x % 2 = 0
  "#;

  let input =r#"
SELECT t.id AS id,
       x AS even
FROM matrices AS t,
     t.matrix AS y,
     y AS x
WHERE x/2 = 0
  "#;
    let sql = parser::sql(&input)?;
    dbg!(&sql);



  Ok(())
}
