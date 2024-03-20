#![allow(unused)]
use pyo3::prelude::*;

use numpy::ndarray::prelude::*;
use pyo3::wrap_pyfunction;

#[pyfunction]
/// Formats the sum of two numbers as string.
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

#[pyfunction]
fn sma(a: Vec<f32>) -> PyResult<Vec<f32>> {
    let prices = Array::from_vec(a);
    let length = prices.len();
    let mut result = Array1::<f32>::zeros(length);
    result[0] = prices[0];

    for i in 1..length {
        result[i] = result[i - 1] + prices[i];
    }

    Ok(Array::to_vec(&result))
}
#[pymodule]
// A Python module implemented in Rust.
fn string_sum(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(sum_as_string))?;
    m.add_wrapped(wrap_pyfunction!(sma))?;

    Ok(())
}
