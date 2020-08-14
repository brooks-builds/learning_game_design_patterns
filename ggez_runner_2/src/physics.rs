use super::{GameObject, Types};
use ggez::nalgebra::Vector2;
use std::sync::mpsc::Sender;

pub trait Physics
where
    Self: std::fmt::Debug,
{
    fn update(&mut self, location: &mut Vector2<f32>, gravity_force: Vector2<f32>);

    fn handle_collisions(
        &mut self,
        location: &mut Vector2<f32>,
        other_game_objects: Vec<&GameObject>,
        height: &f32,
        width: &f32,
    );

    fn reset(&mut self, speed: f32);
}

#[derive(Debug)]
pub struct PlayerPhysics {
    velocity: Vector2<f32>,
    moved_event_send: Sender<f32>,
    won_event_send: Sender<()>,
    died_event_send: Sender<()>,
}

impl PlayerPhysics {
    pub fn new(
        speed: f32,
        moved_event_send: Sender<f32>,
        won_event_send: Sender<()>,
        died_event_send: Sender<()>,
    ) -> PlayerPhysics {
        PlayerPhysics {
            velocity: Vector2::new(speed, 0.0),
            moved_event_send,
            won_event_send,
            died_event_send,
        }
    }

    fn is_clipping_into_side_of_game_object(
        &self,
        our_location: &Vector2<f32>,
        our_width: &f32,
        our_height: &f32,
        other: &GameObject,
    ) -> bool {
        our_location.y + our_height > other.location.y
            && our_location.x + our_width > other.location.x
            && our_location.x < other.location.x
    }

    fn is_in_game_object(
        &self,
        our_location: &Vector2<f32>,
        our_width: &f32,
        our_height: &f32,
        other: &GameObject,
    ) -> bool {
        our_location.y + our_height > other.location.y
            && our_location.x + our_width < other.location.x + other.width
            && our_location.x > other.location.x
    }
}

impl Physics for PlayerPhysics {
    fn update(&mut self, location: &mut Vector2<f32>, gravity_force: Vector2<f32>) {
        self.velocity += gravity_force;
        *location += self.velocity;
        if let Err(error) = self.moved_event_send.send(self.velocity.x) {
            println!("could not send location when player moving: {}", error);
        }
    }

    fn handle_collisions(
        &mut self,
        location: &mut Vector2<f32>,
        game_objects: Vec<&GameObject>,
        height: &f32,
        width: &f32,
    ) {
        for game_object in game_objects {
            if Types::End == game_object.my_type {
                if let Err(error) = self.won_event_send.send(()) {
                    println!("Error sending win event: {}", error);
                }
            }

            if Types::SpikeUp == game_object.my_type {
                if let Err(error) = self.died_event_send.send(()) {
                    println!("Error sending died event: {}", error);
                }
            }

            if Types::Floor == game_object.my_type {
                if self.is_clipping_into_side_of_game_object(location, width, height, game_object) {
                    self.velocity.x = 0.0;
                    location.x = game_object.location.x - width;
                } else if self.is_in_game_object(location, width, height, game_object) {
                    location.y = game_object.location.y - height;
                    self.velocity.y = 0.0;
                }
            }
        }
    }

    fn reset(&mut self, speed: f32) {
        self.velocity.x = speed;
        self.velocity.y = 0.0;
    }
}
