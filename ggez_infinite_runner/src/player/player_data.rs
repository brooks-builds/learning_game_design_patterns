use super::StateData;
use ggez::nalgebra::{Point2, Vector2};

pub struct PlayerData {
    pub location: Vector2<f32>,
    pub starting_location: Vector2<f32>,
    pub height: f32,
    pub width: f32,
    pub acceleration: Vector2<f32>,
    pub velocity: Vector2<f32>,
    pub jump_force: Vector2<f32>,
    pub is_jumping: bool,
}

impl PlayerData {
    pub fn new(x: f32, y: f32) -> PlayerData {
        PlayerData {
            location: Vector2::new(x, y),
            starting_location: Vector2::new(x, y),
            height: 50.0,
            width: 15.0,
            acceleration: Vector2::new(0.0, 0.0),
            velocity: Vector2::new(0.0, 0.0),
            jump_force: Vector2::new(0.0, -9.0),
            is_jumping: false,
        }
    }

    pub fn apply_force(&mut self, force: Vector2<f32>) {
        self.acceleration + force;
    }
}

impl StateData for PlayerData {
    fn get_width(&self) -> f32 {
        self.width
    }

    fn get_height(&self) -> f32 {
        self.height
    }

    fn get_location_as_point(&self) -> ggez::nalgebra::Point2<f32> {
        Point2::new(self.location.x, self.location.y)
    }
}
