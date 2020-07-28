const POOL_SIZE: usize = 500;

use super::Particle;
use ggez::graphics::{Color, DrawMode, Mesh, MeshBuilder, WHITE};
use ggez::nalgebra::Point2;
use ggez::{Context, GameResult};

pub struct ParticlePool {
    particles: [Particle; POOL_SIZE],
    mesh: Mesh,
}

impl ParticlePool {
    pub fn new(context: &mut Context) -> GameResult<ParticlePool> {
        let mesh = MeshBuilder::new()
            .circle(DrawMode::fill(), Point2::new(0.0, 0.0), 5.0, 0.1, WHITE)
            .build(context)?;
        let particles = [Particle::new(); POOL_SIZE];
        Ok(ParticlePool { particles, mesh })
    }

    pub fn create_particle(&mut self, location: Point2<f32>, velocity: Point2<f32>, lifetime: u32) {
        for particle in self.particles.iter_mut() {
            if !particle.in_use() {
                particle.init(location, velocity, lifetime);
                break;
            }
        }
    }

    pub fn animate(&mut self) {
        for particle in self.particles.iter_mut() {
            if particle.in_use() {
                particle.animate();
            }
        }
    }

    pub fn draw(&self, context: &mut Context) -> GameResult<()> {
        for particle in self.particles.iter() {
            if particle.in_use() {
                particle.draw(context, &self.mesh)?;
            }
        }
        Ok(())
    }
}
