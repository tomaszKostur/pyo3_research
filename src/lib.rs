use pyo3::prelude::*;
use pyo3::types::PyDict;

/// Formats the sum of two numbers as string.
#[pyfunction]
#[pyo3(text_signature = "(rust usize a, rust usize b, str)")]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

// Example of function that has access to python module
// Has to have &PyModule at it's first argument
#[pyfunction]
#[pyo3(pass_module)]
fn return_module_name(module: &PyModule) -> PyResult<&str> {
    module.name()
}


// function witk kwargs
#[pyfunction(kwargs="**")]
fn kwargs_example(kwargs: Option<&PyDict>) -> PyResult<String> {
    for i in kwargs.unwrap().iter() {
        println!("kwargs_example iter: {:?}", i);
    }
    Ok(String::from("return from kwargs example"))
}

// Research of what python classes are capable of
#[pyclass]
struct RustImplemented {
    some_int: isize,

}

#[pymethods]
impl RustImplemented {
    #[new]
    fn new(v: isize) -> Self { // Why 'Self' starting from capital letter? becouse of 'Self' in python?
        RustImplemented{some_int: v}
    }

    fn add_predefined(&self, input: isize) -> isize {
        self.some_int + input
    }
}

/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
fn module_written_in_rust(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(return_module_name, m)?)?;
    m.add_function(wrap_pyfunction!(kwargs_example, m)?)?;
    m.add_class::<RustImplemented>()?;
    Ok(())
}