use super::{Commands, Player, State};

pub struct JumpingState {}

impl JumpingState {
    pub fn new() -> JumpingState {
        JumpingState {}
    }
}

impl State for JumpingState {
    fn handle_input(&self, _command: &Commands, _actor: &mut Player) {}

    fn update(&self) {}
}
