const UNITS_TO_CREATE: u32 = 500;
const UPDATE_FPS: u32 = 50;

mod grid;
mod unit;
mod unit_moving;

use ggez::event::EventHandler;
use ggez::graphics::{DrawMode, Mesh, MeshBuilder, BLACK, WHITE};
use ggez::nalgebra::Point2;
use ggez::{graphics, Context, GameResult};
use grid::Grid;
use rand::prelude::*;

pub struct GameState {
    grid: Grid,
    rng: ThreadRng,
    unit_mesh: Mesh,
}

impl GameState {
    pub fn new(context: &mut Context) -> GameResult<GameState> {
        let mut grid = Grid::new(context)?;
        let mut rng = thread_rng();
        let (arena_width, arena_height) = graphics::drawable_size(context);
        let unit_radius = 5.0;
        let unit_mesh = MeshBuilder::new()
            .circle(
                DrawMode::fill(),
                Point2::new(0.0, 0.0),
                unit_radius,
                0.5,
                WHITE,
            )
            .build(context)?;

        for _ in 0..UNITS_TO_CREATE {
            grid.add(
                rng.gen_range(0.0, arena_width),
                rng.gen_range(0.0, arena_height),
                context,
                &mut rng,
                unit_radius,
            )?;
        }
        Ok(GameState {
            grid,
            rng,
            unit_mesh,
        })
    }
}

impl EventHandler for GameState {
    fn update(&mut self, context: &mut Context) -> GameResult {
        while ggez::timer::check_update_time(context, UPDATE_FPS) {
            self.grid.update(&mut self.rng, context);
        }
        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> GameResult {
        graphics::clear(context, BLACK);

        self.grid.draw(context, &self.unit_mesh)?;

        graphics::present(context)
    }
}
