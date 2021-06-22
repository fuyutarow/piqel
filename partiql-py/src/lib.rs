use pyo3::prelude::*;
use pyo3::types::IntoPyDict;
use pyo3::{wrap_pyfunction, wrap_pymodule};

#[pyfunction]
fn subfunction() -> String {
    "Subfunction".to_string()
}

fn init_submodule(module: &PyModule) -> PyResult<()> {
    module.add_function(wrap_pyfunction!(subfunction, module)?)?;
    Ok(())
}

#[pymodule]
fn supermodule(py: Python, module: &PyModule) -> PyResult<()> {
    let submod = PyModule::new(py, "submodule")?;
    init_submodule(submod)?;
    module.add_submodule(submod)?;
    Ok(())
}
