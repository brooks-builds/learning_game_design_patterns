use ggez::graphics::{DrawMode, Mesh, MeshBuilder, Rect, WHITE};
use ggez::nalgebra::{Point2, Vector2};
use ggez::{Context, GameResult};

pub struct Player {
    location: Vector2<f32>,
    height: f32,
    width: f32,
    acceleration: Vector2<f32>,
    velocity: Vector2<f32>,
    jump_force: Vector2<f32>,
    is_jumping: bool,
}

impl Player {
    pub fn new(location_x: f32, location_y: f32) -> Player {
        let acceleration = Vector2::new(0.0, 0.0);
        let velocity = Vector2::new(0.0, 0.0);
        let jump_force = Vector2::new(0.0, -0.1);
        let is_jumping = false;

        Player {
            location: Vector2::new(location_x, location_y),
            height: 50.0,
            width: 15.0,
            acceleration,
            velocity,
            jump_force,
            is_jumping,
        }
    }

    pub fn create_mesh(&self, context: &mut Context) -> GameResult<Mesh> {
        let rect_bounds = Rect::new(0.0, 0.0, self.width, self.height);
        let mesh = MeshBuilder::new()
            .rectangle(DrawMode::fill(), rect_bounds, WHITE)
            .build(context)?;

        Ok(mesh)
    }

    pub fn get_location(&self) -> Point2<f32> {
        Point2::new(self.location.x, self.location.y)
    }

    pub fn apply_force(&mut self, force: &Vector2<f32>) {
        self.acceleration += force;
    }

    pub fn run(&mut self) {
        self.velocity += self.acceleration;
        self.location += self.velocity;
        self.acceleration *= 0.0;
    }

    pub fn hit_ground(&mut self, arena_height: f32) {
        if self.location.y + self.height > arena_height {
            self.location.y = arena_height - self.height;
            self.velocity.y = 0.0;
            self.is_jumping = false;
        }
    }

    pub fn jump(&mut self) {
        if !self.is_jumping {
            let jump_force = self.jump_force.clone();
            self.apply_force(&jump_force);
            self.is_jumping = true;
        }
    }
}
