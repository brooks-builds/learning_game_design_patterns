use super::command_trait::GameCommand;
use super::Obstacle;
use super::Player;
use super::WrappedGameState;
use super::WrappedScore;

pub struct ResetGameCommand {}

impl ResetGameCommand {
    pub fn new() -> ResetGameCommand {
        ResetGameCommand {}
    }
}

impl GameCommand for ResetGameCommand {
    fn execute(
        &self,
        player: &mut Player,
        wrapped_score: WrappedScore,
        obstacle_1: &mut Obstacle,
        obstacle_2: &mut Obstacle,
        wrapped_game_state: WrappedGameState,
    ) {
        player.reset_location();
        wrapped_score.lock().unwrap().reset();
        obstacle_1.reset_to_start();
        obstacle_2.reset_to_start();
        wrapped_game_state.lock().unwrap().playing();
    }
}
