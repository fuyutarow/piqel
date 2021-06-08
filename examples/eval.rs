#![feature(box_patterns)]

use collect_mac::collect;
use indexmap::IndexMap as Map;
use partiql::parser;
use partiql::pqlir_parser;
use partiql::sql::evaluate;
use partiql::sql::re_from_str;
use partiql::sql::restrict;
use partiql::sql::Bindings;
use partiql::sql::DPath;
use partiql::sql::Expr;
use partiql::sql::Field;
use partiql::sql::WhereCond;
use partiql::value::PqlValue;

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
        let model = pqlir_parser::pql_model(&input)?;
        model
    };
    dbg!(&data);

    let d = evaluate(&sql, &data);
    dbg!(d);

    Ok(())
}
