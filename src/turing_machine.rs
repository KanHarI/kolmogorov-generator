use rand::prelude::{SeedableRng, StdRng};
use rand::Rng;

pub enum TapeAction {
    MoveLeft,
    MoveRight,
}

pub struct TuringMachineTransition {
    pub zero_state: usize,
    pub one_state: usize,
    pub zero_write: bool,
    pub one_write: bool,
    pub tape_action: TapeAction,
}

pub struct TuringMachine {
    // Turing machines always start on state no. 1 and halt when they reach state 0
    tape_size: usize,
    start_position: usize,
    initialization_symbol: bool,
    transitions: Vec<TuringMachineTransition>,
}

impl TuringMachine {
    pub fn new(transitions: Vec<TuringMachineTransition>, tape_size: usize, start_position: usize, initialization_symbol: bool) -> Self {
        if tape_size < 2 || transitions.len() < 2 {
            panic!("Turing machine must have at least 2 states and a tape size of at least 2");
        }
        Self {
            tape_size,
            transitions,
            start_position,
            initialization_symbol,
        }
    }

    pub fn create_random_turing_machine(
        num_states: usize,
        tape_size: usize,
        seed: [u8; 32],
    ) -> Self {
        let mut rng: StdRng = SeedableRng::from_seed(seed);
        let mut transitions = Vec::new();
        for _ in 0..num_states {
            transitions.push(TuringMachineTransition {
                zero_state: rng.gen_range(0..num_states),
                one_state: rng.gen_range(0..num_states),
                zero_write: rng.gen(),
                one_write: rng.gen(),
                tape_action: if rng.gen::<bool>() {
                    TapeAction::MoveLeft
                } else {
                    TapeAction::MoveRight
                },
            });
        }
        Self::new(transitions, tape_size, rng.gen_range(0..tape_size), rng.gen())
    }

    pub fn run_machine(&self, max_steps: usize) -> Vec<bool> {
        let mut tape = vec![self.initialization_symbol; self.tape_size];
        let mut state = 1;
        let mut step = 0;
        let mut pos = self.start_position;
        while state != 0 && step < max_steps {
            let transition = &self.transitions[state];
            let tape_val = tape[pos];
            state = if tape_val {
                transition.one_state
            } else {
                transition.zero_state
            };
            tape[pos] = if tape_val {
                transition.one_write
            } else {
                transition.zero_write
            };
            match transition.tape_action {
                TapeAction::MoveLeft => {
                    if pos == 0 {
                        pos = self.tape_size - 1;
                    } else {
                        pos -= 1;
                    }
                }
                TapeAction::MoveRight => {
                    if pos == self.tape_size - 1 {
                        pos = 0;
                    } else {
                        pos += 1;
                    }
                }
            }
            step += 1;
        }
        tape
    }
}
