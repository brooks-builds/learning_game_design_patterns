const NUMBER_OF_CELLS: usize = 8;
const CELL_SIZE: usize = 100;

use super::unit::Unit;
use super::unit_moving::UnitMoving;
use ggez::graphics::{Color, DrawParam, Mesh, MeshBuilder, WHITE};
use ggez::nalgebra::Point2;
use ggez::{graphics, Context, GameResult};
use rand::prelude::*;
use std::collections::HashMap;

pub struct Grid {
    cells: Vec<Vec<HashMap<u64, Unit>>>,
    horizontile_line_mesh: Mesh,
    vertical_line_mesh: Mesh,
    next_id: u64,
}

impl Grid {
    pub fn new(context: &mut Context) -> GameResult<Grid> {
        let mut cells = vec![];
        for _ in 0..NUMBER_OF_CELLS {
            let mut x_cells = vec![];
            for _x in 0..NUMBER_OF_CELLS {
                x_cells.push(HashMap::new());
            }
            cells.push(x_cells);
        }

        let (screen_width, screen_height) = graphics::drawable_size(context);
        let horizontile_line_mesh = MeshBuilder::new()
            .line(
                &[Point2::new(0.0, 0.0), Point2::new(screen_width, 0.0)],
                1.0,
                WHITE,
            )?
            .build(context)?;

        let vertical_line_mesh = MeshBuilder::new()
            .line(
                &[Point2::new(0.0, 0.0), Point2::new(0.0, screen_height)],
                1.0,
                WHITE,
            )?
            .build(context)?;
        let next_id = 0;
        Ok(Grid {
            cells,
            horizontile_line_mesh,
            vertical_line_mesh,
            next_id,
        })
    }

    pub fn draw(&self, context: &mut Context, unit_mesh: &Mesh) -> GameResult<()> {
        for count in 1..NUMBER_OF_CELLS {
            graphics::draw(
                context,
                &self.horizontile_line_mesh,
                DrawParam::new().dest(Point2::new(0.0, (count * CELL_SIZE) as f32)),
            )?;
            graphics::draw(
                context,
                &self.vertical_line_mesh,
                DrawParam::new().dest(Point2::new((count * CELL_SIZE) as f32, 0.0)),
            )?;
        }

        for (y_index, y_row) in self.cells.iter().enumerate() {
            for (x_index, units) in y_row.iter().enumerate() {
                for (unit_id, unit) in units {
                    unit.draw(context, x_index, y_index, unit_mesh)?;
                }
            }
        }

        Ok(())
    }

    pub fn add(
        &mut self,
        x: f32,
        y: f32,
        context: &mut Context,
        rng: &mut ThreadRng,
        unit_radius: f32,
    ) -> GameResult<()> {
        let unit = Unit::new(x, y, context, self.next_id, rng, unit_radius)?;
        let cell_x = x as usize / CELL_SIZE;
        let cell_y = y as usize / CELL_SIZE;
        self.cells[cell_y][cell_x].insert(unit.id, unit);
        self.next_id += 1;
        Ok(())
    }

    pub fn update(&mut self, rng: &mut ThreadRng, context: &Context) {
        let (arena_width, arena_height) = graphics::drawable_size(context);
        let mut unit_moves = vec![];
        for (y_index, y_row) in &mut self.cells.iter_mut().enumerate() {
            for (x_index, units) in y_row.iter_mut().enumerate() {
                let cloned_units = units.clone();
                for (unit_id, unit) in units {
                    // this will potentially change the position of the unit
                    unit.update();
                    if y_index == 0 {
                        unit.collide_with_top_wall();
                    } else if y_index == NUMBER_OF_CELLS - 1 {
                        unit.collide_with_bottom_wall(arena_height);
                    }
                    if x_index == 0 {
                        unit.collide_with_left_wall();
                    } else if x_index == NUMBER_OF_CELLS - 1 {
                        unit.collide_with_right_wall(arena_width);
                    }

                    // have unit collide with each other
                    for (cloned_unit_index, cloned_unit) in &cloned_units {
                        if unit_id == cloned_unit_index {
                            continue;
                        }
                        unit.handle_collision(cloned_unit);
                    }
                    unit_moves.push(unit.get_move());
                }
            }
        }
        self.handle_moving_cells(unit_moves, rng);
    }

    pub fn handle_moving_cells(&mut self, unit_moves: Vec<UnitMoving>, rng: &mut ThreadRng) {
        for unit_moving in unit_moves {
            let old_x = unit_moving.old_x as usize / CELL_SIZE;
            let old_y = unit_moving.old_y as usize / CELL_SIZE;
            let new_x = unit_moving.new_x as usize / CELL_SIZE;
            let new_y = unit_moving.new_y as usize / CELL_SIZE;
            if old_x == new_x && old_y == new_y {
                continue;
            }
            let unit = self.cells[old_y][old_x].remove(&unit_moving.unit_id);
            let random_color = Color::new(
                rng.gen_range(0.0, 1.0),
                rng.gen_range(0.0, 1.0),
                rng.gen_range(0.0, 1.0),
                1.0,
            );
            if let Some(mut unit) = unit {
                unit.color = random_color;
                self.cells[new_y][new_x].insert(unit.id, unit);
            }
        }
    }
}
