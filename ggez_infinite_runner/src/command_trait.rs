use super::player::Player;

pub trait Command {
    fn execute(&self, actor: &mut Player);
}