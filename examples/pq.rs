use std::io::{self, Read};
use std::path::PathBuf;
use std::str::FromStr;

use partiql::lang::{Lang, LangType};
use partiql::sql::evaluate;
use partiql::sql::parser;
use partiql::value::JsonValue;

fn main() -> anyhow::Result<()> {
    let input = r#"
{
  "name": "partiql-pokemon",
  "version": "0.202105.0",
  "array": [
    1,
    3,
    "ko"
  ],
  "private": true,
  "scripts": {
    "dev": "next",
    "build": "next build",
    "start": "next start",
    "prod": "next build && next start",
    "lint": "eslint . --fix -c .eslintrc.js --ext js,jsx,ts,tsx --ignore-pattern='!.*'",
    "type-check": "tsc"
  },
  "license": "MIT"
}
"#;
    let input = std::fs::read_to_string("samples/ip_addr.json").unwrap();

    let r = serde_json::from_str::<JsonValue>(&input);
    dbg!(&r);

    let sql = {
        //     let input = "
        // SELECT
        //   address,
        //   info.family AS inet,
        //   info.local
        // FROM addr_info AS info
        // WHERE inet LIKE 'inet%'
        //     ";
        let input = "
    SELECT
      address,
      info.family AS inet,
      info.local
    FROM addr_info AS info
        ";

        let sql = parser::sql(&input)?;
        sql
    };

    let mut lang = Lang::from_str(&input)?;
    lang.to = LangType::Toml;
    // lang.to = LangType::Yaml;

    // dbg!(&lang);
    let d = evaluate(&sql, &lang.data);
    dbg!(d);

    // lang.print();

    Ok(())
}
