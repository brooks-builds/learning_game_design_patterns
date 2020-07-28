mod particle;
mod particle_pool;

use ggez::event::EventHandler;
use ggez::graphics::{Color, BLACK};
use ggez::nalgebra::Point2;
use ggez::{graphics, Context, GameResult};
use particle::Particle;
use particle_pool::ParticlePool;
use rand::prelude::*;

pub struct GameState {
    particle_pool: ParticlePool,
    center_of_screen: Point2<f32>,
    rng: ThreadRng,
}

impl GameState {
    pub fn new(context: &mut Context) -> GameResult<GameState> {
        let particle_pool = ParticlePool::new(context)?;
        let (screen_width, screen_height) = graphics::drawable_size(context);
        let center_of_screen = Point2::new(screen_width / 2.0, screen_height / 2.0);
        let rng = thread_rng();
        Ok(GameState {
            particle_pool,
            center_of_screen,
            rng,
        })
    }
}

impl EventHandler for GameState {
    fn update(&mut self, _context: &mut Context) -> GameResult {
        let velocity = Point2::new(self.rng.gen_range(-1.0, 1.0), self.rng.gen_range(-1.0, 1.0));
        self.particle_pool
            .create_particle(self.center_of_screen, velocity, 500);
        self.particle_pool.animate();
        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> GameResult {
        graphics::clear(context, BLACK);

        self.particle_pool.draw(context)?;

        graphics::present(context)
    }
}
