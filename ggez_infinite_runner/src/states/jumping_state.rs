use super::{Commands, State};

pub struct JumpingState {}

impl JumpingState {
    pub fn new() -> JumpingState {
        JumpingState {}
    }
}

impl State for JumpingState {
    fn handle_input(&self, _command: &Commands, state: &Box<dyn State>) -> Option<Box<dyn State>> {
        None
    }

    fn update(&self) {}
}
