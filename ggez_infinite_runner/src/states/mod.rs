pub mod jumping_state;
pub mod standing_state;

use super::Commands;

pub trait State {
    fn handle_input(&self, _command: &Commands, state: &Box<dyn State>) -> Option<Box<dyn State>> {
        None
    }
    fn update(&self) {}
}

pub trait StateData {}
