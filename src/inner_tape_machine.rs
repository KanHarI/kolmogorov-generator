use rand::prelude::StdRng;
use rand::{Rng, SeedableRng};

#[derive(Clone, Copy)]
pub enum TapeAction {
    MoveLeft,
    MoveRight,
    Stay,
}

#[derive(Clone, Copy)]
pub struct Transition {
    pub state: usize,
    pub write_inner: bool,
    pub write_outer: bool,
    pub inner_tape_action: TapeAction,
    pub outer_tape_action: TapeAction,
}

pub struct State {
    pub transitions: [Transition; 4],
}

pub struct InnerTapeMachine {
    pub states: Vec<State>,
    pub inner_tape_size: usize,
    pub outer_tape_size: usize,
    pub outer_start_position: usize,
    pub inner_initialization_symbol: bool,
    pub outer_initialization_symbol: bool,
}

impl InnerTapeMachine {
    pub fn create_random_stack_machine(
        num_states: usize,
        inner_tape_size: usize,
        outer_tape_size: usize,
        seed: [u8; 32],
    ) -> Self {
        let mut rng: StdRng = SeedableRng::from_seed(seed);
        let mut states = Vec::new();
        for _ in 0..num_states {
            let mut transitions = [Transition {
                state: 0,
                write_inner: false,
                write_outer: false,
                inner_tape_action: TapeAction::Stay,
                outer_tape_action: TapeAction::Stay,
            }; 4];
            for transition in &mut transitions {
                transition.state = rng.gen_range(0..num_states);
                transition.write_inner = rng.gen();
                transition.write_outer = rng.gen();
                transition.inner_tape_action = match rng.gen_range(0..3) {
                    0 => TapeAction::MoveLeft,
                    1 => TapeAction::MoveRight,
                    2 => TapeAction::Stay,
                    _ => unreachable!(),
                };
                transition.outer_tape_action = match rng.gen_range(0..3) {
                    0 => TapeAction::MoveLeft,
                    1 => TapeAction::MoveRight,
                    2 => TapeAction::Stay,
                    _ => unreachable!(),
                };
            }
            states.push(State { transitions });
        }
        Self {
            states,
            inner_tape_size,
            outer_tape_size,
            outer_start_position: rng.gen_range(0..outer_tape_size),
            inner_initialization_symbol: rng.gen(),
            outer_initialization_symbol: rng.gen(),
        }
    }

    pub fn run_machine(&self, max_steps: usize) -> Vec<bool> {
        let mut inner_tape = vec![self.inner_initialization_symbol; self.inner_tape_size];
        let mut outer_tape = vec![self.outer_initialization_symbol; self.outer_tape_size];
        let mut state = 0;
        let mut step = 0;
        let mut inner_pos = 0;
        let mut outer_pos = self.outer_start_position;
        while step < max_steps {
            let machine_state = &self.states[state];
            let inner_tape_val = inner_tape[inner_pos];
            let outer_tape_val = outer_tape[outer_pos];
            let transition = if inner_tape_val && outer_tape_val {
                &machine_state.transitions[0]
            } else if inner_tape_val && !outer_tape_val {
                &machine_state.transitions[1]
            } else if !inner_tape_val && outer_tape_val {
                &machine_state.transitions[2]
            } else {
                &machine_state.transitions[3]
            };
            state = transition.state;
            inner_tape[inner_pos] = transition.write_inner;
            outer_tape[outer_pos] = transition.write_outer;
            match transition.inner_tape_action {
                TapeAction::MoveLeft => {
                    if inner_pos > 0 {
                        inner_pos -= 1;
                    } else {
                        inner_pos = self.inner_tape_size - 1;
                    }
                }
                TapeAction::MoveRight => {
                    if inner_pos < self.inner_tape_size - 1 {
                        inner_pos += 1;
                    } else {
                        inner_pos = 0;
                    }
                }
                TapeAction::Stay => {}
            }
            match transition.outer_tape_action {
                TapeAction::MoveLeft => {
                    if outer_pos > 0 {
                        outer_pos -= 1;
                    } else {
                        outer_pos = self.outer_tape_size - 1;
                    }
                }
                TapeAction::MoveRight => {
                    if outer_pos < self.outer_tape_size - 1 {
                        outer_pos += 1;
                    } else {
                        outer_pos = 0;
                    }
                }
                TapeAction::Stay => {}
            }
            step += 1;
        }
        outer_tape
    }
}
