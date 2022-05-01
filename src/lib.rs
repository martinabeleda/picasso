use pyo3::prelude::*;
use pyo3::{
    wrap_pyfunction,
};
use std::str::FromStr;
use wkt::Wkt;


/// A Python module implemented in Rust.
#[pymodule]
fn picasso(py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    register_wkt(py, m);
    Ok(())
}

// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

fn register_wkt(py: Python<'_>, parent_module: &PyModule) -> PyResult<()> {
    let wkt = PyModule::new(py, "wkt")?;
    wkt.add_function(wrap_pyfunction!(loads, wkt)?)?;
    parent_module.add_submodule(wkt)?;
    Ok(())
}

// Load a geometry from a WKT string
#[pyfunction]
pub fn loads(data: String) -> PyResult<()> {
     let wkt: Wkt<f64> = Wkt::from_str(&data).unwrap();
     println!("Parsed obj: {:?}", wkt);
     
     Ok(())
}
