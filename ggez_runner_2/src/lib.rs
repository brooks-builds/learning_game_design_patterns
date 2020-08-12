mod camera;
mod errors;
pub mod game_data;
pub mod game_object;
mod grid;
mod meshes;
mod physics;
mod states;
mod types;

use camera::Camera;
pub use errors::CustomError;
use game_data::GameData;
use game_object::GameObject;
use ggez::event::EventHandler;
use ggez::graphics::Color;
use ggez::{graphics, timer, Context, GameResult};
use grid::Grid;
use meshes::Meshes;
use physics::{PlayerPhysics, StaticPhysics};
pub use states::States;
use std::sync::mpsc::{channel, Receiver, Sender};
pub use types::Types;

pub struct GameState {
    background_color: Color,
    grid: Grid,
    meshes: Meshes,
    camera: Camera,
    state: States,
    won_event_receive: Receiver<()>,
}

impl GameState {
    pub fn new(game_data: GameData, context: &mut Context) -> GameResult<GameState> {
        let (player_moved_event_send, player_moved_event_receive) = channel::<f32>();
        let (won_event_send, won_event_receive) = channel::<()>();
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
        )?;
        let meshes = Meshes::new(context, &game_data)?;

        Self::populate_level(
            &mut grid,
            &game_data,
            &mut next_object_id,
            player_moved_event_send,
            won_event_send,
        );

        Ok(GameState {
            background_color: Color::from_rgb(0, 51, 102),
            grid,
            meshes,
            camera,
            state: States::Playing,
            won_event_receive,
        })
    }

    fn populate_level(
        grid: &mut Grid,
        game_data: &GameData,
        next_object_id: &mut u64,
        player_moved_event_send: Sender<f32>,
        won_event_send: Sender<()>,
    ) {
        for (index, level_type) in game_data.level.iter().enumerate() {
            match level_type {
                Types::Floor => {
                    Self::create_floor_object(next_object_id, game_data, index, grid);
                }
                Types::Start => {
                    Self::create_floor_object(next_object_id, game_data, index, grid);

                    let start = GameObject::new(
                        *next_object_id,
                        5.0,
                        game_data.cell_size,
                        game_data.cell_size * index as f32,
                        game_data.floor_y - game_data.cell_size,
                        Types::Start,
                        Some(Box::new(StaticPhysics)),
                    );

                    *next_object_id += 1;
                    grid.add(start);
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
                        ))),
                    );
                    grid.add(player);
                    *next_object_id += 1;
                }
                Types::SpikeUp => {
                    Self::create_floor_object(next_object_id, game_data, index, grid);

                    let spike = GameObject::new(
                        *next_object_id,
                        game_data.cell_size,
                        game_data.cell_size,
                        game_data.cell_size * index as f32,
                        game_data.floor_y - game_data.cell_size,
                        Types::SpikeUp,
                        Some(Box::new(StaticPhysics)),
                    );

                    grid.add(spike);
                    *next_object_id += 1;
                }
                Types::End => {
                    Self::create_floor_object(next_object_id, game_data, index, grid);

                    let end = GameObject::new(
                        *next_object_id,
                        game_data.end_width,
                        game_data.cell_size,
                        game_data.cell_size * index as f32,
                        game_data.floor_y - game_data.cell_size,
                        Types::End,
                        Some(Box::new(StaticPhysics)),
                    );
                    grid.add(end);
                    *next_object_id += 1;
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
    ) {
        let floor = GameObject::new(
            *next_object_id,
            game_data.cell_size,
            game_data.cell_size,
            game_data.cell_size * offset as f32,
            game_data.floor_y,
            Types::Floor,
            Some(Box::new(StaticPhysics)),
        );
        *next_object_id += 1;
        grid.add(floor);
    }
}

impl EventHandler for GameState {
    fn update(&mut self, context: &mut Context) -> GameResult {
        while timer::check_update_time(context, 60) {
            if let States::Playing = self.state {
                self.grid.update();
                self.camera.update();

                if let Ok(_) = self.won_event_receive.try_recv() {
                    self.state = States::Won;
                }
            }
        }
        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> GameResult {
        graphics::clear(context, self.background_color);

        if let Err(error) = self.camera.draw(&self.grid, &self.meshes, context) {
            match error {
                CustomError::GgezGameError(error) => return Err(error),
                _ => panic!("unknown draw error"),
            }
        }

        graphics::present(context)
    }
}
