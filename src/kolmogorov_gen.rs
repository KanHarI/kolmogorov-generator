use crate::turing_machine::TuringMachine;
use numpy::ndarray::Array1;
use numpy::{IntoPyArray, Ix1, PyArray};
use pyo3::Python;
use rand::prelude::{SeedableRng, StdRng};
use rand::Rng;

#[pyclass]
pub struct KolmogorovGen {
    tape_size: usize,
    states_discount_rate: f64,
    max_states: usize,
    max_steps: usize,
    filter_uniform_outputs: bool,
    rng: StdRng,
}

#[pymethods]
impl KolmogorovGen {
    #[new]
    pub fn new(
        tape_size: usize,
        states_discount_rate: f64,
        max_states: usize,
        max_steps: usize,
        filter_uniform_outputs: bool,
        seed: Option<[u8; 32]>,
    ) -> Self {
        Self {
            tape_size,
            states_discount_rate,
            max_states,
            max_steps,
            filter_uniform_outputs,
            rng: if let Some(seed) = seed {
                StdRng::from_seed(seed)
            } else {
                StdRng::from_entropy()
            },
        }
    }

    fn _generate(&mut self) -> Vec<bool> {
        let mut num_states = 2;
        while num_states < self.max_states && self.rng.gen::<f64>() < self.states_discount_rate {
            num_states += 1;
        }
        loop {
            let turing_machine = TuringMachine::create_random_turing_machine(
                num_states,
                self.tape_size,
                self.rng.gen(),
            );
            let result = turing_machine.run_machine(self.max_steps);
            if !self.filter_uniform_outputs || result.iter().any(|&x| x != result[0]) {
                return result;
            }
        }
    }

    pub fn generate<'py>(&mut self, py: Python<'py>) -> &'py PyArray<bool, Ix1> {
        let result = self._generate();
        let result = Array1::from(result);
        result.into_pyarray(py)
    }
}
