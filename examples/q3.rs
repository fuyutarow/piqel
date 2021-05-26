use indexmap::IndexMap;

use itertools::Itertools;

use partiql::dsql_parser;
use partiql::pqlir_parser;
use partiql::sql::to_list;
use partiql::sql::Bindings;
use partiql::sql::DSql as Sql;
use partiql::value::JsonValue;

fn main() {
    parse();
}

fn run(sql: &Sql, data: &JsonValue) -> JsonValue {
    let fields = sql
        .select_clause
        .iter()
        .chain(sql.from_clause.iter())
        .chain(sql.left_join_clause.iter())
        .map(|e| e.to_owned())
        .collect::<Vec<_>>();
    let bindings = Bindings::from(fields.as_slice());

    let select_fields = sql
        .select_clause
        .iter()
        .map(|field| field.to_owned().full(&bindings))
        .collect::<Vec<_>>();
    let bindings_for_select = Bindings::from(select_fields.as_slice());

    let value = data.select_by_fields(&select_fields).unwrap();
    let list = to_list(value);
    dbg!(&list);

    let filtered_list = list
        .iter()
        .filter_map(|value| match &sql.where_clause {
            Some(cond) if cond.eval(&value.to_owned(), &bindings, &bindings_for_select) => {
                Some(value.to_owned())
            }
            Some(_) => None,
            _ => Some(value.to_owned()),
        })
        .collect::<Vec<JsonValue>>();
    dbg!(&filtered_list);

    JsonValue::Array(filtered_list)
}

fn parse() -> anyhow::Result<()> {
    let sql = {
        let input = std::fs::read_to_string("samples/q3.sql").unwrap();
        let sql = dsql_parser::sql(&input)?;
        sql
    };

    let data = {
        let input = std::fs::read_to_string("samples/q3.env").unwrap();
        let model = pqlir_parser::pql_model(&input)?;
        model
    };

    let output = {
        let input = std::fs::read_to_string("samples/q3.output").unwrap();
        let v = input.split("---").collect::<Vec<_>>();
        let input = v.first().unwrap();
        let model = pqlir_parser::pql_model(&input)?;
        model
    };

    let res = run(&sql, &data);

    dbg!(&res);
    dbg!(&output);

    assert_eq!(res, output);

    dbg!("END OF FILE");
    Ok(())
}
