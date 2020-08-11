use ggez::event::EventHandler;
use ggez::graphics::{Color, DrawMode, DrawParam, MeshBuilder};
use ggez::nalgebra::Point2;
use ggez::{graphics, Context, GameResult};

pub struct GameState {
    background_color: Color,
}

impl GameState {
    pub fn new() -> GameState {
        GameState {
            background_color: Color::from_rgb(0, 51, 102),
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
