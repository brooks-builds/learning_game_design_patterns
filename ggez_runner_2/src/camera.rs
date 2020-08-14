use super::{CustomError, GameObject, Grid, Meshes};
use ggez::graphics::DrawParam;
use ggez::nalgebra::{Point2, Vector2};
use ggez::{graphics, Context};
use std::collections::HashMap;
use std::sync::mpsc::Receiver;

pub struct Camera {
    pub location: Vector2<f32>,
    initial_location: Vector2<f32>,
    width: f32,
    height: f32,
    player_moved_event_receive: Receiver<f32>,
}

impl Camera {
    pub fn new(
        location_x: f32,
        location_y: f32,
        width: f32,
        height: f32,
        player_moved_event_receive: Receiver<f32>,
    ) -> Camera {
        let location = Vector2::new(location_x, location_y);
        Camera {
            location,
            initial_location: location.clone(),
            width,
            height,
            player_moved_event_receive,
        }
    }

    pub fn draw(
        &mut self,
        grid: &Grid,
        meshes: &Meshes,
        context: &mut Context,
        game_objects: &mut HashMap<u64, GameObject>,
    ) -> Result<(), CustomError> {
        let mut game_objects = grid.query_mut(
            self.location.x,
            self.location.y,
            self.location.x + self.width,
            self.location.y + self.height,
            game_objects,
        );

        graphics::push_transform(
            context,
            Some(
                DrawParam::new()
                    .dest(Point2::from(self.location * -1.0))
                    .to_matrix(),
            ),
        );
        if let Err(error) = graphics::apply_transformations(context) {
            return Err(CustomError::GgezGameError(error));
        }
        for game_object in game_objects {
            game_object.draw(meshes, context)?;
        }
        graphics::pop_transform(context);

        Ok(())
    }

    pub fn update(&mut self) {
        if let Ok(player_moved_x) = self.player_moved_event_receive.try_recv() {
            self.location.x += player_moved_x
        }
    }

    pub fn reset(&mut self) {
        self.location = self.initial_location.clone();
    }
}
