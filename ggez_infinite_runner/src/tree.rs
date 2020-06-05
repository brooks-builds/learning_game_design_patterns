use super::{TreeModel, TreeType};
use ggez::graphics::{Color, DrawParam};
use ggez::nalgebra::{Point2, Vector2};
use ggez::{graphics, Context, GameResult};
use rand::prelude::*;

#[derive(Copy, Clone)]
pub struct Tree {
    location: Vector2<f32>,
    pub tree_type: TreeType,
}

impl Tree {
    pub fn new(
        arena_width: f32,
        arena_height: f32,
        tree_model: &TreeModel,
        tree_type: TreeType,
    ) -> Tree {
        let location = Vector2::new(
            arena_width + tree_model.get_leaves_length(),
            arena_height - tree_model.get_trunk_height(&tree_type),
        );

        Tree {
            location,
            tree_type,
        }
    }

    pub fn draw(
        &self,
        context: &mut Context,
        tree_model: &TreeModel,
        _rng: &mut ThreadRng,
    ) -> GameResult<()> {
        graphics::draw(
            context,
            tree_model.get_trunk_mesh(&self.tree_type),
            DrawParam::default().dest(Point2::new(self.location.x, self.location.y)),
        )?;

        for tree_count in 0..tree_model.get_leaf_count() {
            graphics::draw(
                context,
                tree_model.get_leaves_mesh(),
                DrawParam::default()
                    .dest(Point2::new(
                        self.location.x,
                        self.location.y + 50.0 * tree_count as f32,
                    ))
                    .color(tree_model.get_branch_color()),
            )?;
        }

        Ok(())
    }

    pub fn update(&mut self, tree_model: &TreeModel) {
        self.location += tree_model.get_velocity(&self.tree_type);
    }

    pub fn is_off_screen(&self, tree_model: &TreeModel) -> bool {
        self.location.x + tree_model.get_trunk_width() + tree_model.get_leaves_length() < 0.0
    }
}
