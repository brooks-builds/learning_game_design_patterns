use super::jumping_state::JumpingState;
use super::{Commands, Player, State};

pub struct StandingState {}

impl StandingState {
    pub fn new() -> StandingState {
        StandingState {}
    }
}

impl State for StandingState {
    fn handle_input(&self, command: &Commands, player: &mut Player) -> Option<Box<dyn State>> {
        if let Commands::Jump = command {
            return Some(Box::new(JumpingState::new()));
        }

        None
    }

    fn update(&self) {}
}
