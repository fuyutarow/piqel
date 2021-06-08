use partiql::parser;

fn main() -> anyhow::Result<()> {
    let input = r#"
SELECT e.id,
       e.name AS employeeName,
       UPPER(e.title) AS outputTitle FROM hr.employeesWithMissing AS e

      "#;
    let sql = parser::parse_sql(&input)?;
    dbg!(&sql);

    Ok(())
}
