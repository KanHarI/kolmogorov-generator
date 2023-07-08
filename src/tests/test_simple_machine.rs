use super::super::turing_machine::{TapeAction, TuringMachine, TuringMachineTransition};

#[test]
pub fn test_simple_turing_machine() {
    let simple_turing_machine = TuringMachine::new(
        vec![
            TuringMachineTransition {
                zero_state: 1,
                one_state: 1,
                zero_write: true,
                one_write: true,
                zero_action: TapeAction::MoveRight,
                one_action: TapeAction::MoveRight,
            },
            TuringMachineTransition {
                zero_state: 2,
                one_state: 2,
                zero_write: true,
                one_write: true,
                zero_action: TapeAction::MoveRight,
                one_action: TapeAction::MoveRight,
            },
            TuringMachineTransition {
                zero_state: 2,
                one_state: 2,
                zero_write: false,
                one_write: true,
                zero_action: TapeAction::MoveRight,
                one_action: TapeAction::MoveRight,
            },
        ],
        4,
        0,
        false,
    );
    assert_eq!(
        simple_turing_machine.run_machine(100),
        vec![true, true, false, false]
    );
}
