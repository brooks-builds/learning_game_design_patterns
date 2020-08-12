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
    my_type: Types,
}

impl GameObject {
    pub fn new(
        id: u64,
        width: f32,
        height: f32,
        location_x: f32,
        location_y: f32,
        my_type: Types,
    ) -> GameObject {
        GameObject {
            id,
            width,
            height,
            location: Vector2::new(location_x, location_y),
            my_type,
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
}
