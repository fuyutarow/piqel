use indexmap::IndexMap;

use itertools::Itertools;

use partiql::dsql_parser;
use partiql::models::JsonValue;
use partiql::pqlir_parser;
use partiql::sql::run;
use partiql::sql::to_list;
use partiql::sql::Bindings;
use partiql::sql::DField;
use partiql::sql::DSql as Sql;
use partiql::sql::DWhereCond;
use partiql::sql::Dpath;

#[test]
fn q1() -> anyhow::Result<()> {
    let qi = "q1";
    let sql = {
        let input = std::fs::read_to_string(format!("samples/{}.sql", qi)).unwrap();
        let sql = dsql_parser::sql(&input)?;
        sql
    };

    let data = {
        let input = std::fs::read_to_string(format!("samples/{}.env", qi)).unwrap();
        let model = pqlir_parser::pql_model(&input)?;
        model
    };

    let output = {
        let input = std::fs::read_to_string(format!("samples/{}.output", qi)).unwrap();
        let v = input.split("---").collect::<Vec<_>>();
        let input = v.first().unwrap();
        let model = pqlir_parser::pql_model(&input)?;
        model
    };

    let res = run(&sql, &data);
    assert_eq!(res, output);
    Ok(())
}

#[test]
fn q2() -> anyhow::Result<()> {
    let qi = "q2";
    let sql = {
        let input = std::fs::read_to_string(format!("samples/{}.sql", qi)).unwrap();
        let sql = dsql_parser::sql(&input)?;
        sql
    };

    let data = {
        let input = std::fs::read_to_string(format!("samples/{}.env", qi)).unwrap();
        let model = pqlir_parser::pql_model(&input)?;
        model
    };

    let output = {
        let input = std::fs::read_to_string(format!("samples/{}.output", qi)).unwrap();
        let v = input.split("---").collect::<Vec<_>>();
        let input = v.first().unwrap();
        let model = pqlir_parser::pql_model(&input)?;
        model
    };

    let res = run(&sql, &data);
    assert_eq!(res, output);
    Ok(())
}

#[test]
fn q3() -> anyhow::Result<()> {
    let qi = "q3";
    let sql = {
        let input = std::fs::read_to_string(format!("samples/{}.sql", qi)).unwrap();
        let sql = dsql_parser::sql(&input)?;
        sql
    };

    let data = {
        let input = std::fs::read_to_string(format!("samples/{}.env", qi)).unwrap();
        let model = pqlir_parser::pql_model(&input)?;
        model
    };

    let output = {
        let input = std::fs::read_to_string(format!("samples/{}.output", qi)).unwrap();
        let v = input.split("---").collect::<Vec<_>>();
        let input = v.first().unwrap();
        let model = pqlir_parser::pql_model(&input)?;
        model
    };

    let res = run(&sql, &data);
    assert_eq!(res, output);
    Ok(())
}
