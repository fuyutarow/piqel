use partiql::sql::parser;

fn main() -> anyhow::Result<()> {
  let input = r#"
  SELECT e.name AS employeeName,
    ( SELECT COUNT(*)
      FROM e.projects AS p
      WHERE p.name LIKE'%querying%'
    ) AS queryProjectsNum
  FROM hr.employeesNest AS e
      "#;
  let sql = parser::parse_sql(&input)?;
  dbg!(&sql);

  Ok(())
}
