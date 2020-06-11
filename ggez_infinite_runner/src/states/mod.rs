pub mod jumping_state;
pub mod standing_state;

use super::{Commands, Player};

pub trait State {
    fn handle_input(&self, command: &Commands, actor: &mut Player) {}
    fn update(&self) {}
}
