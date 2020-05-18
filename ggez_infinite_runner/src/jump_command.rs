use super::command_trait::Command;
use super::player::Player;

pub struct JumpCommand {

}

impl JumpCommand {
    pub fn new() -> JumpCommand {
        JumpCommand{}
    }
}

impl Command for JumpCommand {
    fn execute(&self, actor: &mut Player) {
        actor.jump();
    }
}