use super::jumping_state::JumpingState;
use super::{Commands, Player, State};

pub struct StandingState {}

impl StandingState {
    pub fn new() -> StandingState {
        StandingState {}
    }
}

impl State for StandingState {
    fn handle_input(&self, command: &Commands, actor: &mut Player) {
        if let Commands::Jump = command {
            actor.state = JumpingState::new();
        }
    }

    fn update(&self) {}
}
