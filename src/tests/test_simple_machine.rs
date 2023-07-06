use super::super::turing_machine::{TapeAction, TuringMachine, TuringMachineTransition};

#[test]
pub fn test_simple_turing_machine() {
    let simple_turing_machine = TuringMachine::new(
        vec![
            TuringMachineTransition {
                zero_state: 0,
                one_state: 0,
                zero_write: true,
                one_write: true,
                tape_action: TapeAction::MoveRight,
            },
            TuringMachineTransition {
                zero_state: 2,
                one_state: 2,
                zero_write: true,
                one_write: true,
                tape_action: TapeAction::MoveRight,
            },
            TuringMachineTransition {
                zero_state: 0,
                one_state: 0,
                zero_write: true,
                one_write: true,
                tape_action: TapeAction::MoveRight,
            },
        ],
        4,
    );
    assert_eq!(
        simple_turing_machine.run_machine(100),
        vec![true, true, false, false]
    );
}
