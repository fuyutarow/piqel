use collect_mac::collect;
use indexmap::IndexMap as Map;
use partiql::pqlir_parser;
use partiql::sql::parser;
use partiql::sql::re_from_str;
use partiql::sql::restrict;
use partiql::sql::to_list;
use partiql::sql::Bindings;
use partiql::sql::DPath;
use partiql::sql::Field;
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
    dbg!(&bindings);

    let path = DPath::from("hr.employeesNest.projects.name");
    let data = restrict(Some(data), &path, Some("%security%")).unwrap();

    let fields = vec![
        Field {
            path: DPath::from("hr.employeesNest.name"),
            alias: Some("employeeName".to_owned()),
        },
        Field {
            path: DPath::from("hr.employeesNest.projects.name"),
            alias: Some("projectName".to_owned()),
        },
    ];
    let d = data.select_by_fields(&fields).unwrap();
    let d = to_list(d);

    dbg!(d);

    Ok(())
}
