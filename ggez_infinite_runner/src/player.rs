use ggez::graphics::{DrawMode, Mesh, MeshBuilder, Rect, WHITE};
use ggez::nalgebra::{Point2, Vector2};
use ggez::{Context, GameResult};

pub struct Player {
    location: Vector2<f32>,
    height: f32,
    width: f32,
}

impl Player {
    pub fn new(location_x: f32, location_y: f32) -> Player {
        Player {
            location: Vector2::new(location_x, location_y),
            height: 50.0,
            width: 15.0,
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
}
