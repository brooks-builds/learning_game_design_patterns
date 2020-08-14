use super::{physics, Meshes, Types};
use ggez::nalgebra::{Point2, Vector2};
use ggez::{graphics, Context, GameResult};
use graphics::DrawParam;
use physics::{Physics, PhysicsState};
use std::time::{Duration, Instant};

pub trait Draw
where
    Self: std::fmt::Debug,
{
    fn draw(
        &mut self,
        my_type: &Types,
        meshes: &Meshes,
        physics: &Option<Box<dyn Physics>>,
        context: &mut Context,
        location: &Vector2<f32>,
    ) -> GameResult;
}

#[derive(Debug)]
pub struct PlayerDraw {
    switch_walk_every: Duration,
    walking_1: bool,
    switch_walk_at: Instant,
}

impl PlayerDraw {
    pub fn new() -> PlayerDraw {
        let switch_walk_every = Duration::from_millis(250);
        let walking_1 = true;
        let switch_walk_at = Instant::now() + switch_walk_every;

        PlayerDraw {
            switch_walk_every,
            walking_1,
            switch_walk_at,
        }
    }
}

impl Draw for PlayerDraw {
    fn draw(
        &mut self,
        my_type: &Types,
        meshes: &Meshes,
        physics: &Option<Box<dyn Physics>>,
        context: &mut Context,
        location: &Vector2<f32>,
    ) -> GameResult {
        if let Some(physics) = physics {
            let (mesh, shift_x) = match physics.get_state() {
                PhysicsState::Standing => (&meshes.player_standing, 0.0),
                PhysicsState::Running => {
                    let now = Instant::now();

                    if now > self.switch_walk_at {
                        self.walking_1 = !self.walking_1;
                        self.switch_walk_at = now + self.switch_walk_every;
                    }
                    if self.walking_1 {
                        (&meshes.player_walk_1, 0.0)
                    } else {
                        (&meshes.player_walk_2, 0.0)
                    }
                }
                PhysicsState::Jumping | PhysicsState::Falling => (&meshes.player_jumping, 0.0),
                PhysicsState::Dead => {
                    (&meshes.player_dead, (meshes.player_dead.width() / 4) as f32)
                }
            };

            graphics::draw(
                context,
                mesh,
                DrawParam::new()
                    .dest(Point2::new(location.x + shift_x, location.y))
                    .scale([0.25, 0.25]),
            )?;
        }

        Ok(())
    }
}
