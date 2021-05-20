use partiql::parser;

#[test]
fn array123() {
    let input = "[1,2,3]";

    let (input, res) = parser::array(input).unwrap();
    assert_eq!(vec![1, 2, 3], res);
}

#[test]
fn select123() {
    let input = "SELECT * FROM [1,2,3]";

    let res = parser::sql(input);
    assert_eq!(Ok(("", ())), res)
}
