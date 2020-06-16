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
            jump_force: Vector2::new(0.0, -7.0),
            is_jumping: false,
        }
    }
}
