use super::player::Player;
use super::MyGame;

pub trait ActorCommand {
    fn execute(&self, actor: &mut Player);
}

pub trait GameCommand {
    fn execute(&self, my_game: &mut MyGame);
}