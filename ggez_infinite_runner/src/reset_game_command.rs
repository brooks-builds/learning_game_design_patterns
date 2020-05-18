use super::command_trait::GameCommand;
use super::MyGame;
use super::GameState;

pub struct ResetGameCommand {}

impl ResetGameCommand {
    pub fn new() -> ResetGameCommand {
        ResetGameCommand {}
    }
}

impl GameCommand for ResetGameCommand {
    fn execute(&self, my_game: &mut MyGame) {
        my_game.player.reset_location();
        my_game.score = 0;
        my_game.obstacle_1.reset_to_start();
        my_game.obstacle_2.reset_to_start();
        my_game.game_state = GameState::Playing;
    }
}