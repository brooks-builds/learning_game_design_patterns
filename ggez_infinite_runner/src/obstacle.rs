use ggez::graphics::{DrawMode, Mesh, MeshBuilder, WHITE};
use ggez::nalgebra::{Point2, Vector2};
use ggez::{Context, GameResult};

pub struct Obstacle {
    location: Vector2<f32>,
    location_to_reset_to: Vector2<f32>,
    velocity: Vector2<f32>,
    width: f32,
    height: f32,
    speed_increase_rate: f32,
}

impl Obstacle {
    pub fn new(location_x: f32, location_y: f32, size: f32, arena_width: f32) -> Obstacle {
        let location = Vector2::new(location_x, location_y);
        let location_to_reset_to = Vector2::new(arena_width + size, location_y);
        let velocity = Vector2::new(-0.1, 0.0);
        let width = size;
        let height = size;
        let speed_increase_rate = -0.01;

        Obstacle {
            location,
            location_to_reset_to,
            velocity,
            width,
            height,
            speed_increase_rate,
        }
    }

    pub fn create_mesh(&self, context: &mut Context) -> GameResult<Mesh> {
        let triangle_points = [
            Point2::new(0.0, 0.0),
            Point2::new(self.width / 2.0, self.height),
            Point2::new(self.width, 0.0),
        ];
        MeshBuilder::new()
            .polyline(DrawMode::fill(), &triangle_points, WHITE)?
            .build(context)
    }

    pub fn get_location(&self) -> Point2<f32> {
        Point2::new(self.location.x, self.location.y)
    }

    pub fn run(&mut self) {
        self.location += self.velocity;
    }

    pub fn reset_location(&mut self) {
        self.location = self.location_to_reset_to;
    }

    pub fn is_offscreen(&self, arena_width: f32) -> bool {
        self.location.x + self.width < 0.0
    }

    pub fn increase_speed(&mut self) {
        self.velocity.x += self.speed_increase_rate;
    }
}
