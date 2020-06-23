mod button;
mod commands;
mod events;
mod game_state;
mod input_handler;
mod obstacle;
mod player;
mod score;
mod tree;
mod tree_model;

use button::Button;
use commands::Commands;
use events::{Event, Observer, PossibleObserver, Subject, WrappedGameState, WrappedScore};
use game_state::GameState;
use ggez::event::winit_event::WindowEvent;
use ggez::event::{EventHandler, EventsLoop};
use ggez::graphics::{DrawParam, Font, Mesh, Scale, Text};
use ggez::input::keyboard::{KeyCode, KeyMods};
use ggez::input::mouse;
use ggez::nalgebra::{Point2, Vector2};
use ggez::{graphics, timer, Context, GameResult};
use input_handler::InputHandler;
use obstacle::Obstacle;
use player::Player;
use rand::distributions::Uniform;
use rand::prelude::*;
use score::Score;
use std::sync::{Arc, Mutex};
use tree::Tree;
use tree_model::{TreeModel, TreeType};

const MS_PER_FRAME: u32 = 1000 / 60;

pub struct MyGame {
    player: Player,
    player_mesh: Mesh,
    gravity: Vector2<f32>,
    obstacle_1: Obstacle,
    obstacle_2: Obstacle,
    obstacle_mesh: Mesh,
    increase_speed_every_seconds: u64,
    time_since_start_to_increase_speed: u64,
    wrapped_game_state: WrappedGameState,
    wrapped_score: WrappedScore,
    input_handler: InputHandler,
    rebind_jump_button: Button,
    rebind_reset_game_button: Button,
    tree_model: TreeModel,
    trees: Vec<Tree>,
    rng: ThreadRng,
    create_tree_at: u64,
    trees_to_clone: Vec<Tree>,
    trees_to_clone_distribution: Uniform<usize>,
    target_fps: std::time::Duration,
}

impl MyGame {
    pub fn new(context: &mut Context) -> GameResult<MyGame> {
        // Load/create resources such as images here.
        let rng = rand::thread_rng();
        let (arena_width, arena_height) = graphics::drawable_size(context);
        let mut player = Player::new(350.0, 50.0);
        let player_mesh = player.create_mesh(context)?;
        let gravity = Vector2::new(0.0, 0.3);
        let obstacle_size = 25.0;
        let mut obstacle_1 = Obstacle::new(
            arena_width + obstacle_size,
            arena_height - obstacle_size,
            obstacle_size,
            arena_width,
        );
        let mut obstacle_2 = Obstacle::new(
            arena_width + (arena_width / 2.0),
            arena_height - obstacle_size,
            obstacle_size,
            arena_width,
        );
        let obstacle_mesh = obstacle_1.create_mesh(context)?;
        let increase_speed_every_seconds = 5;
        let time_since_start_to_increase_speed = increase_speed_every_seconds;
        let wrapped_game_state = Arc::new(Mutex::new(GameState::Playing));
        let wrapped_score = Arc::new(Mutex::new(Score::new()));
        let input_handler = InputHandler::new();
        let rebind_jump_button = Button::new(200.0, 200.0, "Rebind Jump", context)?;
        let rebind_reset_game_button = Button::new(200.0, 400.0, "Rebind Reset Game", context)?;
        let tree_model = TreeModel::new(context)?;
        let trees = vec![];
        let create_tree_at = 0;
        let score_observer = PossibleObserver::Score(wrapped_score.clone());
        obstacle_1.add_observer(score_observer.clone());
        obstacle_2.add_observer(score_observer);

        // player.add_observer(PossibleObserver::GameState(wrapped_game_state.clone()));

        let tree = Tree::new(arena_width, arena_height, &tree_model, TreeType::Normal);
        let tall_tree = Tree::new(arena_width, arena_height, &tree_model, TreeType::Tall);
        let trees_to_clone = vec![tree, tall_tree];
        let trees_to_clone_distribution = Uniform::new_inclusive(0, trees_to_clone.len() - 1);

        Ok(MyGame {
            player,
            player_mesh,
            gravity,
            obstacle_1,
            obstacle_2,
            obstacle_mesh,
            increase_speed_every_seconds,
            time_since_start_to_increase_speed,
            wrapped_game_state,
            wrapped_score,
            input_handler,
            rebind_jump_button,
            rebind_reset_game_button,
            tree_model,
            trees,
            rng,
            create_tree_at,
            trees_to_clone,
            trees_to_clone_distribution,
            target_fps: std::time::Duration::new(1000 / 60, 0),
        })
    }

