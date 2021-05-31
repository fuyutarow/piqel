use indexmap::IndexMap as Map;
use ordered_float::OrderedFloat;
use partiql::value::PqlValue;
use polars::datatypes::AnyValue;
use polars::prelude::CsvReader;
use polars::prelude::*;
use rayon::prelude::*;

use partiql::lang::Lang;
use partiql::lang::LangType;

fn main() -> anyhow::Result<()> {
    let input = include_str!("samples/boston.csv");
    let c = std::io::Cursor::new(input.to_owned());
    let df = CsvReader::new(c).infer_schema(Some(100)).finish()?;

    let v = PqlValue::from(df);
    dbg!(v);

    Ok(())
}
