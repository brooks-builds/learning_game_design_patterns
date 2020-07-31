mod grid;
mod unit;
mod unit_moving;

use ggez::event::EventHandler;
use ggez::graphics::BLACK;
use ggez::{graphics, Context, GameResult};
use grid::Grid;
use rand::prelude::*;

pub struct GameState {
    grid: Grid,
    rng: ThreadRng,
}

impl GameState {
    pub fn new(context: &mut Context) -> GameResult<GameState> {
        let mut grid = Grid::new(context)?;
        let rng = thread_rng();

        grid.add(255.0, 255.0, context)?;
        Ok(GameState { grid, rng })
    }
}

impl EventHandler for GameState {
    fn update(&mut self, _context: &mut Context) -> GameResult {
        self.grid.update(&mut self.rng);
        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> GameResult {
        graphics::clear(context, BLACK);

        self.grid.draw(context)?;

        graphics::present(context)
    }
}
