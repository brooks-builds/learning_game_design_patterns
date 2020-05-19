mod game_state;
mod obstacle;
mod player;
mod input_handler;
mod command_trait;
mod jump_command;
mod reset_game_command;
mod score;
mod button;

use game_state::GameState;
use ggez::event::EventHandler;
use ggez::graphics::{DrawParam, Font, Mesh, Scale, Text};
use ggez::input::keyboard;
use ggez::input::keyboard::{KeyCode, KeyMods};
use ggez::input::mouse;
use ggez::nalgebra::{Point2, Vector2};
use ggez::{graphics, timer, Context, GameResult};
use obstacle::Obstacle;
use player::Player;
use input_handler::InputHandler;
use jump_command::JumpCommand;
use command_trait::ActorCommand;
use command_trait::GameCommand;
use reset_game_command::ResetGameCommand;
use score::Score;
use button::Button;


pub struct MyGame {
    player: Player,
    player_mesh: Mesh,
    gravity: Vector2<f32>,
    obstacle_1: Obstacle,
    obstacle_2: Obstacle,
    obstacle_mesh: Mesh,
    increase_speed_every_seconds: u64,
    time_since_start_to_increase_speed: u64,
    game_state: GameState,
    score: Score,
    input_handler: InputHandler,
    rebind_space_button: Button,
}

impl MyGame {
    pub fn new(context: &mut Context) -> GameResult<MyGame> {
        // Load/create resources such as images here.
        let (arena_width, arena_height) = graphics::drawable_size(context);
        let player = Player::new(350.0, 50.0);
        let player_mesh = player.create_mesh(context)?;
        let gravity = Vector2::new(0.0, 0.05);
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
        let game_state = GameState::Playing;
        let score = Score::new();
        let input_handler = InputHandler::new(JumpCommand::new(), ResetGameCommand::new());
        let rebind_space_button = Button::new(200.0, 200.0, "Rebind Jump", context)?;

        Ok(MyGame {
            player,
            player_mesh,
            gravity,
            obstacle_1,
            obstacle_2,
            obstacle_mesh,
            increase_speed_every_seconds,
            time_since_start_to_increase_speed,
            game_state,
            score,
            input_handler,
            rebind_space_button,
        })
    }

    fn draw_end_game_screen(&self, context: &mut Context) -> GameResult<()> {
        let mut game_over_text = Text::new("Game Over");
        let mut score_text = Text::new(format!("You jumped over {} obstacles", self.score.get()));
        let mut restart_text = Text::new("Restart game by pressing Space");
        let (arena_width, arena_height) = graphics::drawable_size(context);

        game_over_text.set_font(Font::default(), Scale::uniform(100.0));
        score_text.set_font(Font::default(), Scale::uniform(75.0));
        restart_text.set_font(Font::default(), Scale::uniform(50.0));

        let (game_over_width, _game_over_height) = game_over_text.dimensions(context);
        let (score_width, score_height) = score_text.dimensions(context);
        let (restart_width, restart_height) = restart_text.dimensions(context);

        graphics::draw(
            context,
            &game_over_text,
            DrawParam::default().dest(Point2::new(
                (arena_width / 2.0) - (game_over_width as f32 / 2.0),
                250.0,
            )),
        )?;

        graphics::draw(
            context,
            &score_text,
            DrawParam::default().dest(Point2::new(
                (arena_width / 2.0) - (score_width as f32 / 2.0),
                (arena_height / 2.0) - (score_height as f32 / 2.0),
            )),
        )?;

        graphics::draw(
            context,
            &restart_text,
            DrawParam::default().dest(Point2::new(
                (arena_width / 2.0) - (restart_width as f32 / 2.0),
                arena_height - 250.0,
            )),
        )?;

        Ok(())
    }

    fn draw_score(&self, context: &mut Context) -> GameResult<()> {
        let mut score_text = Text::new(format!("Score: {}", self.score.get()));
        score_text.set_font(Font::default(), Scale::uniform(25.0));

        graphics::draw(
            context,
            &score_text,
            DrawParam::default().dest(Point2::new(5.0, 5.0)),
        )
    }

    fn draw_fps(&self, context: &mut Context) -> GameResult<()> {
        let fps = timer::fps(context);
        let mut fps_text = Text::new(format!("FPS: {}", fps));
        fps_text.set_font(Font::default(), Scale::uniform(25.0));

        graphics::draw(
            context,
            &fps_text,
            DrawParam::default().dest(Point2::new(5.0, 30.0)),
        )
    }

