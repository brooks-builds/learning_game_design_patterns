use ggez::graphics::{Color, DrawMode, Mesh, MeshBuilder, Rect, WHITE};
use ggez::nalgebra::{Point2, Vector2};
use ggez::{Context, GameResult};

pub struct TreeModel {
    trunk_width: f32,
    trunk_height: f32,
    velocity: Vector2<f32>,
    trunk_mesh: Mesh,
    leaves_mesh: Mesh,
    leaves_length: f32,
}

impl TreeModel {
    pub fn new(context: &mut Context) -> GameResult<TreeModel> {
        let trunk_width = 25.0;
        let trunk_height = 350.0;
        let leaves_length = 250.0;
        let velocity = Vector2::new(-0.5, 0.0);
        let trunk_mesh = MeshBuilder::new()
            .rectangle(
                DrawMode::fill(),
                Rect::new(0.0, 0.0, trunk_width, trunk_height),
                Color::from_rgb(210, 105, 30),
            )
            .build(context)?;
        let leaves_points = [
            Point2::new(trunk_width / 2.0, -(leaves_length / 2.0)),
            Point2::new(trunk_width / 2.0 - leaves_length / 2.0, leaves_length / 2.0),
            Point2::new(trunk_width / 2.0 + leaves_length / 2.0, leaves_length / 2.0),
        ];
        let leaves_mesh = MeshBuilder::new()
            .triangles(&leaves_points, WHITE)?
            .build(context)?;

        Ok(TreeModel {
            trunk_width,
            trunk_height,
            velocity,
            trunk_mesh,
            leaves_mesh,
            leaves_length,
        })
    }

    pub fn get_leaves_length(&self) -> f32 {
        self.leaves_length
    }

    pub fn get_trunk_height(&self) -> f32 {
        self.trunk_height
    }

    pub fn get_trunk_width(&self) -> f32 {
        self.trunk_width
    }

    pub fn get_trunk_mesh(&self) -> &Mesh {
        &self.trunk_mesh
    }

    pub fn get_leaves_mesh(&self) -> &Mesh {
        &self.leaves_mesh
    }

    pub fn get_velocity(&self) -> &Vector2<f32> {
        &self.velocity
    }
}
