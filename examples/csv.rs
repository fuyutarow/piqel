use std::any::Any;
use std::convert::TryFrom;

use indexmap::IndexMap as Map;
use ordered_float::OrderedFloat;
use partiql::value::PqlValue;
use polars::datatypes::AnyValue;
use polars::df;
use polars::prelude::CsvReader;
use polars::prelude::*;
use rayon::prelude::*;

use partiql::lang::Lang;
use partiql::lang::LangType;

fn main() -> anyhow::Result<()> {
    let input = include_str!("samples/boston.csv");
    let c = std::io::Cursor::new(input.to_owned());
    let df = CsvReader::new(c).infer_schema(Some(100)).finish()?;
    dbg!(df.width(), df.height());
    dbg!(&df);

    let v = PqlValue::from(df);

    let df2 = DataFrame::try_from(v)?;
    dbg!(df2);

    Ok(())
}
