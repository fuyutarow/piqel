use partiql::sql_parser as parser;

#[test]
fn array123() {
    let input = "[1,2,3]";

    let (input, res) = parser::array(input).unwrap();
    assert_eq!(vec![1, 2, 3], res);
}

// #[test]
// fn select123() {
//     let input = "SELECT * FROM [1,2,3]";

//     let res = parser::sql(input);
//     assert_eq!(Ok(("", ())), res)
// }

#[test]
fn q1() {
    let input = r#"
    SELECT e.id,
        e.name AS employeeName,
        e.title AS title
    FROM hr.employees AS e
    WHERE e.title = 'Dev Mgr'"#;

    let res = parser::sql(input);
    assert_eq!(Ok(("", ())), res)
}
