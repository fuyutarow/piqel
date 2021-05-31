
use std::str::FromStr;

use partiql::lang::Lang;
use partiql::lang::LangType;

fn main() -> anyhow::Result<()> {
    let input = r#"
  {
    "addr_info": {
      "family": "inet6",
      "local": "de99::112:5dfd:de17:e1cf",
      "preferred_life_time": 4294393545,
      "prefixlen": 64,
      "scope": "link",
      "valid_life_time": 42949393995
    }
  }
    "#;

    let mut lang = Lang::from_str(input)?;
    lang.to = LangType::Json;

    lang.print();

    Ok(())
}
