use super::{CustomError, GameObject, Meshes};
use ggez::graphics::{DrawMode, DrawParam, Mesh, MeshBuilder, Rect, WHITE};
use ggez::nalgebra::Point2;
use ggez::{graphics, Context, GameResult};
use std::collections::HashMap;

#[derive(Debug)]
pub struct Grid {
    cell_width: f32,
    cell_height: f32,
    cells: Vec<Vec<HashMap<u64, GameObject>>>,
}

impl Grid {
    pub fn new(cell_width: f32, cell_height: f32, world_height: u8) -> GameResult<Grid> {
        let mut cells = vec![];

        for _y_count in 0..world_height {
            let mut row = vec![];
            for _x_count in 0..cell_width as u32 {
                row.push(HashMap::new());
            }
            cells.push(row);
        }

        Ok(Grid {
            cell_width,
            cell_height,
            cells,
        })
    }

    pub fn draw(&self, context: &mut Context, meshes: &Meshes) -> Result<(), CustomError> {
        for (y_index, row) in self.cells.iter().enumerate() {
            for (x_index, cell) in row.iter().enumerate() {
                // if let Err(error) = graphics::draw(
                //     context,
                //     &meshes.cell,
                //     DrawParam::new().dest(Point2::new(
                //         x_index as f32 * self.cell_width,
                //         y_index as f32 * self.cell_height,
                //     )),
                // ) {
                //     return Err(CustomError::GgezGameError(error));
                // }
                for (id, game_object) in cell {
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
}
