use std::io::{self, Read};
use std::path::PathBuf;
use std::str::FromStr;

use partiql::lang::{Lang, LangType};
use partiql::sql::evaluate;
use partiql::sql::parser;
use partiql::value::JsonValue;

fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("samples/ip_addr.json").unwrap();

    let r = serde_json::from_str::<JsonValue>(&input);
    dbg!(&r);

    let sql = {
        let input = "
SELECT addr_info
WHERE addr_info.family = 'inet6'
        ";
        parser::sql(&input)?
    };
    dbg!(&sql);

    let mut lang = Lang::from_str(&input)?;
    lang.to = LangType::Toml;
    // lang.to = LangType::Yaml;

    // dbg!(&lang);
    let d = evaluate(&sql, &lang.data);
    dbg!(d);

    // lang.print();

    Ok(())
}
