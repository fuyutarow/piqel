use collect_mac::collect;
use indexmap::IndexMap as Map;
use partiql::pqlir_parser;
use partiql::sql::DPath;
use partiql::value::PqlValue;

fn eval_bool(value: &Option<PqlValue>) -> bool {
    match value {
        Some(PqlValue::Boolean(boolean)) => boolean.to_owned(),
        Some(PqlValue::Null) => false,
        None => false,
        _ => todo!(),
    }
}

fn main() -> anyhow::Result<()> {
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

    match data.select_by_path(&DPath::from("top")) {
        Some(PqlValue::Array(array)) => {
            for d in array {
                let t = d.select_by_path(&DPath::from("b"));
                let b = eval_bool(&t);
                if b {
                    dbg!(d);
                }
            }
        }
        _ => todo!(),
    }

    Ok(())
}
