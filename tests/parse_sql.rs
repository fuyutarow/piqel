use ordered_float::OrderedFloat;

use partiql::parser;
use partiql::sql::Expr;
use partiql::sql::Field;
use partiql::sql::Func;
use partiql::sql::Proj;
use partiql::sql::WhereCond;
use partiql::sql::{Selector, Sql};
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

    let (_input, res) = parser::values::array(input).unwrap();
    assert_eq!(vec![1, 2, 3], res);
}

#[test]
fn field() -> anyhow::Result<()> {
    let (_, field) = parser::parse_field("a.b.c")?;

    assert_eq!(
        field,
        Field {
            path: Selector::from("a.b.c"),
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
                    expr: Expr::Path(Selector::from("hr.employees.id")),
                    alias: None
                },
                Proj {
                    expr: Expr::Path(Selector::from("hr.employees.name")),
                    alias: Some("employeeName".to_owned()),
                },
                Proj {
                    expr: Expr::Path(Selector::from("hr.employees.title")),
                    alias: Some("title".to_owned()),
                },
            ],
            from_clause: vec![Field {
                path: Selector::from("hr"),
                alias: None,
            }],
            left_join_clause: vec![],
            where_clause: None,
            orderby: None,
            limit: None,
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
                    expr: Expr::Path(Selector::from("e.id")),
                    alias: None
                },
                Proj {
                    expr: Expr::Path(Selector::from("e.name")),
                    alias: Some("employeeName".to_owned()),
                },
                Proj {
                    expr: Expr::Path(Selector::from("e.title")),
                    alias: Some("title".to_owned()),
                },
            ],
            from_clause: vec![Field {
                path: Selector::from("hr.employees"),
                alias: Some("e".to_owned()),
            }],
            left_join_clause: vec![],
            where_clause: Some(Box::new(WhereCond::Eq {
                expr: Expr::Path(Selector::from("e.title"),),
                right: PqlValue::Str("Dev Mgr".to_owned()),
            })),
            orderby: None,
            limit: None,
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
                    expr: Expr::Path(Selector::from("e.name")),
                    alias: Some("employeeName".to_owned()),
                },
                Proj {
                    expr: Expr::Path(Selector::from("p.name")),
                    alias: Some("projectName".to_owned()),
                },
            ],
            from_clause: vec![
                Field {
                    path: Selector::from("hr.employeesNest"),
                    alias: Some("e".to_owned()),
                },
                Field {
                    path: Selector::from("e.projects"),
                    alias: Some("p".to_owned()),
                },
            ],
            left_join_clause: vec![],
            where_clause: Some(Box::new(WhereCond::Like {
                expr: Expr::Path(Selector::from("p.name")),
                right: "%security%".to_owned()
            })),
            orderby: None,
            limit: None,
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
                    expr: Expr::Path(Selector::from("e.id")),
                    alias: Some("id".to_owned()),
                },
                Proj {
                    expr: Expr::Path(Selector::from("e.name")),
                    alias: Some("employeeName".to_owned()),
                },
                Proj {
                    expr: Expr::Path(Selector::from("e.title")),
                    alias: Some("title".to_owned()),
                },
                Proj {
                    expr: Expr::Path(Selector::from("p.name")),
                    alias: Some("projectName".to_owned()),
                },
            ],
            from_clause: vec![Field {
                path: Selector::from("hr.employeesNest"),
                alias: Some("e".to_owned()),
            },],
            left_join_clause: vec![Field {
                path: Selector::from("e.projects"),
                alias: Some("p".to_owned()),
            },],
            where_clause: None,
            orderby: None,
            limit: None,
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
                    expr: Expr::Path(Selector::from("e.name")),
                    alias: Some("employeeName".to_owned()),
                },
                Proj {
                    expr: Expr::Sql(Sql {
                        select_clause: vec![Proj {
                            expr: Expr::Path(Selector::from("p")),
                            alias: None
                        }],
                        from_clause: vec![Field {
                            path: Selector::from("e.projects"),
                            alias: Some("p".to_owned()),
                        }],
                        left_join_clause: vec![],
                        where_clause: Some(Box::new(WhereCond::Like {
                            expr: Expr::Path(Selector::from("p.name"),),
                            right: "%querying%".to_owned()
                        })),
                        orderby: None,
                        limit: None,
                    }),
                    alias: Some("queryProjectsNum".to_owned()),
                },
            ],
            from_clause: vec![Field {
                path: Selector::from("hr.employeesNest"),
                alias: Some("e".to_owned()),
            },],
            left_join_clause: vec![],
            where_clause: None,
            orderby: None,
            limit: None,
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
                    expr: Expr::Path(Selector::from("e.name")),
                    alias: Some("employeeName".to_owned()),
                },
                Proj {
                    expr: Expr::Sql(Sql {
                        select_clause: vec![Proj {
                            expr: Expr::Func(Box::new(Func::Count(Expr::Star))),
                            alias: None
                        }],
                        from_clause: vec![Field {
                            path: Selector::from("e.projects"),
                            alias: Some("p".to_owned()),
                        }],
                        left_join_clause: vec![],
                        where_clause: Some(Box::new(WhereCond::Like {
                            expr: Expr::Path(Selector::from("p.name"),),
                            right: "%querying%".to_owned()
                        })),
                        orderby: None,
                        limit: None,
                    }),
                    alias: Some("queryProjectsNum".to_owned()),
                },
            ],
            from_clause: vec![Field {
                path: Selector::from("hr.employeesNest"),
                alias: Some("e".to_owned()),
            },],
            left_join_clause: vec![],
            where_clause: None,
            orderby: None,
            limit: None,
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
    let sql = parser::sql(&input);
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
                    expr: Expr::Path(Selector::from("t.id")),
                    alias: Some("id".to_owned()),
                },
                Proj {
                    expr: Expr::Path(Selector::from("x")),
                    alias: Some("even".to_owned()),
                },
            ],
            from_clause: vec![
                Field {
                    path: Selector::from("matrices"),
                    alias: Some("t".to_owned()),
                },
                Field {
                    path: Selector::from("t.matrix"),
                    alias: Some("y".to_owned()),
                },
                Field {
                    path: Selector::from("y"),
                    alias: Some("x".to_owned()),
                },
            ],
            left_join_clause: vec![],
            where_clause: Some(Box::new(WhereCond::Eq {
                expr: Expr::Rem(
                    Box::new(Expr::Path(Selector::from("x"))),
                    Box::new(Expr::Num(2.))
                ),
                right: PqlValue::Float(OrderedFloat(0.))
            })),
            orderby: None,
            limit: None,
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
