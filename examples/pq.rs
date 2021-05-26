use std::io::{self, Read};
use std::path::PathBuf;
use std::str::FromStr;

use structopt::StructOpt;

use partiql::dsql_parser as sql_parser;
use partiql::lang::{Lang, LangType};
use partiql::sql::run;

use collect_mac::collect;

use std::collections::HashMap as Map;

fn main() -> anyhow::Result<()> {
    let input = r#"
{
    "employees": [
      {
        "id": 3,
        "name": "Bob Smith",
        "title": null
      },
      {
        "id": 4,
        "name": "Susan Smith",
        "title": "Dev Mgr"
      },
      {
        "id": 6,
        "name": "Jane Smith",
        "title": "Software Eng 2"
      }
    ]
}
"#;
    let input = r#"
    [
      {
        "id": 3,
        "name": "Bob Smith",
        "title": null
      },
      {
        "id": 4,
        "name": "Susan Smith",
        "title": "Dev Mgr"
      },
      {
        "id": 6,
        "name": "Jane Smith",
        "title": "Software Eng 2"
      }
    ]
"#;

    let mut lang = Lang::from_str(&input)?;
    lang.to = LangType::Toml;

    dbg!(&lang);

    lang.print();

    Ok(())
}
