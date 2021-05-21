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
    let input = std::fs::read_to_string("samples/q1.sql").unwrap();
    let input = input.trim_end();

    let res = parser::sql(input);
    assert_eq!(Ok(("", ())), res)
}

#[test]
fn q2() {
    let input = std::fs::read_to_string("samples/q2.sql").unwrap();
    let input = input.trim_end();

    let res = parser::sql(input);
    assert_eq!(Ok(("", ())), res)
}

#[test]
fn q3() {
    let input = std::fs::read_to_string("samples/q3.sql").unwrap();
    let input = input.trim_end();

    let res = parser::sql(input);
    assert_eq!(Ok(("", ())), res)
}
