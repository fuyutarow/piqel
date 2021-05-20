use partiql::models;

fn main() {
    let data = vec![1, 2, 3];
    let arr = models::Array::from(data.as_slice());

    dbg!(&arr);

    let atom = models::Atom::from(34);
    println!("{}", atom);
    println!("{}", arr);

    let v = vec![
        models::Atom::from(1),
        models::Atom::from(2),
        models::Atom::from(3),
    ];

    // println!("{}", &v);
}
