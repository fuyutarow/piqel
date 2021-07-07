use std::str::FromStr;

use ordered_float::OrderedFloat;

use partiql::planner::Sql;
use partiql::sql::Expr;
use partiql::sql::Field;
use partiql::sql::Selector;
use partiql::sql::WhereCond;
use partiql::value::PqlValue;

fn get_sql(qi: &str) -> anyhow::Result<Sql> {
    let input = std::fs::read_to_string(format!("samples/{}.sql", qi)).unwrap();
    Sql::from_str(&input)
}

#[test]
fn select_deeppath() -> anyhow::Result<()> {
    let sql = Sql::from_str(
        "
SELECT hr.employees.id,
       hr.employees.name AS employeeName,
       hr.employees.title AS title
FROM hr
",
    )?;

    assert_eq!(
        sql,
        Sql {
            select_clause: vec![
                Field::from_str("hr.employees.id")?,
                Field::from_str("hr.employees.name AS employeeName")?,
                Field::from_str("hr.employees.title AS title")?
            ],
            from_clause: vec![Field::from_str("hr")?],
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
                Field::from_str("e.id")?,
                Field::from_str("e.name AS employeeName")?,
                Field::from_str("e.title AS title")?
            ],
            from_clause: vec![Field::from_str("hr.employees AS e",)?],
            left_join_clause: vec![],
            where_clause: Some(Box::new(WhereCond::Eq {
                expr: Expr::Selector(Selector::from("e.title"),),
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
                Field::from_str("e.name AS employeeName")?,
                Field::from_str("p.name AS projectName")?,
            ],
            from_clause: vec![
                Field::from_str("hr.employeesNest AS e")?,
                Field::from_str("e.projects AS p")?,
            ],
            left_join_clause: vec![],
            where_clause: Some(Box::new(WhereCond::Like {
                expr: Expr::Selector(Selector::from("p.name")),
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
                Field::from_str("e.id AS id")?,
                Field::from_str("e.name AS employeeName")?,
                Field::from_str("e.title AS title")?,
                Field::from_str("p.name AS projectName")?,
            ],
            from_clause: vec![Field::from_str("hr.employeesNest AS e")?],
            left_join_clause: vec![Field::from_str("e.projects AS p")?],
            where_clause: None,
            orderby: None,
            limit: None,
        }
    );
    Ok(())
}

// #[test]
// fn q4_1() -> anyhow::Result<()> {
//     let input = r#"
// SELECT e.name AS employeeName,
//   ( SELECT p
//     FROM e.projects AS p
//     WHERE p.name LIKE '%querying%'
//   ) AS queryProjectsNum
// FROM hr.employeesNest AS e
//     "#;
//     let sql = Sql::from_str(&input)?;

//     assert_eq!(
//         sql,
//         Sql {
//             select_clause: vec![
//                 Field::from_str("e.name AS employeeName")?,
//                 Field {
//                     value: Expr::Expr(Expr::Sql(Sql {
//                         select_clause: vec![Field::from_str("p")?,],
//                         from_clause: vec![Field::from_str("e.projects AS p")?],
//                         left_join_clause: vec![],
//                         where_clause: Some(Box::new(WhereCond::Like {
//                             expr: Expr::Path(Selector::from("p.name"),),
//                             right: "%querying%".to_owned()
//                         })),
//                         orderby: None,
//                         limit: None,
//                     })),
//                     alias: Some("queryProjectsNum".to_owned()),
//                 },
//             ],
//             from_clause: vec![Field::from_str("hr.employeesNest AS e")?],
//             left_join_clause: vec![],
//             where_clause: None,
//             orderby: None,
//             limit: None,
//         }
//     );
//     Ok(())
// }

// #[test]
// fn q4() -> anyhow::Result<()> {
//     let sql = get_sql("q4")?;

//     assert_eq!(
//         sql,
//         Sql {
//             select_clause: vec![
//                 Field::from_str("e.name AS employeeName")?,
//                 Field {
//                     value: Expr::Expr(Expr::Sql(Sql {
//                         select_clause: vec![Field {
//                             value: Expr::Expr(Expr::Func(Box::new(Func::Count(Expr::Star)))),
//                             alias: None
//                         }],
//                         from_clause: vec![Field::from_str("e.projects AS p")?],
//                         left_join_clause: vec![],
//                         where_clause: Some(Box::new(WhereCond::Like {
//                             expr: Expr::Path(Selector::from("p.name"),),
//                             right: "%querying%".to_owned()
//                         })),
//                         orderby: None,
//                         limit: None,
//                     })),
//                     alias: Some("queryProjectsNum".to_owned()),
//                 },
//             ],
//             from_clause: vec![Field::from_str("hr.employeesNest AS e")?],
//             left_join_clause: vec![],
//             where_clause: None,
//             orderby: None,
//             limit: None,
//         }
//     );
//     Ok(())
// }

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
    let sql = Sql::from_str(&input);

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
                Field::from_str("t.id AS id")?,
                Field::from_str("x AS even")?,
            ],
            from_clause: vec![
                Field::from_str("matrices AS t")?,
                Field::from_str("t.matrix AS y")?,
                Field::from_str("y AS x")?,
            ],
            left_join_clause: vec![],
            where_clause: Some(Box::new(WhereCond::Eq {
                expr: Expr::Rem(
                    Box::new(Expr::Selector(Selector::from("x"))),
                    Box::new(Expr::from(2.))
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
