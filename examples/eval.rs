#![feature(box_patterns)]

use partiql::parser;
use partiql::pqlir_parser;
use partiql::sql::evaluate;

fn main() -> anyhow::Result<()> {
    let input = "
SELECT
  address,
  info.family AS inet,
  info.local
FROM addr_info AS info
WHERE inet LIKE 'inet%'
        ";
    let sql = parser::sql(&input)?;
    dbg!(&sql);

    let data = {
        let input = std::fs::read_to_string("samples/ip_addr.json").unwrap();
        let model = pqlir_parser::pql_value(&input)?;
        model
    };
    dbg!(&data);

    let d = evaluate(&sql, &data);
    dbg!(d);

    Ok(())
}
