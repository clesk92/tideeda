use pyo3::prelude::*;

#[pyfunction]
fn synthesize(_uhdm_path: String, _output_path: String) -> PyResult<()> {
    // Placeholder for synthesis flow
    Ok(())
}

#[pymodule]
fn tideeda(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(synthesize, m)?)?;
    Ok(())
}
