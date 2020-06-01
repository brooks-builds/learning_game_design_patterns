use super::command_trait::GameCommand;
use super::GameState;
use super::Obstacle;
use super::Player;
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
        game_state: &mut GameState,
    ) {
        player.reset_location();
        wrapped_score.lock().unwrap().reset();
        obstacle_1.reset_to_start();
        obstacle_2.reset_to_start();
        game_state.playing();
    }
}
