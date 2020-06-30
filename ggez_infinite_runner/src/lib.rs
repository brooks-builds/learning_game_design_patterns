pub mod game_loop;

use ggez::event::EventHandler;
use ggez::{graphics, timer, Context, GameResult};

pub struct MyGame {}

impl MyGame {
    pub fn new(_context: &mut Context) -> GameResult<MyGame> {
        Ok(MyGame {})
    }
}

impl EventHandler for MyGame {
    fn update(&mut self, context: &mut Context) -> GameResult {
        if timer::check_update_time(context, 50) {}
        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> GameResult {
        graphics::clear(context, graphics::BLACK);

        let remaining_update_time = timer::remaining_update_time(context);

        graphics::present(context)
    }
}
