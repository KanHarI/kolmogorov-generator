pub mod kolmogorov_gen;
pub mod turing_machine;

use pyo3::prelude::{PyModule, PyResult, Python};
use crate::kolmogorov_gen::KolmogorovGen;

#[macro_use]
extern crate pyo3;

/// A Python module implemented in Rust.
#[pymodule]
fn kolmogorov_generator(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<KolmogorovGen>()?;
    Ok(())
}

#[cfg(test)]
pub mod tests;
