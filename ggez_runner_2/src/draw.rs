use super::{physics, Meshes, States, Types};
use ggez::nalgebra::{Point2, Vector2};
use ggez::{graphics, timer, Context, GameResult};
use graphics::DrawParam;
use physics::{Physics, PhysicsState};
use std::time::{Duration, Instant};

pub trait Draw
where
    Self: std::fmt::Debug,
{
    fn draw(
        &self,
        my_type: &Types,
        meshes: &Meshes,
        physics: &Option<Box<dyn Physics>>,
        context: &mut Context,
        location: &Vector2<f32>,
        game_state: &States,
    ) -> GameResult;

    fn update(&mut self);
}

#[derive(Debug)]
pub struct PlayerDraw {
    switch_walk_every: Duration,
    switch_walk_at: Instant,
    walk_animation_1: bool,
}

impl PlayerDraw {
    pub fn new() -> PlayerDraw {
        let switch_walk_every = Duration::from_millis(100);
        let switch_walk_at = Instant::now() + switch_walk_every;
        let walk_animation_1 = true;

        PlayerDraw {
            switch_walk_every,
            switch_walk_at,
            walk_animation_1,
        }
    }
}

impl Draw for PlayerDraw {
    fn draw(
        &self,
        my_type: &Types,
        meshes: &Meshes,
        physics: &Option<Box<dyn Physics>>,
        context: &mut Context,
        location: &Vector2<f32>,
        game_state: &States,
    ) -> GameResult {
        if let Some(physics) = physics {
            let (mut mesh, shift_x) = match physics.get_state() {
                PhysicsState::Standing => (&meshes.player_standing, 0.0),
                PhysicsState::Running => (
                    match self.walk_animation_1 {
                        true => &meshes.player_walk_1,
                        false => &meshes.player_walk_2,
                    },
                    0.0,
                ),
                PhysicsState::Jumping | PhysicsState::Falling => (&meshes.player_jumping, 0.0),
                PhysicsState::Dead => {
                    (&meshes.player_dead, (meshes.player_dead.width() / 4) as f32)
                }
            };

            if let States::Won = game_state {
                mesh = match self.walk_animation_1 {
                    true => &meshes.player_standing,
                    false => &meshes.player_jumping,
                };
            }

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

    fn update(&mut self) {
        let now = Instant::now();
        if now >= self.switch_walk_at {
            self.walk_animation_1 = !self.walk_animation_1;
            self.switch_walk_at = now + self.switch_walk_every;
        }
    }
}
