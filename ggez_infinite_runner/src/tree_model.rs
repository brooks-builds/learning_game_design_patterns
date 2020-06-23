use ggez::graphics::{Color, DrawMode, Mesh, MeshBuilder, Rect, WHITE};
use ggez::nalgebra::{Point2, Vector2};
use ggez::{Context, GameResult};

#[derive(Clone, Copy)]
pub enum TreeType {
    Normal,
    Tall,
}

pub struct TreeModel {
    trunk_width: f32,
    trunk_height: f32,
    tall_trunk_height: f32,
    velocity: Vector2<f32>,
    tall_velocity: Vector2<f32>,
    trunk_mesh: Mesh,
    tall_trunk_mesh: Mesh,
    leaves_mesh: Mesh,
    leaves_length: f32,
    leaf_count: usize,
    branch_color: Color,
}

impl TreeModel {
    pub fn new(context: &mut Context) -> GameResult<TreeModel> {
        let trunk_width = 25.0;
        let trunk_height = 350.0;
        let tall_trunk_height = 550.0;
        let leaves_length = 250.0;
        let velocity = Vector2::new(-1.0, 0.0);
        let tall_velocity = Vector2::new(-2.0, 0.0);
        let trunk_mesh = MeshBuilder::new()
            .rectangle(
                DrawMode::fill(),
                Rect::new(0.0, 0.0, trunk_width, trunk_height),
                Color::from_rgb(210, 105, 30),
            )
            .build(context)?;
        let tall_trunk_mesh = MeshBuilder::new()
            .rectangle(
                DrawMode::fill(),
                Rect::new(0.0, 0.0, trunk_width, tall_trunk_height),
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
            tall_trunk_height,
            velocity,
            tall_velocity,
            trunk_mesh,
            tall_trunk_mesh,
            leaves_mesh,
            leaves_length,
            leaf_count: 4,
            branch_color: Color::from_rgb(0, 128, 0),
        })
    }

    pub fn get_leaves_length(&self) -> f32 {
        self.leaves_length
    }

    pub fn get_trunk_height(&self, tree_type: &TreeType) -> f32 {
        match tree_type {
            TreeType::Normal => self.trunk_height,
            TreeType::Tall => self.tall_trunk_height,
        }
    }

    pub fn get_trunk_width(&self) -> f32 {
        self.trunk_width
    }

    pub fn get_trunk_mesh(&self, tree_type: &TreeType) -> &Mesh {
        match tree_type {
            TreeType::Normal => &self.trunk_mesh,
            TreeType::Tall => &self.tall_trunk_mesh,
        }
    }

    pub fn get_leaves_mesh(&self) -> &Mesh {
        &self.leaves_mesh
    }

    pub fn get_velocity(&self, tree_type: &TreeType) -> &Vector2<f32> {
        match tree_type {
            TreeType::Normal => &self.velocity,
            TreeType::Tall => &self.tall_velocity,
        }
    }

    pub fn get_leaf_count(&self) -> usize {
        self.leaf_count
    }

    pub fn get_branch_color(&self) -> Color {
        self.branch_color
    }
}
