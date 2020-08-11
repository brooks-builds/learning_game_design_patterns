mod errors;
pub mod game_data;
pub mod game_object;
mod grid;
mod states;
mod types;

pub use errors::CustomError;
use game_data::GameData;
use ggez::event::EventHandler;
use ggez::graphics::Color;
use ggez::{graphics, Context, GameResult};
use grid::Grid;
pub use states::States;
pub use types::Types;

pub struct GameState {
    background_color: Color,
    grid: Grid,
}

impl GameState {
    pub fn new(game_data: GameData, context: &mut Context) -> GameResult<GameState> {
        let grid = Grid::new(
            game_data.cell_size,
            game_data.cell_size,
            context,
            game_data.world_height,
        )?;

        Ok(GameState {
            background_color: Color::from_rgb(0, 51, 102),
            grid,
        })
    }
}

impl EventHandler for GameState {
    fn update(&mut self, _context: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> GameResult {
        graphics::clear(context, self.background_color);

        self.grid.draw(context)?;

        graphics::present(context)
    }
}
