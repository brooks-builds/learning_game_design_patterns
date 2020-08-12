use super::{GameObject, Types};
use ggez::nalgebra::Vector2;
use std::sync::mpsc::Sender;

pub trait Physics
where
    Self: std::fmt::Debug,
{
    fn update(&mut self, location: &mut Vector2<f32>);

    fn handle_collisions(&mut self, other_game_objects: Vec<&GameObject>);
}

#[derive(Debug)]
pub struct PlayerPhysics {
    velocity: Vector2<f32>,
    moved_event_send: Sender<f32>,
    won_event_send: Sender<()>,
}

impl PlayerPhysics {
    pub fn new(
        speed: f32,
        moved_event_send: Sender<f32>,
        won_event_send: Sender<()>,
    ) -> PlayerPhysics {
        PlayerPhysics {
            velocity: Vector2::new(speed, 0.0),
            moved_event_send,
            won_event_send,
        }
    }
}

impl Physics for PlayerPhysics {
    fn update(&mut self, location: &mut Vector2<f32>) {
        *location += self.velocity;
        if let Err(error) = self.moved_event_send.send(self.velocity.x) {
            println!("could not send location when player moving: {}", error);
        }
    }

    fn handle_collisions(&mut self, other_game_objects: Vec<&GameObject>) {
        for game_object in other_game_objects {
            if let Types::End = game_object.my_type {
                if let Err(error) = self.won_event_send.send(()) {
                    println!("cound not send won event :(. I guess we lose forever now");
                }
            }
        }
    }
}

#[derive(Debug)]
pub struct StaticPhysics;

impl Physics for StaticPhysics {
    fn update(&mut self, _location: &mut Vector2<f32>) {}

    fn handle_collisions(&mut self, _other_game_objects: Vec<&GameObject>) {}
}
