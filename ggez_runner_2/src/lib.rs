mod camera;
mod errors;
pub mod game_data;
pub mod game_object;
mod grid;
mod interface;
mod meshes;
mod physics;
mod states;
mod types;

use camera::Camera;
pub use errors::CustomError;
use game_data::GameData;
use game_object::GameObject;
use ggez::event::EventHandler;
use ggez::graphics::{Color, DrawParam, Font, Scale, Text};
use ggez::input::keyboard::{KeyCode, KeyMods};
use ggez::nalgebra::{Point2, Vector2};
use ggez::{graphics, timer, Context, GameResult};
use grid::Grid;
use interface::Interface;
use meshes::Meshes;
use physics::PlayerPhysics;
pub use states::States;
use std::collections::HashMap;
use std::sync::mpsc::{channel, Receiver, Sender};
pub use types::Types;

pub struct GameState {
    background_color: Color,
    grid: Grid,
    meshes: Meshes,
    camera: Camera,
    state: States,
    won_event_receive: Receiver<()>,
    game_data: GameData,
    player_moved_event_send: Sender<f32>,
    died_event_receive: Receiver<()>,
    game_objects: HashMap<u64, GameObject>,
    gravity_force: Vector2<f32>,
    game_object_off_grid_event: Receiver<u64>,
    interface: Interface,
}

impl GameState {
    pub fn new(game_data: GameData, context: &mut Context) -> GameResult<GameState> {
        let (player_moved_event_send, player_moved_event_receive) = channel::<f32>();
        let (won_event_send, won_event_receive) = channel::<()>();
        let (died_event_send, died_event_receive) = channel::<()>();
        let (game_object_off_grid_event_send, game_object_off_grid_event_receive) =
            channel::<u64>();
        let camera = Camera::new(
            game_data.player.start_x - game_data.camera_chase_x,
            0.0,
            game_data.camera_width + game_data.cell_size,
            game_data.camera_height,
            player_moved_event_receive,
        );
        let mut next_object_id = 0;
        let mut grid = Grid::new(
            game_data.cell_size,
            game_data.cell_size,
            game_data.world_height,
            game_data.level.len(),
            game_object_off_grid_event_send,
        )?;
        let meshes = Meshes::new(context, &game_data)?;
        let mut game_objects = HashMap::new();
        let interface = Interface::new();

        Self::populate_level(
            &mut grid,
            &game_data,
            &mut next_object_id,
            player_moved_event_send.clone(),
            won_event_send,
            died_event_send,
            &mut game_objects,
        );

        let gravity_force = Vector2::new(0.0, game_data.gravity_force);

        Ok(GameState {
            background_color: Color::from_rgb(0, 51, 102),
            grid,
            meshes,
            camera,
            state: States::NotStarted,
            won_event_receive,
            game_data,
            player_moved_event_send,
            died_event_receive,
            game_objects,
            gravity_force,
            game_object_off_grid_event: game_object_off_grid_event_receive,
            interface,
        })
    }

    fn populate_level(
        grid: &mut Grid,
        game_data: &GameData,
        next_object_id: &mut u64,
        player_moved_event_send: Sender<f32>,
        won_event_send: Sender<()>,
        died_event_send: Sender<()>,
        game_objects: &mut HashMap<u64, GameObject>,
    ) {
        for (index, level_type) in game_data.level.iter().enumerate() {
            match level_type {
                Types::Floor => {
                    Self::create_floor_object(next_object_id, game_data, index, grid, game_objects);
                }
                Types::Start => {
                    Self::create_floor_object(next_object_id, game_data, index, grid, game_objects);

                    let start = GameObject::new(
                        *next_object_id,
                        5.0,
                        game_data.cell_size,
                        game_data.cell_size * index as f32,
                        game_data.floor_y - game_data.cell_size,
                        Types::Start,
                        None,
                    );

                    *next_object_id += 1;
                    grid.add(&start);
                    game_objects.insert(start.id, start);
                    let player = GameObject::new(
                        *next_object_id,
                        game_data.player.body_width,
                        game_data.player.body_height,
                        game_data.player.start_x,
                        game_data.player.start_y,
                        Types::Player,
                        Some(Box::new(PlayerPhysics::new(
                            game_data.player.speed,
                            player_moved_event_send.clone(),
                            won_event_send.clone(),
                            died_event_send.clone(),
                        ))),
                    );
                    grid.add(&player);
                    *next_object_id += 1;
                    game_objects.insert(player.id, player);
                }
                Types::SpikeUp => {
                    Self::create_floor_object(next_object_id, game_data, index, grid, game_objects);

                    let spike = GameObject::new(
                        *next_object_id,
                        game_data.cell_size,
                        game_data.cell_size,
                        game_data.cell_size * index as f32,
                        game_data.floor_y - game_data.cell_size,
                        Types::SpikeUp,
                        None,
                    );

                    grid.add(&spike);
                    *next_object_id += 1;
                    game_objects.insert(spike.id, spike);
                }
                Types::End => {
                    Self::create_floor_object(next_object_id, game_data, index, grid, game_objects);

                    let end = GameObject::new(
                        *next_object_id,
                        game_data.end_width,
                        game_data.cell_size,
                        game_data.cell_size * index as f32,
                        game_data.floor_y - game_data.cell_size,
                        Types::End,
                        None,
                    );
                    grid.add(&end);
                    *next_object_id += 1;
                    game_objects.insert(end.id, end);
                }
                _ => (),
            }
        }
    }

