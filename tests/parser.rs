use partiql::parser;

#[test]
fn select123() {
    let input = "SELECT * FROM [1,2,3]";

    let res = parser::sql(input);
    assert_eq!(Ok(("", ())), res)
}
