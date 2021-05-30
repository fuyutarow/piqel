use collect_mac::collect;
use indexmap::IndexMap as Map;
use partiql::pqlir_parser;
use partiql::sql::DPath;
use partiql::value::PqlValue;

fn eval_bool(value: Option<PqlValue>) -> Option<PqlValue> {
    match value {
        None => None,
        Some(PqlValue::Boolean(boolean)) if boolean => Some(PqlValue::Boolean(boolean)),
        Some(PqlValue::Boolean(boolean)) => None,
        Some(PqlValue::Null) => None,
        Some(PqlValue::Float(float)) => Some(PqlValue::Float(float)),
        Some(PqlValue::Array(array)) => {
            let arr = array
                .into_iter()
                .filter_map(|e| eval_bool(Some(e)))
                .collect::<Vec<_>>();

            Some(PqlValue::Array(arr))
        }
        Some(PqlValue::Object(object)) => {
            let obj = object
                .into_iter()
                .filter_map(|(k, v)| {
                    let vv = eval_bool(Some(v));
                    match vv {
                        Some(val) => Some((k, val)),
                        None => None,
                    }
                })
                .collect::<Map<String, _>>();
            Some(PqlValue::Object(obj))
        }
        _ => {
            dbg!(value);
            todo!();
        }
    }
}

fn main() -> anyhow::Result<()> {
    let data = pqlir_parser::pql_model(
        "
{
    'top': <<
        {'a': 1, 'b': {'c': true}},
        {'a': 2, 'b': {'c': false}},
        {'a': 2, 'b': {}},
        {'a': 4}
    >>
}
   ",
    )?;
    dbg!(&data);

    let d = data.select_by_path(&DPath::from("top.b.c"));
    let d = eval_bool(d);
    dbg!(&d);

    // let d = data.select_by_path(&DPath::from("top.a.b"));
    // dbg!(d);
    // let d = eval_bool(d);
    // dbg!(&d);

    Ok(())
}
