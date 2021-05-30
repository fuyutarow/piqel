use collect_mac::collect;
use indexmap::IndexMap as Map;
use nom::combinator::rest;
use partiql::pqlir_parser;
use partiql::sql::parser;
use partiql::sql::re_from_str;
use partiql::sql::Bindings;
use partiql::sql::DPath;
use partiql::value::PqlValue;

fn eval_bool(value: Option<PqlValue>, path: &DPath, right: Option<&str>) -> Option<PqlValue> {
    match value {
        None => None,
        Some(PqlValue::Boolean(boolean)) if boolean => Some(PqlValue::Boolean(boolean)),
        Some(PqlValue::Boolean(_)) => None,
        Some(PqlValue::Null) => None,
        Some(PqlValue::Str(string)) => {
            if let Some(pattern) = right {
                if re_from_str(pattern).is_match(&string) {
                    dbg!("#1", right, &string);
                    Some(PqlValue::Str(string))
                } else {
                    dbg!("#2", right, &string);
                    None
                }
            } else {
                Some(PqlValue::Str(string))
            }
        }
        Some(PqlValue::Float(float)) => Some(PqlValue::Float(float)),
        Some(PqlValue::Array(array)) => {
            let arr = array
                .into_iter()
                .filter_map(|v| {
                    if let Some((first, tail)) = &path.split_first() {
                        eval_bool(Some(v), tail, right)
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>();

            if arr.len() > 0 {
                dbg!("#1", &arr);
                Some(PqlValue::Array(arr))
            } else {
                dbg!("#2", &arr);
                None
            }
        }
        Some(PqlValue::Object(object)) => {
            let obj = object
                .into_iter()
                .filter_map(|(k, v)| {
                    if let Some((first, tail)) = &path.split_first() {
                        if first.to_string() == k {
                            // dbg!(&k, &v);
                            let vv = eval_bool(Some(v), tail, right);
                            // dbg!(&vv);
                            match vv {
                                Some(val) => Some((k, val)),
                                None => None,
                            }
                        } else {
                            Some((k, v))
                        }
                    } else {
                        Some((k, v))
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
    let qi = "q2";
    let input: String = std::fs::read_to_string(format!("samples/{}.sql", qi))?;
    let data = {
        let input = std::fs::read_to_string(format!("samples/{}.env", qi)).unwrap();
        let model = pqlir_parser::pql_model(&input)?;
        model
    };

    let sql = parser::sql(&input)?;
    dbg!(&sql);
    let fields = sql
        .from_clause
        .iter()
        .chain(sql.left_join_clause.iter())
        .map(|e| e.to_owned())
        .collect::<Vec<_>>();

    let bindings = Bindings::from(fields.as_slice());

    let path = DPath::from("hr.employeesNest.projects.name");
    let right = "%security%".to_owned();
    let restricted_data = eval_bool(Some(data), &path, Some(&right));

    let data = pqlir_parser::pql_model(
        "
{
    'top': <<
        {'a': 1, 'b': true, 'c': 'alpha'},
        {'a': 2, 'b': null, 'c': 'beta'},
        {'a': 3, 'c': 'gamma'}
    >>
}
   ",
    )?;
    let r = eval_bool(Some(data), &DPath::from("top.b"), None);
    dbg!(r);

    // let bindings_for_select = Bindings::from(select_fields.as_slice());

    Ok(())
}
