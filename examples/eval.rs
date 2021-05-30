#![feature(box_patterns)]

use collect_mac::collect;
use indexmap::IndexMap as Map;
use partiql::pqlir_parser;
use partiql::sql::evaluate;
use partiql::sql::parser;
use partiql::sql::re_from_str;
use partiql::sql::restrict;
use partiql::sql::to_list;
use partiql::sql::Bindings;
use partiql::sql::DPath;
use partiql::sql::Expr;
use partiql::sql::Field;
use partiql::sql::WhereCond;
use partiql::value::PqlValue;

fn main() -> anyhow::Result<()> {
    let qi = "q2";
    let input: String = std::fs::read_to_string(format!("samples/{}.sql", qi))?;
    let data = {
        let input = std::fs::read_to_string(format!("samples/{}.env", qi)).unwrap();
        let model = pqlir_parser::pql_model(&input)?;
        model
    };

    let sql = parser::sql(&input)?;

    let d = evaluate(&sql, &data);

    dbg!(d);

    Ok(())
}
