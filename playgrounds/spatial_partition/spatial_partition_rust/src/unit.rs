use super::unit_moving::UnitMoving;
use super::Grid;
use ggez::graphics::{Color, DrawMode, DrawParam, Mesh, MeshBuilder, WHITE};
use ggez::nalgebra::Point2;
use ggez::{graphics, Context, GameResult};
use rand::prelude::*;

#[derive(Debug)]
pub struct Unit {
    pub x: f32,
    pub y: f32,
    mesh: Mesh,
    pub id: u64,
    pub color: Color,
    velocity_x: f32,
    velocity_y: f32,
}

impl Unit {
    pub fn new(x: f32, y: f32, context: &mut Context, id: u64) -> GameResult<Unit> {
        let color = WHITE;
        let mesh = MeshBuilder::new()
            .circle(DrawMode::fill(), Point2::new(0.0, 0.0), 5.0, 0.5, color)
            .build(context)?;
        Ok(Unit {
            x,
            y,
            mesh,
            id,
            color,
            velocity_x: 0.0,
            velocity_y: 0.0,
        })
    }

    pub fn draw(&self, context: &mut Context) -> GameResult<()> {
        graphics::draw(
            context,
            &self.mesh,
            DrawParam::new()
                .dest(Point2::new(self.x, self.y))
                .color(self.color),
        )
    }

    pub fn update(&mut self, rng: &mut ThreadRng) -> UnitMoving {
        self.velocity_x += rng.gen_range(-1.0, 1.0);
        self.velocity_y += rng.gen_range(-1.0, 1.0);
        let new_x = self.x + self.velocity_x;
        let new_y = self.y + self.velocity_y;
        let unit_moving = UnitMoving::new(self.x, self.y, new_x, new_y, self.id);
        self.x = new_x;
        self.y = new_y;
        unit_moving
    }
}
