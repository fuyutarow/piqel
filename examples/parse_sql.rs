use partiql::sql::parser;

fn main() -> anyhow::Result<()> {
  let input = r#"
SELECT t.id AS id,
       x AS even
FROM matrices AS t,
     t.matrix AS y,
     y AS x
WHERE x % 2 = 0
      "#;
  let sql = parser::parse_sql(&input)?;
  dbg!(&sql);

  let input = "a- COUNT(*) * 12.";
  let r = parser::parse_expr(&input)?;
  dbg!(r);

  let input = "WHERE x/2 = 0";
  let r = parser::parse_where(&input)?;
  dbg!(r);

  Ok(())
}
