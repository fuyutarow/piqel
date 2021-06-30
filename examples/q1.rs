use partiql::parser;
use partiql::pqlir_parser;
use partiql::sql::Env;
use partiql::sql::Field;
use partiql::sql::Proj;
use partiql::value::PqlValue;

pub struct Evaluator {
    pub source: PqlValue,
    pub project: Vec<Proj>,
}

fn main() -> anyhow::Result<()> {
    let data = {
        let input = std::fs::read_to_string("samples/q1.env").unwrap();
        pqlir_parser::from_str(&input)?
    };
    let mut env = Env::default();
    env.insert_from_pqlval("", &data);

    let fields = parser::clauses::from("FROM hr.employees AS e, [1,2,3] AS arr")?.1;

    for field in fields {
        if let Some(alias) = field.alias {
            env.insert(&alias, &field.value);
        }
    }

    let fields = parser::clauses::select2(
        r#"SELECT e.id,
       e.name AS employeeName,
       e.title AS title
    "#,
    )?
    .1
    .into_iter()
    .map(|field| field.expand_fullpath(&env))
    .collect::<Vec<Field>>();
    let d = data.select_by_fields(&fields);
    dbg!(&d);

    Ok(())
}
