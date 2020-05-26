use super::TreeModel;
use ggez::graphics::{Color, DrawParam};
use ggez::nalgebra::{Point2, Vector2};
use ggez::{graphics, Context, GameResult};
use rand::prelude::*;

#[derive(Copy, Clone)]
pub struct Tree {
    location: Vector2<f32>,
    branch_color: Color,
    leaf_count: u8,
}

impl Tree {
    pub fn new(
        arena_width: f32,
        arena_height: f32,
        tree_model: &TreeModel,
        rng: &mut ThreadRng,
    ) -> Tree {
        let leaf_count = rng.gen_range(1, 7);
        let location = Vector2::new(
            arena_width + tree_model.get_leaves_length(),
            arena_height - tree_model.get_trunk_height(),
        );
        let branch_color = Color::new(0.0, rng.gen_range(50.0, 255.0), 0.0, 1.0);

        Tree {
            location,
            branch_color,
            leaf_count,
        }
    }

    pub fn draw(
        &self,
        context: &mut Context,
        tree_model: &TreeModel,
        rng: &mut ThreadRng,
    ) -> GameResult<()> {
        graphics::draw(
            context,
            tree_model.get_trunk_mesh(),
            DrawParam::default().dest(Point2::new(self.location.x, self.location.y)),
        )?;

        for tree_count in 0..self.leaf_count {
            graphics::draw(
                context,
                tree_model.get_leaves_mesh(),
                DrawParam::default()
                    .dest(Point2::new(
                        self.location.x,
                        self.location.y + 50.0 * tree_count as f32,
                    ))
                    .color(self.branch_color),
            )?;
        }

        Ok(())
    }

    pub fn update(&mut self, tree_model: &TreeModel) {
        self.location += tree_model.get_velocity();
    }

    pub fn is_off_screen(&self, tree_model: &TreeModel) -> bool {
        self.location.x + tree_model.get_trunk_width() + tree_model.get_leaves_length() < 0.0
    }
}
