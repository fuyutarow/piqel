use std::any::Any;
use std::collections::BTreeMap;
use std::str::FromStr;

use indexmap::IndexMap;
use itertools::Itertools;

use partiql::lang::{Lang, LangType};
use partiql::models::BJsonValue;
use partiql::models::JsonValue;

fn main() -> anyhow::Result<()> {
    let input = include_str!("samples/q2.json");

    let mut lang = Lang::from_str(&input)?;

    dbg!(&lang.data);

    let json = serde_json::to_string(&lang.data).unwrap();
    // let r = serde_json::from_str::<JsonValue>(&json);
    let r = serde_json::from_str::<BJsonValue>(&json);

    dbg!(&r);
    // lang.data

    Ok(())
}