    fn create_floor_object(
        next_object_id: &mut u64,
        game_data: &GameData,
        offset: usize,
        grid: &mut Grid,
        game_objects: &mut HashMap<u64, GameObject>,
    ) {
        let floor = GameObject::new(
            *next_object_id,
            game_data.cell_size,
            game_data.cell_size,
            game_data.cell_size * offset as f32,
            game_data.floor_y,
            Types::Floor,
            None,
        );
        *next_object_id += 1;
        grid.add(&floor);
        game_objects.insert(floor.id, floor);
    }

    fn reset_game(&mut self) {
        if let Some((player_id, player)) = self
            .game_objects
            .iter_mut()
            .find(|(game_object_id, game_object)| game_object.my_type == Types::Player)
        {
            let old_player_location_x = player.location.x;
            self.grid.remove(player);
            player.reset(
                self.game_data.player.start_x,
                self.game_data.player.start_y,
                self.game_data.player.speed,
            );
            if let Err(error) = self
                .player_moved_event_send
                .send(player.location.x - old_player_location_x)
            {
                println!("error resetting camera location: {}", error);
            }
            self.grid.add(player);
            self.state = States::NotStarted;
        }
    }

    fn handle_collisions(&mut self) {
        let game_objects_clone = self.game_objects.clone();

        if let Some((player_id, player)) = self
            .game_objects
            .iter_mut()
            .find(|(game_object_id, game_object)| game_object.my_type == Types::Player)
        {
            let nearby_game_objects = self.grid.query(
                player.location.x,
                player.location.y,
                player.location.x + self.game_data.cell_size * 2.0,
                player.location.y + self.game_data.cell_size * 2.0,
                &game_objects_clone,
            );
            player.handle_collisions(nearby_game_objects);
        }
    }

    fn handle_events(&mut self) {
        if let Ok(_) = self.won_event_receive.try_recv() {
            self.state = States::Won;
        }

        if let Ok(_) = self.died_event_receive.try_recv() {
            self.state = States::Died;
        }

        if let Ok(game_object_id) = self.game_object_off_grid_event.try_recv() {
            if let Some(game_object) = self.game_objects.get(&game_object_id) {
                if game_object.my_type == Types::Player {
                    self.state = States::Died;
                }
            }
        }
    }
}

impl EventHandler for GameState {
    fn update(&mut self, context: &mut Context) -> GameResult {
        while timer::check_update_time(context, 60) {
            match self.state {
                States::Playing => {
                    self.grid.update(&mut self.game_objects, self.gravity_force);
                    self.handle_collisions();
                    self.handle_events();
                }
                _ => (),
            }
        }
        self.camera.update();
        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> GameResult {
        graphics::clear(context, self.background_color);

        self.interface.draw(&self.state, context)?;

        if let Err(error) = self
            .camera
            .draw(&self.grid, &self.meshes, context, &self.game_objects)
        {
            match error {
                CustomError::GgezGameError(error) => return Err(error),
                _ => panic!("unknown draw error"),
            }
        }

        // if let Err(error) = self.grid._draw(context, &self.meshes) {
        //     if let CustomError::GgezGameError(error) = error {
        //         return Err(error);
        //     } else {
        //         println!("Error drawing the grid: {:?}", error);
        //     }
        // }

        graphics::present(context)
    }

    fn key_down_event(
        &mut self,
        _context: &mut Context,
        keycode: KeyCode,
        _keymods: KeyMods,
        _repeat: bool,
    ) {
        match self.state {
            States::NotStarted => {
                if keycode == KeyCode::Return {
                    self.state = States::Playing;
                }
            }
            States::Won => {
                if keycode == KeyCode::Return {
                    self.reset_game();
                }
            }
            States::Died => {
                if keycode == KeyCode::Return {
                    self.reset_game();
                }
            }
            _ => (),
        }
    }
}
