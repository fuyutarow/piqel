use partiql::{
    dsql_parser,
    sql::{DField, DSql, Dpath},
};

#[test]
fn array123() {
    let input = "[1,2,3]";

    let (input, res) = dsql_parser::array(input).unwrap();
    assert_eq!(vec![1, 2, 3], res);
}

#[test]
fn field() -> anyhow::Result<()> {
    let (_, field) = dsql_parser::parse_field("a.b.c")?;

    assert_eq!(
        field,
        DField {
            path: Dpath::from("a.b.c"),
            alias: None
        }
    );

    Ok(())
}

#[test]
fn select_deeppath() -> anyhow::Result<()> {
    let sql = dsql_parser::sql(
        "
SELECT hr.employees.id,
       hr.employees.name AS employeeName,
       hr.employees.title AS title
FROM hr
",
    )?;
    dbg!(&sql);

    assert_eq!(
        sql,
        DSql {
            select_clause: vec![
                DField {
                    path: Dpath::from("hr.employees.id"),
                    alias: None
                },
                DField {
                    path: Dpath::from("hr.employees.name"),
                    alias: Some("employeeName".to_owned()),
                },
                DField {
                    path: Dpath::from("hr.employees.title"),
                    alias: Some("title".to_owned()),
                },
            ],
            from_clause: vec![DField {
                path: Dpath::from("hr"),
                alias: None,
            }],
            left_join_clause: vec![],
            where_clause: None,
        }
    );

    Ok(())
}

// #[test]
// fn select123() {
//     let input = "SELECT * FROM [1,2,3]";

//     let res = parser::sql(input);
//     assert_eq!(Ok(("", ())), res)
// }

// #[test] // fn q1() { //     let input = std::fs::read_to_string("samples/q1.sql").unwrap(); //     let input = input.trim_end();
//     let res = parser::sql(input);
//     assert_eq!(Ok(("", ())), res)
// }

// #[test]
// fn q2() {
//     let input = std::fs::read_to_string("samples/q2.sql").unwrap();
//     let input = input.trim_end();

//     let res = parser::sql(input);
//     assert_eq!(Ok(("", ())), res)
// }

// #[test]
// fn q3() {
//     let input = std::fs::read_to_string("samples/q3.sql").unwrap();
//     let input = input.trim_end();

//     let res = parser::sql(input);
//     assert_eq!(Ok(("", ())), res)
// }
