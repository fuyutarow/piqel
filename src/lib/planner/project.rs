use collect_mac::collect;
use indexmap::IndexMap as Map;
use itertools::Itertools;

use crate::sql::Env;
use crate::sql::Expr;
use crate::sql::Field;
use crate::sql::Selector;
use crate::value::PqlValue;

#[derive(Debug, Default, Clone)]
pub struct Projection(pub Vec<Field>);

impl Projection {
    pub fn execute(self, data: PqlValue, env: &Env) -> Vec<PqlValue> {
        let v = self.step1(data, env);
        dbg!(&v);
        let v = self.step2(v);
        let v = self.step3(v);
        let v = self.step4(v);
        v
    }

    pub fn step1(&self, data: PqlValue, env: &Env) -> PqlValue {
        dbg!(&self);
        let fields = self
            .0
            .iter()
            .map(|field| field.expand_fullpath(&env))
            .collect::<Vec<Field>>();
        dbg!(&fields);
        let projected = data.select_by_fields(&fields, &env).unwrap_or_default();
        projected
    }

    pub fn step2(&self, data: PqlValue) -> Rows {
        Rows::from(data)
    }

    pub fn step3(&self, rows: Rows) -> Records {
        Records::from(rows)
    }

    pub fn step4(&self, records: Records) -> Vec<PqlValue> {
        records.into_list()
    }
}

impl PqlValue {
    pub fn project_by_selector(
        &self,
        alias: Option<String>,
        selector: &Selector,
    ) -> (String, Self) {
        if let Some(value) = self.select_by_selector(&selector) {
            let key = alias.clone().unwrap_or({
                let last = selector.to_vec().last().unwrap().to_string();
                last
            });
            (key, value)
        } else {
            dbg!(&selector);
            todo!()
        }
    }

    pub fn select_by_fields(&self, field_list: &[Field], _env: &Env) -> Option<Self> {
        let mut new_map = Map::<String, Self>::new();

        for field in field_list {
            match &field.expr {
                Expr::Selector(selector) => {
                    if let Some(value) = self.select_by_selector(&selector) {
                        let key = field.alias.clone().unwrap_or({
                            let last = selector.to_vec().last().unwrap().to_string();
                            last
                        });
                        new_map.insert(key, value);
                    } else {
                        dbg!(&selector);
                        todo!()
                    }
                }
                Expr::Value(_) => todo!(),
                _ => todo!(),
            }
        }

        Some(Self::Object(new_map))
    }
}

#[derive(Debug, Default, Clone)]
pub struct Rows {
    data: Map<String, Vec<PqlValue>>,
    size: usize,
    keys: Vec<String>,
}

impl From<PqlValue> for Rows {
    fn from(value: PqlValue) -> Self {
        let mut size = 0;

        let data = match value {
            PqlValue::Object(record) => record
                .into_iter()
                .map(|(key, val)| match val {
                    PqlValue::Array(array) => {
                        if size == 0 {
                            size = array.len();
                        }
                        (key, array)
                    }
                    _ => {
                        size = 1;
                        (key, vec![val])
                    }
                })
                .collect::<Map<String, Vec<PqlValue>>>(),
            _ => unreachable!(),
        };

        let keys = data.keys().map(String::from).collect();
        Self { data, size, keys }
    }
}

impl From<Rows> for PqlValue {
    fn from(records: Rows) -> Self {
        let array = records
            .data
            .into_iter()
            .map(|(k, v)| {
                PqlValue::Object(collect! {
                    as Map<String, PqlValue>:
                    k => PqlValue::Array(v)
                })
            })
            .collect::<Vec<_>>();
        PqlValue::Array(array)
    }
}

#[derive(Debug, Default, Clone)]
pub struct Records(pub Vec<Map<String, Vec<PqlValue>>>);

impl From<Rows> for Records {
    fn from(rows: Rows) -> Self {
        let records = {
            let mut records = Vec::<Map<String, Vec<PqlValue>>>::new();
            for i in 0..rows.size {
                let mut record = Map::<String, Vec<PqlValue>>::new();
                for key in &rows.keys {
                    let v = rows.data.get(key.as_str()).unwrap().get(i).unwrap();
                    // record.insert(key.to_string(), v.to_owned());
                    match v {
                        PqlValue::Array(array) => {
                            record.insert(key.to_string(), array.to_owned());
                        }
                        _ => {
                            record.insert(key.to_string(), vec![v.to_owned()]);
                        }
                    }
                }
                records.push(record);
            }
            records
        };
        Self(records)
    }
}

impl From<Records> for PqlValue {
    fn from(records: Records) -> Self {
        Self::Array(
            records
                .0
                .into_iter()
                .map(|obj| {
                    Self::Object(
                        obj.into_iter()
                            .map(|(k, v)| (k, Self::Array(v)))
                            .collect::<Map<String, _>>(),
                    )
                })
                .collect::<Vec<_>>(),
        )
    }
}

impl Records {
    pub fn into_list(self) -> Vec<PqlValue> {
        self.0
            .into_iter()
            .map(|record| {
                let record = record
                    .into_iter()
                    .filter_map(|(k, v)| if !v.is_empty() { Some((k, v)) } else { None })
                    .collect::<Map<String, Vec<PqlValue>>>();

                let keys = record.keys();
                let it = record.values().into_iter().multi_cartesian_product();
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
            })
            .flatten()
            .collect::<Vec<PqlValue>>()
    }
}

#[cfg(test)]
mod tests {
    use super::Records;
    use super::Rows;
    use crate::value::PqlValue;
    use std::str::FromStr;

    #[test]
    fn test_convert_coloumnar_to_rowwise() -> anyhow::Result<()> {
        let form0 = PqlValue::from_str(
            r#"
{
  "projectName": [
    [
      "AWS Redshift security",
      "AWS Aurora security"
    ],
    [
      "AWS Redshift security"
    ]
  ],
  "employeeName": [
    "Bob Smith",
    "Jane Smith"
  ]
}
"#,
        )?;
        let form1 = PqlValue::from_str(
            r#"
[
  {
    "projectName": [
      [
        "AWS Redshift security",
        "AWS Aurora security"
      ],
      [
        "AWS Redshift security"
      ]
    ]
  },
  {
    "employeeName": [
      "Bob Smith",
      "Jane Smith"
    ]
  }
]
"#,
        )?;
        let form2 = PqlValue::from_str(
            r#"
[
  {
    "projectName": [
      "AWS Redshift security",
      "AWS Aurora security"
    ],
    "employeeName": [
      "Bob Smith"
    ]
  },
  {
    "projectName": [
      "AWS Redshift security"
    ],
    "employeeName": [
      "Jane Smith"
    ]
  }
]
"#,
        )?;
        let form3 = PqlValue::from_str(
            r#"
[
  {
    "projectName": "AWS Redshift security",
    "employeeName": "Bob Smith"
  },
  {
    "projectName": "AWS Aurora security",
    "employeeName": "Bob Smith"
  },
  {
    "projectName": "AWS Redshift security",
    "employeeName": "Jane Smith"
  }
]
"#,
        )?;

        let rows = Rows::from(form0.to_owned());
        assert_eq!(PqlValue::from(rows.to_owned()), form1);

        let records = Records::from(rows);
        assert_eq!(PqlValue::from(records.to_owned()), form2);

        let list = records.into_list();
        assert_eq!(PqlValue::from(list.to_owned()), form3);

        Ok(())
    }
}
