use pyo3::prelude::*;

#[doc = "MyClass(value)\n--\n"]
/// MyClass docstring
#[pyclass(module = "rust_python_test")]
struct MyClass {
    value: i32,
}

#[pymethods]
impl MyClass {
    #[doc = "__new__($cls, value)\n--\n"]
    /// MyClass.__new__ docstring
    #[new]
    fn __new__(obj: &PyRawObject, value: i32) {
        obj.init(Self { value });
    }
    /// MyClass.value getter docstring
    #[getter]
    fn value(&self) -> i32 {
        self.value
    }
    /// MyClass.value setter docstring
    #[setter(value)]
    fn set_value(&mut self, value: i32) {
        self.value = value;
    }
    #[doc = "member_fn($self, arg = 5, *, arg2 = None)\n--\n"]
    /// MyClass.member_fn docstring
    #[args(arg = 5, "*", arg2 = "None")]
    fn member_fn(&self, arg: i32, arg2: Option<i32>) {
        println!(
            "arg = {}, arg2 = {}",
            arg,
            arg2.map_or_else(|| "None".to_string(), |arg2| arg2.to_string())
        );
    }
}

/// rust_python_test docstring
#[pymodule]
fn rust_python_test(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<MyClass>()?;
    Ok(())
}
