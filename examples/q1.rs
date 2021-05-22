use partiql::models::JsonValue;
use partiql::pqlon_parser as parser;
use partiql::sql_parser;
fn main() {
    parse();
}

fn parse() -> anyhow::Result<()> {
    let sql = {
        let input = std::fs::read_to_string("samples/q1.sql").unwrap();
        let sql = sql_parser::sql(&input)?;
        sql
    };
    // dbg!(&sql);

    let data = {
        let input = std::fs::read_to_string("samples/q1.env").unwrap();
        let model = parser::pql_model(&input)?;
        model
    };
    // dbg!(&data);

    // let s = data.get("hr");
    // dbg!(s);

    // let data = data.get("hr").unwrap();
    // dbg!(&data);
    // let data = data.get("employees");
    // dbg!(&data);

    let from_path = sql
        .from_clause
        .first()
        .unwrap()
        .path
        .split(".")
        .collect::<Vec<_>>();

    dbg!(&from_path);
    let rows = data.get_path(&from_path).unwrap();
    dbg!(&rows);

    let vec_path = sql
        .select_clause
        .into_iter()
        .map(|field| {
            // let e_path = field.path.split(".").collect::<Vec<_>>();
            let e_path = field
                .path
                .split(".")
                .map(String::from)
                .collect::<Vec<String>>();
            e_path
            // let (_, child_path) = e_path.split_first().unwrap();
            // dbg!(&child_path);
            // child_path
        })
        .map(|path| {
            let (_, child_path) = path.split_first().unwrap();
            dbg!(&child_path);
            // child_path.to_owned()
            child_path.first().unwrap().to_string()
        })
        .collect::<Vec<_>>();

    let path_list = &vec_path.iter().map(|s| s.as_str()).collect::<Vec<_>>();
    let data = rows.filter_map(&path_list);
    dbg!(&data);

    // dbg!(&path1);
    // match data {
    //     JsonValue::Object(map) => {
    //         let s = map.get("hr");
    //         dbg!(s);
    //     }
    //     _ => {}
    // }

    let output = {
        let input = std::fs::read_to_string("samples/q1.output").unwrap();
        let v = input.split("---").collect::<Vec<_>>();
        let input = v.first().unwrap();
        let model = parser::pql_model(&input)?;
        model
    };
    // dbg!(&output);

    Ok(())
}
