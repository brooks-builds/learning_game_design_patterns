mod errors;
pub mod game_data;
mod states;
mod types;

pub use errors::CustomError;
use game_data::GameData;
use ggez::event::EventHandler;
use ggez::graphics::Color;
use ggez::{graphics, Context, GameResult};
pub use states::States;
pub use types::Types;

pub struct GameState {
    background_color: Color,
    game_data: GameData,
}

impl GameState {
    pub fn new(game_data: GameData) -> GameState {
        GameState {
            background_color: Color::from_rgb(0, 51, 102),
            game_data,
        }
    }
}

impl EventHandler for GameState {
    fn update(&mut self, _context: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> GameResult {
        graphics::clear(context, self.background_color);

        graphics::present(context)
    }
}
