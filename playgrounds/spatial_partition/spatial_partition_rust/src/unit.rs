use super::unit_moving::UnitMoving;
use super::Grid;
use ggez::graphics::{Color, DrawMode, DrawParam, Mesh, MeshBuilder, WHITE};
use ggez::nalgebra::{Point2, Vector2};
use ggez::{graphics, Context, GameResult};
use rand::prelude::*;

#[derive(Debug, Clone, Copy)]
pub struct Unit {
    pub x: f32,
    pub y: f32,
    old_x: f32,
    old_y: f32,
    pub id: u64,
    pub color: Color,
    velocity_x: f32,
    velocity_y: f32,
    radius: f32,
}

impl Unit {
    pub fn new(
        x: f32,
        y: f32,
        context: &mut Context,
        id: u64,
        rng: &mut ThreadRng,
        radius: f32,
    ) -> GameResult<Unit> {
        let color = WHITE;
        Ok(Unit {
            x,
            y,
            old_x: 0.0,
            old_y: 0.0,
            id,
            color,
            velocity_x: rng.gen_range(-1.0, 1.0),
            velocity_y: rng.gen_range(-1.0, 1.0),
            radius,
        })
    }

    pub fn draw(
        &self,
        context: &mut Context,
        _cell_index_x: usize,
        _cell_index_y: usize,
        mesh: &Mesh,
    ) -> GameResult<()> {
        graphics::draw(
            context,
            mesh,
            DrawParam::new()
                .dest(Point2::new(self.x, self.y))
                .color(self.color),
        )?;

        // let cell_coordinate_text =
        //     graphics::Text::new(format!("({},{})", cell_index_x, cell_index_y));
        // graphics::draw(
        //     context,
        //     &cell_coordinate_text,
        //     DrawParam::new().dest(Point2::new(self.x + self.radius, self.y + self.radius)),
        // )

        Ok(())
    }

    pub fn update(&mut self) {
        self.old_x = self.x;
        self.old_y = self.y;
        self.x += self.velocity_x;
        self.y += self.velocity_y;
    }

    pub fn collide_with_top_wall(&mut self) {
        if self.y - self.radius < 0.0 {
            self.y = self.radius;
            self.velocity_y *= -1.0;
        }
    }

    pub fn collide_with_left_wall(&mut self) {
        if self.x - self.radius < 0.0 {
            self.x = self.radius;
            self.velocity_x *= -1.0;
        }
    }

    pub fn collide_with_right_wall(&mut self, arena_width: f32) {
        if self.x + self.radius > arena_width {
            self.x = arena_width - self.radius;
            self.velocity_x *= -1.0;
        }
    }

    pub fn collide_with_bottom_wall(&mut self, arena_height: f32) {
        if self.y + self.radius > arena_height {
            self.y = arena_height - self.radius;
            self.velocity_y *= -1.0;
        }
    }

    pub fn get_move(&self) -> UnitMoving {
        UnitMoving::new(self.old_x, self.old_y, self.x, self.y, self.id)
    }

    pub fn handle_collision(&mut self, other: &Unit) {
        let my_location = Vector2::new(self.x, self.y);
        let other_location = Vector2::new(other.x, other.y);
        let direction = my_location - other_location;
        let mut distance = direction.magnitude();

        distance -= self.radius * 2.0;

        if distance < self.radius * 2.0 {
            if self.y <= other.y {
                self.velocity_y *= -1.0
            }

            if self.x <= other.x {
                self.velocity_x *= -1.0
            }

            let new_velocity = Vector2::new(self.velocity_x, self.velocity_y).normalize() * 1.0;
            self.velocity_x = new_velocity.x;
            self.velocity_y = new_velocity.y;
        }
    }
}
