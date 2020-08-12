mod errors;
pub mod game_data;
pub mod game_object;
mod grid;
mod meshes;
mod states;
mod types;

pub use errors::CustomError;
use game_data::GameData;
use game_object::GameObject;
use ggez::event::EventHandler;
use ggez::graphics::Color;
use ggez::{graphics, Context, GameResult};
use grid::Grid;
use meshes::Meshes;
pub use states::States;
pub use types::Types;

pub struct GameState {
    background_color: Color,
    grid: Grid,
    meshes: Meshes,
}

impl GameState {
    pub fn new(game_data: GameData, context: &mut Context) -> GameResult<GameState> {
        let mut next_object_id = 0;
        let mut grid = Grid::new(
            game_data.cell_size,
            game_data.cell_size,
            game_data.world_height,
        )?;
        let meshes = Meshes::new(context, &game_data)?;

        Self::populate_level(&mut grid, &game_data, &mut next_object_id);

        Ok(GameState {
            background_color: Color::from_rgb(0, 51, 102),
            grid,
            meshes,
        })
    }

    fn populate_level(grid: &mut Grid, game_data: &GameData, next_object_id: &mut u64) {
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
        );
        *next_object_id += 1;
        grid.add(floor);
    }
}

impl EventHandler for GameState {
    fn update(&mut self, _context: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> GameResult {
        graphics::clear(context, self.background_color);

        if let Err(error) = self.grid.draw(context, &self.meshes) {
            match error {
                CustomError::GgezGameError(error) => return Err(error),
                _ => panic!("unknown draw error"),
            }
        }

        graphics::present(context)
    }
}
