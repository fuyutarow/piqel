use partiql::dsql_parser as parser;

fn main() {
    parse();
}

fn parse() -> anyhow::Result<()> {
    let input = "SELECT * FROM [1,2,3]";

    // let _ = parser::sql(input)?;

    let input = "[ 1, 2,3]";
    let r = parser::array(input)?;
    dbg!(r);

    // dbg!(input);

    let input = "LEFT JOIN e.projects AS p";
    let r = parser::parse_left_join(input);
    dbg!(r);

    let input = std::fs::read_to_string("samples/q1.sql").unwrap();
    println!("{}", input);
    let r = parser::sql(&input);
    match r {
        Ok(sql) => {
            dbg!(sql);
        }
        Err(err) => eprintln!("{}", err),
    }

    //     let input = r#"
    //     SELECT e.id AS id,
    //        e.name AS employeeName,
    //        e.title AS title,
    //        p.name AS projectName
    // FROM hr.employeesNest AS e
    //     LEFT JOIN e.projects AS p
    //     "#;
    //     let r = parser::sql(&input);
    //     dbg!(r);

    dbg!("=======");

    Ok(())
}
