use super::player::Player;
use super::GameState;
use super::Obstacle;
use super::WrappedScore;

pub trait ActorCommand {
    fn execute(&self, actor: &mut Player);
}

pub trait GameCommand {
    fn execute(
        &self,
        player: &mut Player,
        wrapped_score: WrappedScore,
        obstacle_1: &mut Obstacle,
        obstacle_2: &mut Obstacle,
        game_state: &mut GameState,
    );
}
