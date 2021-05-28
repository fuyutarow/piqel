use partiql::sql::parser;
use partiql::sql::DWhereCond;
use partiql::sql::Expr;
use partiql::sql::Field;
use partiql::sql::Proj;
use partiql::sql::{DPath, Sql};

fn get_sql(qi: &str) -> anyhow::Result<Sql> {
    let input = std::fs::read_to_string(format!("samples/{}.sql", qi)).unwrap();
    dbg!(qi);
    println!("{}", input);
    let sql = parser::sql(&input)?;
    Ok(sql)
}

#[test]
fn array123() {
    let input = "[1,2,3]";

    let (input, res) = parser::array(input).unwrap();
    assert_eq!(vec![1, 2, 3], res);
}

#[test]
fn field() -> anyhow::Result<()> {
    let (_, field) = parser::parse_field("a.b.c")?;

    assert_eq!(
        field,
        Field {
            path: DPath::from("a.b.c"),
            alias: None
        }
    );

    Ok(())
}

#[test]
fn select_deeppath() -> anyhow::Result<()> {
    let sql = parser::sql(
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
        Sql {
            select_clause: vec![
                Proj {
                    expr: Expr::Path(DPath::from("hr.employees.id")),
                    alias: None
                },
                Proj {
                    expr: Expr::Path(DPath::from("hr.employees.name")),
                    alias: Some("employeeName".to_owned()),
                },
                Proj {
                    expr: Expr::Path(DPath::from("hr.employees.title")),
                    alias: Some("title".to_owned()),
                },
            ],
            from_clause: vec![Field {
                path: DPath::from("hr"),
                alias: None,
            }],
            left_join_clause: vec![],
            where_clause: None,
        }
    );

    Ok(())
}

#[test]
fn q1() -> anyhow::Result<()> {
    let sql = get_sql("q1")?;
    dbg!(&sql);

    assert_eq!(
        sql,
        Sql {
            select_clause: vec![
                Proj {
                    expr: Expr::Path(DPath::from("e.id")),
                    alias: None
                },
                Proj {
                    expr: Expr::Path(DPath::from("e.name")),
                    alias: Some("employeeName".to_owned()),
                },
                Proj {
                    expr: Expr::Path(DPath::from("e.title")),
                    alias: Some("title".to_owned()),
                },
            ],
            from_clause: vec![Field {
                path: DPath::from("hr.employees"),
                alias: Some("e".to_owned()),
            }],
            left_join_clause: vec![],
            where_clause: Some(DWhereCond::Eq {
                field: Field {
                    path: DPath::from("e.title"),
                    alias: None
                },
                right: "Dev Mgr".to_owned()
            }),
        }
    );
    Ok(())
}

#[test]
fn q2() -> anyhow::Result<()> {
    let sql = get_sql("q2")?;
    dbg!(&sql);

    assert_eq!(
        sql,
        Sql {
            select_clause: vec![
                Proj {
                    expr: Expr::Path(DPath::from("e.name")),
                    alias: Some("employeeName".to_owned()),
                },
                Proj {
                    expr: Expr::Path(DPath::from("p.name")),
                    alias: Some("projectName".to_owned()),
                },
            ],
            from_clause: vec![
                Field {
                    path: DPath::from("hr.employeesNest"),
                    alias: Some("e".to_owned()),
                },
                Field {
                    path: DPath::from("e.projects"),
                    alias: Some("p".to_owned()),
                },
            ],
            left_join_clause: vec![],
            where_clause: Some(DWhereCond::Like {
                field: Field {
                    path: DPath::from("p.name"),
                    alias: None
                },
                right: "%security%".to_owned(),
            }),
        }
    );
    Ok(())
}

#[test]
fn q3() -> anyhow::Result<()> {
    let sql = get_sql("q3")?;
    dbg!(&sql);

    assert_eq!(
        sql,
        Sql {
            select_clause: vec![
                Proj {
                    expr: Expr::Path(DPath::from("e.id")),
                    alias: Some("id".to_owned()),
                },
                Proj {
                    expr: Expr::Path(DPath::from("e.name")),
                    alias: Some("employeeName".to_owned()),
                },
                Proj {
                    expr: Expr::Path(DPath::from("e.title")),
                    alias: Some("title".to_owned()),
                },
                Proj {
                    expr: Expr::Path(DPath::from("p.name")),
                    alias: Some("projectName".to_owned()),
                },
            ],
            from_clause: vec![Field {
                path: DPath::from("hr.employeesNest"),
                alias: Some("e".to_owned()),
            },],
            left_join_clause: vec![Field {
                path: DPath::from("e.projects"),
                alias: Some("p".to_owned()),
            },],
            where_clause: None,
        }
    );
    Ok(())
}

#[test]
fn q4_1() -> anyhow::Result<()> {
    let input = r#"
SELECT e.name AS employeeName,
  ( SELECT p
    FROM e.projects AS p
    WHERE p.name LIKE '%querying%'
  ) AS queryProjectsNum
FROM hr.employeesNest AS e
    "#;
    let sql = parser::sql(&input)?;
    dbg!(&sql);

    assert_eq!(
        sql,
        Sql {
            select_clause: vec![
                Proj {
                    expr: Expr::Path(DPath::from("e.name")),
                    alias: Some("employeeName".to_owned()),
                },
                Proj {
                    expr: Expr::Sql(Sql {
                        select_clause: vec![Proj {
                            expr: Expr::Path(DPath::from("p")),
                            alias: None
                        }],
                        from_clause: vec![Field {
                            path: DPath::from("e.projects"),
                            alias: Some("p".to_owned()),
                        }],
                        left_join_clause: vec![],
                        where_clause: Some(DWhereCond::Like {
                            field: Field {
                                path: DPath::from("p.name"),
                                alias: None
                            },
                            right: "%querying%".to_owned(),
                        }),
                    }),
                    alias: Some("queryProjectsNum".to_owned()),
                },
            ],
            from_clause: vec![Field {
                path: DPath::from("hr.employeesNest"),
                alias: Some("e".to_owned()),
            },],
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
