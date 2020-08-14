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

    fn is_standing_on(
        &self,
        location: &mut Vector2<f32>,
        width: &f32,
        height: &f32,
        floor: &GameObject,
    ) -> bool {
        location.x < floor.location.x + floor.width
            && location.x + width > floor.location.x
            && location.y + height > floor.location.y
            && location.y + height <= floor.location.y + self.velocity.y
    }

    fn colliding_with(
        &self,
        location: &Vector2<f32>,
        width: &f32,
        height: &f32,
        game_object: &GameObject,
    ) -> bool {
        location.x + width > game_object.location.x
            && location.x < game_object.location.x + game_object.width
            && location.y + height > game_object.location.y
            && location.y < game_object.location.y + game_object.height
    }

    fn is_past_other_game_object(
        &self,
        location: &mut Vector2<f32>,
        game_object: &GameObject,
    ) -> bool {
        location.x > game_object.location.x + game_object.width
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
                if self.is_past_other_game_object(location, game_object) {
                    if let Err(error) = self.won_event_send.send(()) {
                        println!("Error sending win event: {}", error);
                    }
                }
            }

            if Types::SpikeUp == game_object.my_type {
                if self.colliding_with(location, width, height, game_object) {
                    if let Err(error) = self.died_event_send.send(()) {
                        println!("Error sending died event: {}", error);
                    }
                }
            }

            if Types::Floor == game_object.my_type {
                if self.is_standing_on(location, width, height, game_object) {
                    println!("standing on a floor");
                    location.y = game_object.location.y - height;
                    self.velocity.y = 0.0;
                } else if self.colliding_with(location, width, height, game_object) {
                    self.velocity.x = 0.0;
                    location.x = game_object.location.x - width;
                }
            }
        }
    }

    fn reset(&mut self, speed: f32) {
        self.velocity.x = speed;
        self.velocity.y = 0.0;
    }
}
