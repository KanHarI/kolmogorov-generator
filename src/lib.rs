pub mod inner_tape_machine;
pub mod kolmogorov_gen;
pub mod kolmogorov_inner_state_gen;
pub mod turing_machine;

use crate::kolmogorov_gen::KolmogorovGen;
use crate::kolmogorov_inner_state_gen::KolmogorovInnerStateGen;
use pyo3::prelude::{PyModule, PyResult, Python};

#[macro_use]
extern crate pyo3;

/// A Python module implemented in Rust.
#[pymodule]
fn kolmogorov_generator(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<KolmogorovGen>()?;
    m.add_class::<KolmogorovInnerStateGen>()?;
    Ok(())
}

#[cfg(test)]
pub mod tests;
