use collect_mac::collect;
use indexmap::IndexMap as Map;
use partiql::pqlir_parser;
use partiql::sql::parser;
use partiql::sql::re_from_str;
use partiql::sql::restrict;
use partiql::sql::Bindings;
use partiql::sql::DPath;
use partiql::value::PqlValue;

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
    let data = restrict(Some(data), &path, Some("%security%")).unwrap();

    let d = data.select_by_path(&DPath::from("hr.employeesNest.name"));
    dbg!(&d);

    // let bindings_for_select = Bindings::from(select_fields.as_slice());

    Ok(())
}
