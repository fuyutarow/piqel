use partiql::models;

fn main() {
    let data = vec![1, 2, 3];
    let arr = models::Array::from(data.as_slice());

    dbg!(arr);
}
