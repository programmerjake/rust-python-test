#[cfg(feature = "python")]
use pyo3::prelude::*;

#[cfg(feature = "python-extension")]
#[pymodule]
fn rust_python_test(py: Python, m: &PyModule) -> PyResult<()> {
    #[pyfn(m, "my_function")]
    fn my_function(_py: Python, i: String) -> PyResult<String> {
        Ok(i + " suffix")
    }
    Ok(())
}
