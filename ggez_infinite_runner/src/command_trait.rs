use super::player::Player;
use super::Obstacle;
use super::GameState;
use super::Score;

pub trait ActorCommand {
    fn execute(&self, actor: &mut Player);
}

pub trait GameCommand {
    fn execute(&self, player: &mut Player, score: &mut Score, obstacle_1: &mut Obstacle, obstacle_2: &mut Obstacle, game_state: &mut GameState);
}