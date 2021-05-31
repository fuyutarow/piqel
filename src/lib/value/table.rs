use std::convert::TryFrom;

use indexmap::IndexMap as Map;
use ordered_float::OrderedFloat;
use polars::datatypes::AnyValue;
use polars::prelude::DataFrame;
use polars::prelude::*;
use rayon::prelude::*;

use crate::value::PqlValue;

impl From<DataFrame> for PqlValue {
    fn from(df: DataFrame) -> Self {
        let n_cols = df.height();
        let col_names = df
            .fields()
            .into_iter()
            .map(|field| field.name().to_owned())
            .collect::<Vec<_>>();
        let table = (0..n_cols)
            .into_par_iter()
            .map(|idx| {
                let row = df.take_iter(idx..idx + 1);

                let values = row
                    .get_columns()
                    .into_par_iter()
                    .map(|v| match v.to_owned().get(0) {
                        AnyValue::Int64(int) => PqlValue::Int(int),
                        AnyValue::Float64(float) => PqlValue::Float(OrderedFloat(float)),
                        _ => todo!(),
                    })
                    .collect::<Vec<PqlValue>>();

                let object = col_names
                    .to_owned()
                    .into_iter()
                    .zip(values.into_iter())
                    .collect::<Map<String, _>>();

                PqlValue::Object(object)
            })
            .collect::<Vec<_>>();

        PqlValue::Array(table)
    }
}

impl TryFrom<PqlValue> for DataFrame {
    type Error = anyhow::Error;

    fn try_from(pqlv: PqlValue) -> anyhow::Result<Self> {
        let rows = match pqlv {
            PqlValue::Array(array) => {
                let rows = array
                    .into_par_iter()
                    .map(|value| {
                        let series_list = match value {
                            PqlValue::Object(object) => {
                                let series_list = object
                                    .into_iter()
                                    .map(|(k, v)| match v {
                                        PqlValue::Int(int) => Series::new(&k, &[int]),
                                        PqlValue::Float(float) => {
                                            let f: f64 = float.into_inner();
                                            Series::new(&k, &[f])
                                        }
                                        _ => todo!(),
                                    })
                                    .collect::<Vec<Series>>();
                                series_list
                            }
                            _ => panic!("Only object can be converted to tables"),
                        };
                        let row = DataFrame::new(series_list).unwrap();
                        row
                    })
                    .collect::<Vec<DataFrame>>();
                rows
            }
            _ => anyhow::bail!("Only arrays can be converted to tables"),
        };
        dbg!(rows.len());

        let mut it = rows.into_iter();
        let mut df = it.next().unwrap();
        for row in it {
            df = df.vstack(&row)?;
        }

        Ok(df)
    }
}
