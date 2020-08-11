use super::game_object::GameObject;
use ggez::graphics::{DrawMode, DrawParam, Mesh, MeshBuilder, Rect, WHITE};
use ggez::nalgebra::Point2;
use ggez::{graphics, Context, GameResult};
use std::collections::HashMap;

pub struct Grid {
    cell_width: f32,
    cell_height: f32,
    cell_mesh: Mesh,
    cells: Vec<Vec<HashMap<u64, GameObject>>>,
}

impl Grid {
    pub fn new(
        cell_width: f32,
        cell_height: f32,
        context: &mut Context,
        world_height: u8,
    ) -> GameResult<Grid> {
        let cell_mesh = MeshBuilder::new()
            .rectangle(
                DrawMode::stroke(1.0),
                Rect::new(0.0, 0.0, cell_width, cell_height),
                WHITE,
            )
            .build(context)?;
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
            cell_mesh,
            cells,
        })
    }

    pub fn draw(&self, context: &mut Context) -> GameResult {
        for (y_index, row) in self.cells.iter().enumerate() {
            for (x_index, _cell) in row.iter().enumerate() {
                graphics::draw(
                    context,
                    &self.cell_mesh,
                    DrawParam::new().dest(Point2::new(
                        x_index as f32 * self.cell_width,
                        y_index as f32 * self.cell_height,
                    )),
                )?;
            }
        }
        Ok(())
    }
}
