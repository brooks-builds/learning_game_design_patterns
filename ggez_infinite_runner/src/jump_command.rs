use super::command_trait::ActorCommand;
use super::player::Player;

pub struct JumpCommand {

}

impl JumpCommand {
    pub fn new() -> JumpCommand {
        JumpCommand{}
    }
}

impl ActorCommand for JumpCommand {
    fn execute(&self, actor: &mut Player) {
        actor.jump();
    }
}