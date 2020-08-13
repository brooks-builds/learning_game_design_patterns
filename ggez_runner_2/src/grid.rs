use super::{CustomError, GameObject, Meshes, Types};
use ggez::{Context, GameResult};
use std::collections::HashMap;

#[derive(Debug)]
pub struct Grid {
    cell_width: f32,
    cell_height: f32,
    cells: Vec<Vec<HashMap<u64, GameObject>>>,
    world_width: usize,
    world_height: u8,
    game_objects: HashMap<u64, GameObject>,
}

impl Grid {
    pub fn new(
        cell_width: f32,
        cell_height: f32,
        world_height: u8,
        world_width: usize,
    ) -> GameResult<Grid> {
        let mut cells = vec![];

        for _y_count in 0..world_height {
            let mut row = vec![];
            for _x_count in 0..world_width {
                row.push(HashMap::new());
            }
            cells.push(row);
        }

        Ok(Grid {
            cell_width,
            cell_height,
            cells,
            world_width,
            world_height,
            game_objects: HashMap::new(),
        })
    }

    pub fn _draw(&self, context: &mut Context, meshes: &Meshes) -> Result<(), CustomError> {
        for (_y_index, row) in self.cells.iter().enumerate() {
            for (_x_index, cell) in row.iter().enumerate() {
                if let Err(error) = ggez::graphics::draw(
                    context,
                    &meshes.cell,
                    ggez::graphics::DrawParam::new().dest(ggez::nalgebra::Point2::new(
                        _x_index as f32 * self.cell_width,
                        _y_index as f32 * self.cell_height,
                    )),
                ) {
                    return Err(CustomError::GgezGameError(error));
                }
                for (_id, game_object) in cell {
                    game_object.draw(meshes, context)?;
                }
            }
        }
        Ok(())
    }

    pub fn add(&mut self, game_object: GameObject) {
        let x_index = (game_object.location.x / self.cell_width) as usize;
        let y_index = (game_object.location.y / self.cell_height) as usize;

        self.cells[y_index][x_index].insert(game_object.id, game_object);
    }

    pub fn query(&self, start_x: f32, start_y: f32, end_x: f32, end_y: f32) -> Vec<&GameObject> {
        let mut game_objects = vec![];

        let index_start_x = (start_x / self.cell_width) as usize;
        let index_start_y = (start_y / self.cell_height) as usize;
        let index_end_x = (end_x / self.cell_width) as usize;
        let index_end_y = (end_y / self.cell_height) as usize;

        for y_index in index_start_y..index_end_y {
            for x_index in index_start_x..index_end_x {
                if y_index >= self.cells.len() || x_index >= self.cells[0].len() {
                    continue;
                }
                for (_game_object_id, game_object) in self.cells[y_index][x_index].iter() {
                    game_objects.push(game_object);
                }
            }
        }

        game_objects
    }

    pub fn update(&mut self) {
        self.move_game_objects();
        self.handle_player_collisions();
    }

    fn move_game_objects(&mut self) {
        let mut game_object_moves = vec![];
        for (y_index, row) in self.cells.iter_mut().enumerate() {
            for (x_index, cell) in row.iter_mut().enumerate() {
                let cell_clone = cell.clone();
                for (_id, game_object) in cell.iter_mut() {
                    let previous_index_x = (game_object.location.x / self.cell_width) as usize;
                    let previous_index_y = (game_object.location.y / self.cell_height) as usize;
                    game_object.update();
                    let next_index_x = (game_object.location.x / self.cell_width) as usize;
                    let next_index_y = (game_object.location.y / self.cell_height) as usize;

                    if previous_index_x == next_index_x && previous_index_y == next_index_y {
                        continue;
                    }

                    // let other_game_objects = cell_clone
                    //     .iter()
                    //     .filter_map(|(other_game_object_id, other_game_object)| {
                    //         if other_game_object_id == &game_object.id {
                    //             None
                    //         } else {
                    //             Some(other_game_object)
                    //         }
                    //     })
                    //     .collect::<Vec<_>>();
                    // game_object.handle_collisions(other_game_objects);
                    game_object_moves.push(GridMove {
                        previous_index_x,
                        previous_index_y,
                        next_index_x,
                        next_index_y,
                        game_object_id: game_object.id,
                    });
                }
            }
        }

        for grid_move in game_object_moves {
            if let Some(game_object) = self.cells[grid_move.previous_index_y]
                [grid_move.previous_index_x]
                .remove(&grid_move.game_object_id)
            {
                self.cells[grid_move.next_index_y][grid_move.next_index_x]
                    .insert(grid_move.game_object_id, game_object);
            }
        }
    }

    fn handle_player_collisions(&mut self) {
        let player = self.get_first_by_type(Types::Player);
        if let Some(player) = player {
            let player_grid_x = (player.location.x / self.cell_width) as usize;
            let player_grid_y = (player.location.y / self.cell_height) as usize;
        }
    }

    pub fn remove_first_by_type(&mut self, game_object_type: Types) -> Option<GameObject> {
        let mut game_object_to_remove = None;

        for (y_index, row) in self.cells.iter_mut().enumerate() {
            for (x_index, cell) in row.iter_mut().enumerate() {
                for (game_object_id, game_object) in cell.iter_mut() {
                    if game_object_type == game_object.my_type {
                        game_object_to_remove = Some(GameObjectLocation {
                            x: x_index,
                            y: y_index,
                            id: game_object.id,
                        });
                        break;
                    }
                }
            }
        }

        if let Some(game_object_to_remove) = game_object_to_remove {
            self.cells[game_object_to_remove.y][game_object_to_remove.x]
                .remove(&game_object_to_remove.id)
        } else {
            None
        }
    }

    fn get_first_by_type(&mut self, object_type: Types) -> Option<&mut GameObject> {
        for row in self.cells.iter_mut() {
            for cell in row.iter_mut() {
                for game_object in cell.values_mut() {
                    if object_type == game_object.my_type {
                        return Some(game_object);
                    }
                }
            }
        }
        None
    }
}

struct GameObjectLocation {
    x: usize,
    y: usize,
    id: u64,
}

#[derive(Debug)]
struct GridMove {
    previous_index_x: usize,
    previous_index_y: usize,
    next_index_x: usize,
    next_index_y: usize,
    game_object_id: u64,
}
