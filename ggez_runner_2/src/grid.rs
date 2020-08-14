use super::GameObject;
use ggez::nalgebra::{Point2, Vector2};
use ggez::GameResult;
use std::collections::HashMap;
use std::sync::mpsc::Sender;

#[derive(Debug)]
pub struct Grid {
    cell_width: f32,
    cell_height: f32,
    cells: Vec<Vec<Vec<u64>>>,
    world_width: usize,
    world_height: u8,
    game_object_off_grid_event: Sender<u64>,
}

impl Grid {
    pub fn new(
        cell_width: f32,
        cell_height: f32,
        world_height: u8,
        world_width: usize,
        game_object_off_grid_event: Sender<u64>,
    ) -> GameResult<Grid> {
        let mut cells = vec![];

        for _y_count in 0..world_height {
            let mut row = vec![];
            for _x_count in 0..world_width {
                row.push(vec![]);
            }
            cells.push(row);
        }

        Ok(Grid {
            cell_width,
            cell_height,
            cells,
            world_width,
            world_height,
            game_object_off_grid_event,
        })
    }

    pub fn add(&mut self, game_object: &GameObject) {
        let index = self.convert_world_location_to_grid_location(
            game_object.location.x,
            game_object.location.y,
        );

        self.cells[index.y][index.x].push(game_object.id);
    }

    pub fn query<'a>(
        &self,
        start_x: f32,
        start_y: f32,
        end_x: f32,
        end_y: f32,
        game_objects: &'a HashMap<u64, GameObject>,
    ) -> Vec<&'a GameObject> {
        let mut result = vec![];

        let start_index = self.convert_world_location_to_grid_location(start_x, start_y);
        let end_index = self.convert_world_location_to_grid_location(end_x, end_y);

        for y_index in start_index.y..end_index.y {
            for x_index in start_index.x..end_index.x {
                if y_index >= self.cells.len() || x_index >= self.cells[0].len() {
                    continue;
                }
                for id in self.cells[y_index][x_index].iter() {
                    if let Some(game_object) = game_objects.get(id) {
                        result.push(game_object);
                    }
                }
            }
        }

        result
    }

    pub fn query_mut<'a>(
        &mut self,
        start_x: f32,
        start_y: f32,
        end_x: f32,
        end_y: f32,
        game_objects: &'a mut HashMap<u64, GameObject>,
    ) -> Vec<&'a mut GameObject> {
        let mut result = vec![];

        let start_index = self.convert_world_location_to_grid_location(start_x, start_y);
        let end_index = self.convert_world_location_to_grid_location(end_x, end_y);

        for y_index in start_index.y..end_index.y {
            for x_index in start_index.x..end_index.x {
                if y_index >= self.cells.len() || x_index >= self.cells[0].len() {
                    continue;
                }
                for id in self.cells[y_index][x_index].iter_mut() {
                    if let Some(game_object) = game_objects.get_mut(id) {
                        result.push(game_object);
                    }
                }
            }
        }

        result
    }

    pub fn update(
        &mut self,
        game_objects: &mut HashMap<u64, GameObject>,
        gravity_force: Vector2<f32>,
    ) {
        self.move_game_objects(game_objects, gravity_force);
    }

    fn move_game_objects(
        &mut self,
        game_objects: &mut HashMap<u64, GameObject>,
        gravity_force: Vector2<f32>,
    ) {
        // run update on all game objects
        for game_object in game_objects.values_mut() {
            let previous_index = self.convert_world_location_to_grid_location(
                game_object.location.x,
                game_object.location.y,
            );
            game_object.update(gravity_force);
            let next_index = self.convert_world_location_to_grid_location(
                game_object.location.x,
                game_object.location.y,
            );
            if self.still_in_same_cell(previous_index, next_index) {
                continue;
            }
            self.remove(&game_object);
            if self.is_outside_of_grid(next_index) {
                println!("Object with id {} is out of the grid", game_object.id);
                if let Err(error) = self.game_object_off_grid_event.send(game_object.id) {
                    println!("error sending game object off grid event: {}", error);
                }
            } else {
                self.add(game_object);
            }
        }
    }

    pub fn remove(&mut self, game_object: &GameObject) {
        let index = self.convert_world_location_to_grid_location(
            game_object.location.x,
            game_object.location.y,
        );

        if !self.is_outside_of_grid(index) {
            self.cells[index.y][index.x].retain(|id| id != &game_object.id);
        }
    }

    fn convert_world_location_to_grid_location(&self, x: f32, y: f32) -> Point2<usize> {
        Point2::new(
            (x / self.cell_width) as usize,
            (y / self.cell_height) as usize,
        )
    }

    fn still_in_same_cell(&self, previous_index: Point2<usize>, next_index: Point2<usize>) -> bool {
        previous_index.x == next_index.x && previous_index.y == next_index.y
    }

    fn is_outside_of_grid(&self, index: Point2<usize>) -> bool {
        index.x >= self.world_width || index.y >= self.world_height.into()
    }
}
