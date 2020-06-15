use super::jumping_state::JumpingState;
use super::{Commands, State, StateData};

pub struct StandingState {}

impl StandingState {
    pub fn new() -> StandingState {
        StandingState {}
    }
}

impl State for StandingState {
    fn handle_input(&self, command: &Commands, state: &Box<dyn State>) -> Option<Box<dyn State>> {
        if let Commands::Jump = command {
            return Some(Box::new(JumpingState::new()));
        }

        None
    }

    fn update(&self) {}
}
