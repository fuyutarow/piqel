use pyo3::prelude::*;

#[pyclass]
struct Point2 {
    #[pyo3(get)]
    x: f64,
    #[pyo3(get)]
    y: f64,
}

#[pymethods]
impl Point2 {
    #[new]
    pub fn new(x: f64, y: f64) -> Self {
        Point2 { x, y }
    }
}

#[pymodule]
fn partiql(_: Python, m: &PyModule) -> PyResult<()> {
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;
    m.add_class::<Point2>()?;

    #[pyfn(m, "evaluate")]
    fn evaluate(sql: &str, input: &str, from: &str, to: &str) -> PyResult<String> {
        let res = partiql::engine::evaluate(sql, input, from, to);
        Ok(res.unwrap())
    }

    Ok(())
}