    fn draw_help_screen(&self, context: &mut Context) -> GameResult<()> {
        let (arena_width, arena_height) = graphics::drawable_size(context);
        let mut title = Text::new("Commands");
        let mut jump = Text::new("Jump - Space");
        let mut restart = Text::new("Restart Game - Space");

        title.set_font(Font::default(), Scale::uniform(100.0));
        jump.set_font(Font::default(), Scale::uniform(50.0));
        restart.set_font(Font::default(), Scale::uniform(50.0));

        let (title_width, title_height) = title.dimensions(context);
        let (jump_width, jump_height) = jump.dimensions(context);
        let (restart_width, restart_height) = restart.dimensions(context);

        graphics::draw(
            context,
            &title,
            DrawParam::default().dest(Point2::new(
                (arena_width / 2.0) - (title_width as f32 / 2.0),
                250.0,
            )),
        )?;

        graphics::draw(
            context,
            &jump,
            DrawParam::default().dest(Point2::new(
                (arena_width / 2.0) - (jump_width as f32 / 2.0),
                500.0,
            )),
        )?;

        graphics::draw(
            context,
            &restart,
            DrawParam::default().dest(Point2::new(
                (arena_width / 2.0) - (restart_width as f32 / 2.0),
                550.0,
            )),
        )?;

        self.rebind_space_button.draw(context)?;

        Ok(())
    }
}

impl EventHandler for MyGame {
    fn update(&mut self, context: &mut Context) -> GameResult<()> {
        // Update code here...
        match self.game_state {
            GameState::Playing => {
                let (arena_width, arena_height) = graphics::drawable_size(context);
                self.player.apply_force(&self.gravity);
                self.player.run();
                self.player.hit_ground(arena_height);
                if let Some(jump_command) = self.input_handler.handle_player_input(context) {
                    jump_command.execute(&mut self.player);
                }
                self.obstacle_1.run();
                if self.obstacle_1.is_offscreen(arena_width) {
                    self.obstacle_1.reset_location();
                    self.score.increment();
                }
                self.obstacle_2.run();
                if self.obstacle_2.is_offscreen(arena_width) {
                    self.obstacle_2.reset_location();
                    self.score.increment();
                }
                let time_since_start = timer::time_since_start(context).as_secs();
                if time_since_start >= self.time_since_start_to_increase_speed {
                    self.obstacle_1.increase_speed();
                    self.obstacle_2.increase_speed();
                    self.time_since_start_to_increase_speed =
                        time_since_start + self.increase_speed_every_seconds;
                }
                if self.player.is_running_into_obstacle(&self.obstacle_1)
                    || self.player.is_running_into_obstacle(&self.obstacle_2)
                {
                    self.game_state = GameState::GameOver;
                }
            }
            GameState::GameOver => {
                if let Some(command) = &mut self.input_handler.handle_game_input(context) {
                    command.execute(
                        &mut self.player,
                        &mut self.score,
                        &mut self.obstacle_1,
                        &mut self.obstacle_2,
                        &mut self.game_state,
                    );
                }        
            }
            GameState::Help => {
                if mouse::button_pressed(context, mouse::MouseButton::Left) {
                    let mouse_position = mouse::position(context);
                    if self.rebind_space_button.is_being_clicked(mouse_position.into()) {
                        println!("rebind space button clicked!");
                    }
                }
            },
        }

        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> GameResult<()> {
        graphics::clear(context, graphics::BLACK);
        // Draw code here...

        match self.game_state {
            GameState::Playing => {
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
                self.draw_score(context)?;
            }
            GameState::GameOver => self.draw_end_game_screen(context)?,
            GameState::Help => self.draw_help_screen(context)?,
        }
        self.draw_fps(context)?;
        graphics::present(context)
    }

    fn key_down_event(
        &mut self,
        context: &mut Context,
        keycode: KeyCode,
        _keymods: KeyMods,
        _repeat: bool,
    ) {
        if let KeyCode::Escape = keycode {
            match self.game_state {
                GameState::Playing => self.game_state = GameState::Help,
                GameState::GameOver => self.game_state = GameState::Help,
                GameState::Help => self.game_state = GameState::Playing,
            }
        }
    }
}
