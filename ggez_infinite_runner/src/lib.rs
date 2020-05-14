mod player;

use ggez::event::EventHandler;
use ggez::graphics::{DrawParam, Mesh};
use ggez::input::keyboard;
use ggez::input::keyboard::KeyCode;
use ggez::nalgebra::Vector2;
use ggez::{graphics, Context, GameResult};
use player::Player;

pub struct MyGame {
    player: Player,
    player_mesh: Mesh,
    gravity: Vector2<f32>,
}

impl MyGame {
    pub fn new(context: &mut Context) -> GameResult<MyGame> {
        // Load/create resources such as images here.
        let player = Player::new(350.0, 50.0);
        let player_mesh = player.create_mesh(context)?;
        let gravity = Vector2::new(0.0, 0.0001);

        Ok(MyGame {
            player,
            player_mesh,
            gravity,
        })
    }
}

impl EventHandler for MyGame {
    fn update(&mut self, context: &mut Context) -> GameResult<()> {
        // Update code here...
        let (_arena_width, arena_height) = graphics::drawable_size(context);
        self.player.apply_force(&self.gravity);
        self.player.run();
        self.player.hit_ground(arena_height);

        if keyboard::is_key_pressed(context, KeyCode::Space) {
            self.player.jump();
        }

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
