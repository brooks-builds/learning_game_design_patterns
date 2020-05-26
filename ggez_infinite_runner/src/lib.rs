mod button;
mod command_trait;
mod game_state;
mod input_handler;
mod jump_command;
mod obstacle;
mod player;
mod reset_game_command;
mod score;
mod tree;
mod tree_model;

use button::Button;
use command_trait::ActorCommand;
use command_trait::GameCommand;
use game_state::GameState;
use ggez::event::EventHandler;
use ggez::graphics::{DrawParam, Font, Mesh, Scale, Text};
use ggez::input::keyboard::{KeyCode, KeyMods};
use ggez::input::mouse;
use ggez::nalgebra::{Point2, Vector2};
use ggez::{graphics, timer, Context, GameResult};
use input_handler::InputHandler;
use jump_command::JumpCommand;
use obstacle::Obstacle;
use player::Player;
use rand::prelude::*;
use reset_game_command::ResetGameCommand;
use score::Score;
use tree::Tree;
use tree_model::TreeModel;

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
    rebind_jump_button: Button,
    rebind_reset_game_button: Button,
    tree_model: TreeModel,
    trees: Vec<Tree>,
    rng: ThreadRng,
    create_tree_at: u64,
}

impl MyGame {
    pub fn new(context: &mut Context) -> GameResult<MyGame> {
        // Load/create resources such as images here.
        let mut rng = rand::thread_rng();
        let (arena_width, arena_height) = graphics::drawable_size(context);
        let player = Player::new(350.0, 50.0);
        let player_mesh = player.create_mesh(context)?;
        let gravity = Vector2::new(0.0, 0.5);
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
        let rebind_jump_button = Button::new(200.0, 200.0, "Rebind Jump", context)?;
        let rebind_reset_game_button = Button::new(200.0, 400.0, "Rebind Reset Game", context)?;
        let tree_model = TreeModel::new(context)?;
        let trees = vec![];
        let create_tree_at = 0;

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
            rebind_jump_button,
            rebind_reset_game_button,
            tree_model,
            trees,
            rng,
            create_tree_at,
        })
    }

    fn draw_end_game_screen(&self, context: &mut Context) -> GameResult<()> {
        let mut game_over_text = Text::new("Game Over");
        let mut score_text = Text::new(format!("You jumped over {} obstacles", self.score.get()));
        let mut restart_text = Text::new(format!(
            "Restart game by pressing {}",
            self.input_handler.get_reset_game_keycode()
        ));
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
        let mut jump = Text::new(format!("Jump - {}", self.input_handler.get_jump_keycode()));
        let mut restart = Text::new(format!(
            "Restart Game - {}",
            self.input_handler.get_reset_game_keycode()
        ));

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

        self.rebind_jump_button.draw(context)?;
        self.rebind_reset_game_button.draw(context)?;

        Ok(())
    }

    fn draw_trees(&mut self, context: &mut Context) -> GameResult<()> {
        for tree in &self.trees {
            tree.draw(context, &self.tree_model, &mut self.rng)?
        }

        Ok(())
    }

    fn update_trees(&mut self) {
        for tree in &mut self.trees {
            tree.update(&self.tree_model);
        }
    }

    fn create_tree(&mut self, context: &mut Context) -> GameResult<()> {
        let current_time = timer::time_since_start(context).as_secs();
        if current_time < self.create_tree_at {
            return Ok(());
        }

        let (arena_width, arena_height) = graphics::drawable_size(context);

        self.trees.push(Tree::new(
            arena_width,
            arena_height,
            &self.tree_model,
            &mut self.rng,
        ));

        self.create_tree_at = current_time + self.rng.gen_range(1, 30);

        Ok(())
    }

    fn destroy_trees_offscreen(&mut self) {
        let trees: Vec<Tree> = self
            .trees
            .clone()
            .into_iter()
            .filter(|tree| !tree.is_off_screen(&self.tree_model))
            .collect();
        self.trees = trees;
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
                self.update_trees();
                self.create_tree(context)?;
                self.destroy_trees_offscreen();
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
                    if self.input_handler.not_binding() {
                        if self
                            .rebind_jump_button
                            .is_being_clicked(mouse_position.into())
                        {
                            self.input_handler.start_binding_jump();
                        } else if self
                            .rebind_reset_game_button
                            .is_being_clicked(mouse_position.into())
                        {
                            self.input_handler.start_binding_reset_game();
                        }
                    }
                }
            }
        }

        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> GameResult<()> {
        graphics::clear(context, graphics::BLACK);
        // Draw code here...

        match self.game_state {
            GameState::Playing => {
                self.draw_trees(context)?;
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
        } else if self.input_handler.is_rebinding() {
            self.input_handler.bind_key(keycode);
        }
    }
}
