use serde_derive::Deserialize;
use std::collections::VecDeque;
use std::str::FromStr;

use partiql::parser;
use partiql::parser::clauses::from_pql_value;
use partiql::parser::select_statement::parse_sql3;
use partiql::pqlir_parser;
use partiql::sql::Proj;
use partiql::sql::Selector;
use partiql::sql::SelectorNode;
use partiql::value::PqlValue;

pub struct Evaluator {
    pub source: PqlValue,
    pub project: Vec<Proj>,
}

// #[tokio::main]
// async fn main() -> anyhow::Result<()> {
//     let url_s = "https://api.github.com/repos/fuyutarow/partiql-rs/commits?per_page=1";

//     let repos = reqwest::get(url_s).await?.json::<PqlValue>().await?;
//     dbg!(&repos);
//     Ok(())
// }

fn main() -> anyhow::Result<()> {
    dbg!("hello");

    let data = {
        let input = std::fs::read_to_string("samples/q1.env").unwrap();
        pqlir_parser::from_str(&input)?
    };
    dbg!(&data);

    let data = data.select_by_selector(&Selector::from_str("hr.employees[2]")?);
    dbg!(data);

    let (_, q) = parser::select_statement::parse_sql(
        r#"
    SELECT e.name AS employeeName,
       p AS projectName
FROM hr.employeesNestScalars AS e,
     e.projects AS p
WHERE p LIKE '%security%'
    "#,
    )?;
    dbg!(&q);

    let source = parser::clauses::from2("FROM [1,2,3]")?.1;
    dbg!(&source);

    let source = parser::clauses::from2("FROM x.y.z AS xyz")?.1;
    dbg!(&source);

    let source = parser::clauses::from2("FROM [1,2,3] AS arr")?.1;
    dbg!(&source);

    // let r = parser::expressions::parse_selector("x.ys[2].z")?;
    // dbg!((r));

    // let (_, plan) = parse_sql3(
    //     r#"
    // SELECT * FROM [1,2,3] AS arr
    // "#,
    // )?;
    // dbg!(&plan);

    // let evaluator = Evaluator {
    //     source: plan.from,
    //     project: plan.select,
    // };
    let source = parser::clauses::from2(r#"FROM [1,"world",3]"#)?.1;
    dbg!(&source);

    // let source = parser::clauses::select2(r#"SELECT [1,"world",3] AS arr, arr[2]"#)?.1;
    // dbg!(&source);

    let source = parser::clauses::from2(
        r#"FROM
    { "mng": {
        "rust": "cargo",
        "python": "pip",
        "javascript": "npm"
    }}
"#,
    )?
    .1;
    dbg!(&source);
    let selector = parser::expressions::parse_selector("mng.rust")?.1;
    let d = source.select_by_selector(selector);
    dbg!(d);

    // let r = parser::clauses::select2(r#"SELECT arr[1] AS b, "hello"#)?.1;
    // dbg!(&r);

    // let value = PqlValue::from_str(r#"[1,2,4]"#)?;
    // let selected_value = value.select_by_selector(&Selector::from_str("[1]")?);
    // dbg!(selected_value);

    Ok(())
}
