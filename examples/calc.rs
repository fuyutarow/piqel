use std::collections::VecDeque;

use partiql::parser::clauses::from_pql_value;
use partiql::parser::select_statement::parse_sql3;
use partiql::sql::Proj;
use partiql::sql::Selector;
use partiql::sql::SelectorNode;
use partiql::value::PqlValue;

pub struct Evaluator {
    pub source: PqlValue,
    pub project: Vec<Proj>,
}

fn main() -> anyhow::Result<()> {
    dbg!("hello");

    let (_, value) = from_pql_value(
        r#"FROM { "arr": [1,2,3] }
    "#,
    )?;

    // -- SELECT arr FROM { "arr": [1,2,3] }
    let (_, plan) = parse_sql3(
        r#"
    SELECT * FROM { "arr": [1,2,3] }
    "#,
    )?;

    let evaluator = Evaluator {
        source: plan.from,
        project: plan.select,
    };

    let r = evaluator.source.select_by_selector(&Selector {
        data: vec![SelectorNode::Number(1)]
            .into_iter()
            .collect::<VecDeque<SelectorNode>>(),
    });

    dbg!(r);

    Ok(())
}
