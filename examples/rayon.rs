use rayon::prelude::*;

fn main() {
    let par_iter = (10..20)
        .collect::<Vec<i32>>()
        .into_par_iter()
        .map(|x| x * 2)
        .inspect(|e| {
            dbg!(e);
        });

    // par_iter.for_each(|x| {
    //     println!("item: {}", x);
    // });
    let v = par_iter.collect::<Vec<_>>();
    dbg!(&v);
}
