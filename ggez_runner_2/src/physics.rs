use super::{GameObject, Types};
use ggez::nalgebra::Vector2;
use std::sync::mpsc::{Receiver, Sender};

#[derive(Debug, PartialEq)]
pub enum PhysicsState {
    Standing,
    Jumping,
    Falling,
    Running,
    Dead,
}

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

    fn get_state(&self) -> &PhysicsState;
}

#[derive(Debug)]
pub struct PlayerPhysics {
    velocity: Vector2<f32>,
    speed: f32,
    moved_event_send: Sender<f32>,
    won_event_send: Sender<()>,
    died_event_send: Sender<()>,
    jump_command: Receiver<()>,
    jump_velocity: Vector2<f32>,
    pub state: PhysicsState,
    run_command: Receiver<()>,
}

impl PlayerPhysics {
    pub fn new(
        speed: f32,
        moved_event_send: Sender<f32>,
        won_event_send: Sender<()>,
        died_event_send: Sender<()>,
        jump_command: Receiver<()>,
        jump_force: f32,
        run_command: Receiver<()>,
    ) -> PlayerPhysics {
        PlayerPhysics {
            velocity: Vector2::new(0.0, 0.0),
            speed,
            moved_event_send,
            won_event_send,
            died_event_send,
            jump_command,
            jump_velocity: Vector2::new(0.0, -jump_force),
            state: PhysicsState::Standing,
            run_command,
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
            && location.y + height <= floor.location.y + self.velocity.y.abs()
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

    fn is_running_or_falling(&self) -> bool {
        self.state == PhysicsState::Falling || self.state == PhysicsState::Running
    }
}

impl Physics for PlayerPhysics {
    fn update(&mut self, location: &mut Vector2<f32>, gravity_force: Vector2<f32>) {
        self.velocity += gravity_force;
        *location += self.velocity;
        if let Err(error) = self.moved_event_send.send(self.velocity.x) {
            println!("could not send location when player moving: {}", error);
        }

        if let Ok(_) = self.jump_command.try_recv() {
            if self.state == PhysicsState::Running {
                self.velocity += self.jump_velocity;
                self.state = PhysicsState::Jumping;
            }
        }

        if self.state == PhysicsState::Jumping {
            if self.velocity.y > 0.0 {
                self.state = PhysicsState::Falling;
            }
        }

        if let Ok(_) = self.run_command.try_recv() {
            self.velocity.x = self.speed;
            self.state = PhysicsState::Running;
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
                    self.state = PhysicsState::Dead;
                }
            }

            if Types::Floor == game_object.my_type {
                if self.is_running_or_falling()
                    && self.is_standing_on(location, width, height, game_object)
                {
                    self.state = PhysicsState::Running;
                    location.y = game_object.location.y - height;
                    self.velocity.y = 0.0;
                } else if self.colliding_with(location, width, height, game_object) {
                    if self.state == PhysicsState::Jumping {
                        location.y = game_object.location.y - height;
                    } else {
                        self.velocity.x = 0.0;
                        location.x = game_object.location.x - width;
                    }
                }
            }
        }
    }

    fn reset(&mut self, speed: f32) {
        self.velocity *= 0.0;
        self.state = PhysicsState::Standing;
    }

    fn get_state(&self) -> &PhysicsState {
        &self.state
    }
}
