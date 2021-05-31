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
