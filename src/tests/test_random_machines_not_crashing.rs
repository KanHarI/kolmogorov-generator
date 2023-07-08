use crate::inner_tape_machine::InnerTapeMachine;
use crate::turing_machine::TuringMachine;
use rand::rngs::ThreadRng;
use rand::Rng;

#[test]
pub fn test_random_turing_machines_not_crashing() {
    let mut rng = ThreadRng::default();
    let num_tests = 1_000;
    let tape_size = 64;
    for _ in 0..num_tests {
        let num_states = rng.gen_range(2..32);
        let num_steps = rng.gen_range(1..10_000);
        let seed: [u8; 32] = rng.gen();
        let turing_machine =
            TuringMachine::create_random_turing_machine(num_states, tape_size, seed);
        turing_machine.run_machine(num_steps);
    }
}

#[test]
pub fn test_random_inner_tape_machines_not_crashing() {
    let mut rng = ThreadRng::default();
    let num_tests = 1_000;
    for _ in 0..num_tests {
        let num_states = rng.gen_range(2..32);
        let inner_tape_size = rng.gen_range(2..32);
        let outer_tape_size = rng.gen_range(2..32);
        let num_steps = rng.gen_range(1..10_000);
        let seed: [u8; 32] = rng.gen();
        let turing_machine = InnerTapeMachine::create_random_stack_machine(
            num_states,
            inner_tape_size,
            outer_tape_size,
            seed,
        );
        turing_machine.run_machine(num_steps);
    }
}
