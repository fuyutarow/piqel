use partiql::models;

fn main() {
    let data = vec![1, 2, 3];
    let arr = models::Array::from(data.as_slice());

    dbg!(&arr);

    let atom = models::Atom::from(34);
    println!("{}", atom);
    println!("{}", arr);

    let input = std::fs::read_to_string("samples/q1.env").unwrap();
    println!("{}", input);

    // println!("{}", &v);
}
