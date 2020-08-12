use super::physics::Physics;
use super::{CustomError, Meshes, Types};
use ggez::graphics::DrawParam;
use ggez::nalgebra::{Point2, Vector2};
use ggez::{graphics, Context};

#[derive(Debug)]
pub struct GameObject {
    pub id: u64,
    width: f32,
    height: f32,
    pub location: Vector2<f32>,
    pub my_type: Types,
    pub physics: Option<Box<dyn Physics>>,
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
    ) -> GameObject {
        GameObject {
            id,
            width: width,
            height: height,
            location: Vector2::new(location_x, location_y),
            my_type,
            physics,
        }
    }

    pub fn draw(&self, meshes: &Meshes, context: &mut Context) -> Result<(), CustomError> {
        let mesh = match self.my_type {
            Types::Floor => &meshes.floor,
            Types::Start => &meshes.start,
            Types::Player => &meshes.player,
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

    pub fn update(&mut self) {
        if let Some(ref mut physics) = self.physics {
            physics.update(&mut self.location);
        }
    }

    pub fn handle_collisions(&mut self, other_game_objects: Vec<&GameObject>) {
        if let Some(ref mut physics) = self.physics {
            physics.handle_collisions(other_game_objects);
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
        }
    }
}
