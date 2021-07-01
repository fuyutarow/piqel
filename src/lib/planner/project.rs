use indexmap::IndexMap as Map;
use itertools::Itertools;

use crate::sql::Field;

use crate::value::PqlValue;

#[derive(Debug, Default, Clone)]
pub struct Projection(pub Vec<Field>);

impl Projection {
    pub fn execute(self, data: PqlValue) -> Vec<PqlValue> {
        let projected = data.select_by_fields(&self.0).unwrap_or_default();
        Records::from(projected).into_list()
    }
}

#[derive(Debug, Default, Clone)]
pub struct Records(pub Map<String, Vec<PqlValue>>);

impl From<PqlValue> for Records {
    fn from(value: PqlValue) -> Self {
        Self(match value {
            PqlValue::Object(record) => record
                .into_iter()
                .map(|(k, v)| match v {
                    PqlValue::Array(array) => (k, array),
                    _ => unreachable!(),
                })
                .collect::<Map<String, Vec<PqlValue>>>(),
            _ => unreachable!(),
        })
    }
}

impl Records {
    fn into_list(self) -> Vec<PqlValue> {
        let keys = self.0.keys();
        let it = self.0.values().into_iter().multi_cartesian_product();
        it.map(|prod| {
            let map = keys
                .clone()
                .into_iter()
                .zip(prod.into_iter())
                .map(|(key, p)| (key.to_owned(), p.to_owned()))
                .collect::<Map<String, _>>();
            let v = PqlValue::Object(map);
            v
        })
        .collect::<Vec<PqlValue>>()
    }
}
