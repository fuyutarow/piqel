use pyo3::prelude::*;
use pyo3::types::*;
use pythonize::{depythonize, pythonize};

#[pymodule]
fn partiql(py: Python, m: &PyModule) -> PyResult<()> {
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;

    #[pyfn(m, "loads")]
    fn loads(py: Python, input: &str, from: &str) -> PyResult<Py<PyAny>> {
        let data = partiql::engine::loads(input, from).expect("load");
        let obj = pythonize(py, &data).unwrap();
        Ok(obj)
    }

    #[pyfn(m, "dumps")]
    fn dumps(py: Python, obj: Py<PyAny>, to: &str) -> PyResult<String> {
        let data = depythonize(obj.as_ref(py)).unwrap();
        let output = partiql::engine::dumps(data, to).expect("dump");
        Ok(output)
    }

    #[pyfn(m, "query_evaluate")]
    fn query_evaluate(py: Python, obj: Py<PyAny>, q: &str) -> PyResult<Py<PyAny>> {
        let data = depythonize(obj.as_ref(py)).unwrap();
        let data = partiql::engine::query_evaluate(data, q).expect("query evaluate");
        let obj = pythonize(py, &data).unwrap();
        Ok(obj)
    }

    Ok(())
}
