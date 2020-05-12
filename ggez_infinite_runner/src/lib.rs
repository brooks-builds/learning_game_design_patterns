mod player;

use ggez::event::EventHandler;
use ggez::graphics::{DrawParam, Mesh};
use ggez::{graphics, Context, GameResult};
use player::Player;

pub struct MyGame {
    player: Player,
    player_mesh: Mesh,
}

impl MyGame {
    pub fn new(context: &mut Context) -> GameResult<MyGame> {
        // Load/create resources such as images here.
        let player = Player::new(50.0, 50.0);
        let player_mesh = player.create_mesh(context)?;

        Ok(MyGame {
            player,
            player_mesh,
        })
    }
}

impl EventHandler for MyGame {
    fn update(&mut self, _context: &mut Context) -> GameResult<()> {
        // Update code here...
        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> GameResult<()> {
        graphics::clear(context, graphics::BLACK);
        // Draw code here...
        graphics::draw(
            context,
            &self.player_mesh,
            DrawParam::default().dest(self.player.get_location()),
        )?;
        graphics::present(context)
    }
}
