extern crate nalgebra as na;
use na::{Dynamic, SMatrix};
use ordered_float::OrderedFloat;

use partiql::sql::Expr;
use partiql::value::PqlValue;

fn main() -> anyhow::Result<()> {
    let v = SMatrix::<f64, 3, 1>::new(1., 2., 3.);
    dbg!(v);

    let r = SMatrix::<f64, 1, 3>::new(1., 2., 3.);
    dbg!(r);

    dbg!(r * v);
    dbg!(v * r);

    let v = SMatrix::<PqlValue, 3, 1>::new(
        PqlValue::Float(OrderedFloat(1.)),
        PqlValue::Float(OrderedFloat(2.)),
        PqlValue::Float(OrderedFloat(3.)),
    );
    let w = v.clone();
    let r = PqlValue::Float(OrderedFloat(1.)) + PqlValue::Float(OrderedFloat(2.));
    let r = PqlValue::Float(OrderedFloat(1.)) / PqlValue::Float(OrderedFloat(0.));
    dbg!(r);

    Ok(())
}