    fn draw_end_game_screen(&self, context: &mut Context) -> GameResult<()> {
        let mut game_over_text = Text::new("Game Over");
        let mut score_text;

        {
            let wrapped_score = self.wrapped_score.clone();

            score_text = Text::new(format!(
                "You jumped over {} obstacles",
                wrapped_score.lock().unwrap().get()
            ));
        }

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
        let (restart_width, _restart_height) = restart_text.dimensions(context);

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
        let mut score_text;
        {
            let wrapped_score = self.wrapped_score.clone();
            score_text = Text::new(format!("Score: {}", wrapped_score.lock().unwrap().get()));
        }
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
        let (arena_width, _arena_height) = graphics::drawable_size(context);
        let mut title = Text::new("Commands");
        let mut jump = Text::new(format!("Jump - {}", self.input_handler.get_jump_keycode()));
        let mut restart = Text::new(format!(
            "Restart Game - {}",
            self.input_handler.get_reset_game_keycode()
        ));

        title.set_font(Font::default(), Scale::uniform(100.0));
        jump.set_font(Font::default(), Scale::uniform(50.0));
        restart.set_font(Font::default(), Scale::uniform(50.0));

        let (title_width, _title_height) = title.dimensions(context);
        let (jump_width, _jump_height) = jump.dimensions(context);
        let (restart_width, _restart_height) = restart.dimensions(context);

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

        let index = self.rng.sample(&self.trees_to_clone_distribution);
        let tree = self.trees_to_clone[index].clone();

        match tree.tree_type {
            TreeType::Normal => self.trees.insert(0, tree),
            TreeType::Tall => self.trees.push(tree),
        }

        self.create_tree_at = current_time + self.rng.gen_range(1, 15);

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

    fn reset_game(&mut self) {
        self.player.reset();
        self.wrapped_score.lock().unwrap().reset();
        self.obstacle_1.reset_to_start();
        self.obstacle_2.reset_to_start();
        self.wrapped_game_state.lock().unwrap().playing();
    }
}

impl EventHandler for MyGame {
    fn update(&mut self, context: &mut Context) -> GameResult<()> {
        let game_state;

        {
            game_state = self.wrapped_game_state.lock().unwrap().clone();
        }

        match game_state {
            GameState::Playing => {
                let (_arena_width, arena_height) = graphics::drawable_size(context);
                self.player.apply_force(self.gravity);
                let command = self.input_handler.handle_input(context);
                self.player.run(&command, arena_height);
                self.player.hit_ground(arena_height);
                self.obstacle_1.run(&self.player);
                if self.obstacle_1.is_offscreen() {
                    self.obstacle_1.reset_location();
                }
                self.obstacle_2.run(&self.player);
                if self.obstacle_2.is_offscreen() {
                    self.obstacle_2.reset_location();
                }
                let time_since_start = timer::time_since_start(context).as_secs();
                if time_since_start >= self.time_since_start_to_increase_speed {
                    self.obstacle_1.increase_speed();
                    self.obstacle_2.increase_speed();
                    self.time_since_start_to_increase_speed =
                        time_since_start + self.increase_speed_every_seconds;
                }

                self.player.handle_running_into_obstacle(&self.obstacle_1);
                self.player.handle_running_into_obstacle(&self.obstacle_2);

                self.update_trees();
                self.create_tree(context)?;
                self.destroy_trees_offscreen();
            }
            GameState::GameOver => {
                if let Commands::ResetGame = self.input_handler.handle_input(context) {
                    self.reset_game();
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
        let game_state;

        {
            game_state = self.wrapped_game_state.lock().unwrap().clone();
        }

        match game_state {
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

        graphics::present(context)?;

        Ok(())
    }

    fn key_down_event(
        &mut self,
        _context: &mut Context,
        keycode: KeyCode,
        _keymods: KeyMods,
        _repeat: bool,
    ) {
        if let KeyCode::Escape = keycode {
            match *self.wrapped_game_state.lock().unwrap() {
                GameState::Playing => *self.wrapped_game_state.lock().unwrap() = GameState::Help,
                GameState::GameOver => *self.wrapped_game_state.lock().unwrap() = GameState::Help,
                GameState::Help => *self.wrapped_game_state.lock().unwrap() = GameState::Playing,
            }
        } else if self.input_handler.is_rebinding() {
            self.input_handler.bind_key(keycode);
        }
    }
}

pub fn run<S>(ctx: &mut Context, events_loop: &mut EventsLoop, state: &mut S) -> GameResult
where
    S: EventHandler,
{
    use ggez::input::keyboard;

    while ctx.continuing {
        // If you are writing your own event loop, make sure
        // you include `timer_context.tick()` and
        // `ctx.process_event()` calls.  These update ggez's
        // internal state however necessary.
        ctx.timer_context.tick();
        events_loop.poll_events(|event| {
            ctx.process_event(&event);
            match event {
                ggez::event::winit_event::Event::WindowEvent { event, .. } => match event {
                    WindowEvent::Resized(logical_size) => {
                        // let actual_size = logical_size;
                        state.resize_event(
                            ctx,
                            logical_size.width as f32,
                            logical_size.height as f32,
                        );
                    }
                    WindowEvent::CloseRequested => {
                        if !state.quit_event(ctx) {
                            ggez::event::quit(ctx);
                        }
                    }
                    WindowEvent::Focused(gained) => {
                        state.focus_event(ctx, gained);
                    }
                    WindowEvent::ReceivedCharacter(ch) => {
                        state.text_input_event(ctx, ch);
                    }
                    WindowEvent::KeyboardInput {
                        input:
                            ggez::event::winit_event::KeyboardInput {
                                state: ggez::event::winit_event::ElementState::Pressed,
                                virtual_keycode: Some(keycode),
                                modifiers,
                                ..
                            },
                        ..
                    } => {
                        let repeat = keyboard::is_key_repeated(ctx);
                        state.key_down_event(ctx, keycode, modifiers.into(), repeat);
                    }
                    WindowEvent::KeyboardInput {
                        input:
                            ggez::event::winit_event::KeyboardInput {
                                state: ggez::event::winit_event::ElementState::Released,
                                virtual_keycode: Some(keycode),
                                modifiers,
                                ..
                            },
                        ..
                    } => {
                        state.key_up_event(ctx, keycode, modifiers.into());
                    }
                    WindowEvent::MouseWheel { delta, .. } => {
                        let (x, y) = match delta {
                            ggez::event::winit_event::MouseScrollDelta::LineDelta(x, y) => (x, y),
                            ggez::event::winit_event::MouseScrollDelta::PixelDelta(
                                winit::dpi::LogicalPosition { x, y },
                            ) => (x as f32, y as f32),
                        };
                        state.mouse_wheel_event(ctx, x, y);
                    }
                    WindowEvent::MouseInput {
                        state: element_state,
                        button,
                        ..
                    } => {
                        let position = mouse::position(ctx);
                        match element_state {
                            ggez::event::winit_event::ElementState::Pressed => {
                                state.mouse_button_down_event(ctx, button, position.x, position.y)
                            }
                            ggez::event::winit_event::ElementState::Released => {
                                state.mouse_button_up_event(ctx, button, position.x, position.y)
                            }
                        }
                    }
                    WindowEvent::CursorMoved { .. } => {
                        let position = mouse::position(ctx);
                        let delta = mouse::delta(ctx);
                        state.mouse_motion_event(ctx, position.x, position.y, delta.x, delta.y);
                    }
                    _x => {
                        // trace!("ignoring window event {:?}", x);
                    }
                },
                ggez::event::winit_event::Event::DeviceEvent { event, .. } => match event {
                    _ => (),
                },
                ggez::event::winit_event::Event::Awakened => (),
                ggez::event::winit_event::Event::Suspended(_) => (),
            }
        });
        state.update(ctx)?;
        state.draw(ctx)?;
    }

    Ok(())
}
