use partiql::sql_parser as parser;

fn main() {
    parse();
}

fn parse() -> anyhow::Result<()> {
    let input = "SELECT * FROM [1,2,3]";

    // let _ = parser::sql(input)?;

    let input = "[ 1, 2,3]";
    let r = parser::array(input)?;
    dbg!(r);

    //     let input = r#"
    // SELECT e.id,
    //     e.name AS employeeName,
    //     e.title AS title
    // FROM hr.employees e
    // WHERE e.title = 'Dev Mgr'
    //     "#;
    //         let input = r#"
    //  "#;
    //         let r = parser::sql(input)?;
    //         dbg!(r);

    let input = r#"
    SELECT e.id,
        e.name AS employeeName,
        e.title AS title
    FROM hr.employees AS e
    WHERE e.title = 'Dev Mgr'
 "#;
    // let r = parser::sql(input)?;
    let r = parser::sql(input)?;
    dbg!(r);

    Ok(())
}
