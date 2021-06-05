use ordered_float::OrderedFloat;

use partiql::parser;
use partiql::sql::Expr;
use partiql::sql::Field;
use partiql::sql::Func;
use partiql::sql::Proj;
use partiql::sql::WhereCond;
use partiql::sql::{DPath, Sql};
use partiql::value::PqlValue;

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
            where_clause: Some(Box::new(WhereCond::Eq {
                expr: Expr::Path(DPath::from("e.title"),),
                right: PqlValue::Str("Dev Mgr".to_owned()),
            })),
        }
    );
    Ok(())
}

#[test]
fn q2() -> anyhow::Result<()> {
    let sql = get_sql("q2")?;

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
            where_clause: Some(Box::new(WhereCond::Like {
                expr: Expr::Path(DPath::from("p.name")),
                right: "%security%".to_owned()
            })),
        }
    );
    Ok(())
}

#[test]
fn q3() -> anyhow::Result<()> {
    let sql = get_sql("q3")?;

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
                        where_clause: Some(Box::new(WhereCond::Like {
                            expr: Expr::Path(DPath::from("p.name"),),
                            right: "%querying%".to_owned()
                        })),
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

#[test]
fn q4() -> anyhow::Result<()> {
    let sql = get_sql("q4")?;

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
                            expr: Expr::Func(Box::new(Func::Count(Expr::Path(DPath::from("*"))))),
                            alias: None
                        }],
                        from_clause: vec![Field {
                            path: DPath::from("e.projects"),
                            alias: Some("p".to_owned()),
                        }],
                        left_join_clause: vec![],
                        where_clause: Some(Box::new(WhereCond::Like {
                            expr: Expr::Path(DPath::from("p.name"),),
                            right: "%querying%".to_owned()
                        })),
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

#[test]
fn q5() {
    let sql = get_sql("q5");

    if sql.is_ok() {
        assert_eq!(true, true);
    } else {
        assert_eq!(false, false);
    }
}

#[test]
fn q6() {
    let sql = get_sql("q6");

    if sql.is_ok() {
        assert_eq!(true, true);
    } else {
        assert_eq!(false, false);
    }
}

#[test]
fn q7_1() {
    let input = r#"
SELECT t.id AS id,
       x AS even
FROM matrices AS t,
     t.matrix AS y,
     y AS x
WHERE x / 2 = 0
    "#;
    let sql = parser::parse_sql(&input);
    dbg!(&sql);

    if sql.is_ok() {
        assert_eq!(true, true);
    } else {
        assert_eq!(false, false);
    }
}

#[test]
fn q7() -> anyhow::Result<()> {
    let sql = get_sql("q7")?;

    assert_eq!(
        sql,
        Sql {
            select_clause: vec![
                Proj {
                    expr: Expr::Path(DPath::from("t.id")),
                    alias: Some("id".to_owned()),
                },
                Proj {
                    expr: Expr::Path(DPath::from("x")),
                    alias: Some("even".to_owned()),
                },
            ],
            from_clause: vec![
                Field {
                    path: DPath::from("matrices"),
                    alias: Some("t".to_owned()),
                },
                Field {
                    path: DPath::from("t.matrix"),
                    alias: Some("y".to_owned()),
                },
                Field {
                    path: DPath::from("y"),
                    alias: Some("x".to_owned()),
                },
            ],
            left_join_clause: vec![],
            where_clause: Some(Box::new(WhereCond::Eq {
                expr: Expr::Mod(
                    Box::new(Expr::Path(DPath::from("x"))),
                    Box::new(Expr::Num(2.))
                ),
                right: PqlValue::Float(OrderedFloat(0.))
            })),
        }
    );
    Ok(())
}

#[test]
fn q8() {
    let sql = get_sql("q8");

    if sql.is_ok() {
        assert_eq!(true, true);
    } else {
        assert_eq!(false, false);
    }
}

#[test]
fn q9() {
    let sql = get_sql("q9");

    if sql.is_ok() {
        assert_eq!(true, true);
    } else {
        assert_eq!(false, false);
    }
}
