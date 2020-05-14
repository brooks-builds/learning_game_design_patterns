mod obstacle;
mod player;

use ggez::event::EventHandler;
use ggez::graphics::{DrawParam, Mesh};
use ggez::input::keyboard;
use ggez::input::keyboard::KeyCode;
use ggez::nalgebra::Vector2;
use ggez::{graphics, timer, Context, GameResult};
use obstacle::Obstacle;
use player::Player;

pub struct MyGame {
    player: Player,
    player_mesh: Mesh,
    gravity: Vector2<f32>,
    obstacle_1: Obstacle,
    obstacle_2: Obstacle,
    obstacle_mesh: Mesh,
    increase_speed_every_seconds: u64,
    time_since_start_to_increase_speed: u64,
}

impl MyGame {
    pub fn new(context: &mut Context) -> GameResult<MyGame> {
        // Load/create resources such as images here.
        let (arena_width, arena_height) = graphics::drawable_size(context);
        let player = Player::new(350.0, 50.0);
        let player_mesh = player.create_mesh(context)?;
        let gravity = Vector2::new(0.0, 0.0001);
        let obstacle_size = 25.0;
        let obstacle_1 = Obstacle::new(
            arena_width + obstacle_size,
            arena_height - obstacle_size,
            obstacle_size,
            arena_width,
        );
        let obstacle_2 = Obstacle::new(
            arena_width + (arena_width / 2.0),
            arena_height - obstacle_size,
            obstacle_size,
            arena_width,
        );
        let obstacle_mesh = obstacle_1.create_mesh(context)?;
        let increase_speed_every_seconds = 5;
        let time_since_start_to_increase_speed = increase_speed_every_seconds;

        Ok(MyGame {
            player,
            player_mesh,
            gravity,
            obstacle_1,
            obstacle_2,
            obstacle_mesh,
            increase_speed_every_seconds,
            time_since_start_to_increase_speed,
        })
    }
}

impl EventHandler for MyGame {
    fn update(&mut self, context: &mut Context) -> GameResult<()> {
        // Update code here...
        let (arena_width, arena_height) = graphics::drawable_size(context);
        self.player.apply_force(&self.gravity);
        self.player.run();
        self.player.hit_ground(arena_height);

        if keyboard::is_key_pressed(context, KeyCode::Space) {
            self.player.jump();
        }

        self.obstacle_1.run();
        if self.obstacle_1.is_offscreen(arena_width) {
            self.obstacle_1.reset_location();
        }

        self.obstacle_2.run();
        if self.obstacle_2.is_offscreen(arena_width) {
            self.obstacle_2.reset_location();
        }

        let time_since_start = timer::time_since_start(context).as_secs();
        if time_since_start >= self.time_since_start_to_increase_speed {
            self.obstacle_1.increase_speed();
            self.obstacle_2.increase_speed();
            self.time_since_start_to_increase_speed =
                time_since_start + self.increase_speed_every_seconds;
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

        graphics::draw(
            context,
            &self.obstacle_mesh,
            DrawParam::default().dest(self.obstacle_1.get_location()),
        )?;

        graphics::draw(
            context,
            &self.obstacle_mesh,
            DrawParam::default().dest(self.obstacle_2.get_location()),
        )?;
        graphics::present(context)
    }
}
