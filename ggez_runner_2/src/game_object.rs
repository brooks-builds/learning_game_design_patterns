use super::physics::Physics;
use super::{draw, CustomError, Meshes, Types};
use draw::Draw;
use ggez::graphics::DrawParam;
use ggez::nalgebra::{Point2, Vector2};
use ggez::{graphics, Context};

#[derive(Debug)]
pub struct GameObject {
    pub id: u64,
    pub width: f32,
    pub height: f32,
    pub location: Vector2<f32>,
    pub my_type: Types,
    pub physics: Option<Box<dyn Physics>>,
    pub draw_system: Option<Box<dyn Draw>>,
}

impl GameObject {
    pub fn new(
        id: u64,
        width: f32,
        height: f32,
        location_x: f32,
        location_y: f32,
        my_type: Types,
        physics: Option<Box<dyn Physics>>,
        draw_system: Option<Box<dyn Draw>>,
    ) -> GameObject {
        GameObject {
            id,
            width: width,
            height: height,
            location: Vector2::new(location_x, location_y),
            my_type,
            physics,
            draw_system,
        }
    }

    pub fn draw(&mut self, meshes: &Meshes, context: &mut Context) -> Result<(), CustomError> {
        if let Some(draw_system) = &mut self.draw_system {
            draw_system.draw(
                &self.my_type,
                meshes,
                &self.physics,
                context,
                &self.location,
            );
            return Ok(());
        }
        let mesh = match self.my_type {
            Types::Floor => &meshes.floor,
            Types::Start => &meshes.start,
            Types::SpikeUp => &meshes.spike_up,
            Types::End => &meshes.end,
            _ => return Err(CustomError::CouldNotFindType),
        };

        match graphics::draw(
            context,
            mesh,
            DrawParam::new().dest(Point2::from(self.location)),
        ) {
            Ok(_) => Ok(()),
            Err(error) => Err(CustomError::GgezGameError(error)),
        }
    }

    pub fn update(&mut self, gravity_force: Vector2<f32>) {
        if let Some(ref mut physics) = self.physics {
            physics.update(&mut self.location, gravity_force);
        }
    }

    pub fn handle_collisions(&mut self, other_game_objects: Vec<&GameObject>) {
        if let Some(ref mut physics) = self.physics {
            physics.handle_collisions(
                &mut self.location,
                other_game_objects,
                &self.height,
                &self.width,
            );
        }
    }

    pub fn reset(&mut self, location_x: f32, location_y: f32, speed: f32) {
        self.location.x = location_x;
        self.location.y = location_y;
        if let Some(physics) = &mut self.physics {
            physics.reset(speed);
        }
    }
}

impl Clone for GameObject {
    fn clone(&self) -> Self {
        GameObject {
            id: self.id,
            width: self.width,
            height: self.height,
            location: self.location,
            my_type: self.my_type.clone(),
            physics: None,
            draw_system: None,
        }
    }
}
