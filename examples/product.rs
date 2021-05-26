use indexmap::IndexMap;

use collect_mac::collect;
use itertools::Itertools;

use partiql::value::JsonValue;

fn main() {
    let mut records = Vec::<IndexMap<String, Vec<JsonValue>>>::new();

    let record = collect! {
        as IndexMap<String, Vec<JsonValue>>:
        "id".to_owned() => vec![
            JsonValue::Num(
                3.0,
            ),
        ],
        "employeeName".to_owned()=> vec![
            JsonValue::Str(
                "Bob Smith".to_owned(),
            ),
        ],
        "projectName".to_owned()=> vec![
            JsonValue::Str(
                "AWS Redshift Spectrum querying".to_owned(),
            ),
            JsonValue::Str(
                "AWS Redshift security".to_owned(),
            ),
            JsonValue::Str(
                "AWS Aurora security".to_owned(),
            ),
        ],
        "title".to_owned()=> vec![
            JsonValue::Null,
        ],
    };
    dbg!(&record);

    let record = record
        .into_iter()
        .filter_map(|(k, v)| if v.len() > 0 { Some((k, v)) } else { None })
        .collect::<IndexMap<String, Vec<JsonValue>>>();

    let it = record.values().into_iter().multi_cartesian_product();

    for prod in it {
        dbg!(prod);
    }
}
