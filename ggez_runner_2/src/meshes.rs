use super::GameData;
use ggez::graphics::{Color, DrawMode, Image, Mesh, MeshBuilder, Rect, WHITE};
use ggez::nalgebra::Point2;
use ggez::{Context, GameResult};

pub struct Meshes {
    pub floor: Mesh,
    pub cell: Mesh,
    pub start: Mesh,
    pub player: Mesh,
    pub spike_up: Mesh,
    pub end: Mesh,
    pub player_standing: Image,
    pub player_walk_1: Image,
    pub player_walk_2: Image,
    pub player_jumping: Image,
    pub player_dead: Image,
}

impl Meshes {
    pub fn new(context: &mut Context, game_data: &GameData) -> GameResult<Meshes> {
        Ok(Meshes {
            floor: Self::create_floor_mesh(context, game_data.cell_size, game_data.cell_size)?,
            cell: Self::create_cell_mesh(context, game_data.cell_size, game_data.cell_size)?,
            start: Self::create_start_mesh(
                context,
                game_data.start_width,
                game_data.cell_size,
                game_data.cell_size,
            )?,
            player: Self::create_player_mesh(
                context,
                game_data.player.body_width,
                game_data.player.body_height,
                game_data.player.head_size,
            )?,
            spike_up: Self::create_spike_mesh(context, game_data.cell_size, game_data.cell_size)?,
            end: Self::create_end_mesh(context, game_data.end_width, game_data.cell_size)?,
            player_standing: Image::new(context, "/bunny2_stand.png")?,
            player_walk_1: Image::new(context, "/bunny2_walk1.png")?,
            player_walk_2: Image::new(context, "/bunny2_walk2.png")?,
            player_jumping: Image::new(context, "/bunny2_jump.png")?,
            player_dead: Image::new(context, "/bunny2_hurt.png")?,
        })
    }

    fn create_floor_mesh(context: &mut Context, width: f32, height: f32) -> GameResult<Mesh> {
        MeshBuilder::new()
            .rectangle(
                DrawMode::fill(),
                Rect::new(0.0, 0.0, width, height),
                Color::from_rgb(102, 51, 0),
            )
            .build(context)
    }

    fn create_cell_mesh(context: &mut Context, width: f32, height: f32) -> GameResult<Mesh> {
        MeshBuilder::new()
            .rectangle(
                DrawMode::stroke(1.0),
                Rect::new(0.0, 0.0, width, height),
                WHITE,
            )
            .build(context)
    }

    fn create_start_mesh(
        context: &mut Context,
        width: f32,
        height: f32,
        world_width: f32,
    ) -> GameResult<Mesh> {
        MeshBuilder::new()
            .rectangle(
                DrawMode::fill(),
                Rect::new(world_width - width, 0.0, width, height),
                Color::new(0.0, 1.0, 0.0, 1.0),
            )
            .build(context)
    }

    fn create_end_mesh(context: &mut Context, width: f32, height: f32) -> GameResult<Mesh> {
        MeshBuilder::new()
            .rectangle(
                DrawMode::fill(),
                Rect::new(0.0, 0.0, width, height),
                Color::new(1.0, 0.0, 0.0, 1.0),
            )
            .build(context)
    }

    fn create_player_mesh(
        context: &mut Context,
        width: f32,
        height: f32,
        head_radius: f32,
    ) -> GameResult<Mesh> {
        MeshBuilder::new()
            .circle(
                DrawMode::fill(),
                Point2::new(width / 2.0, head_radius / 2.0),
                head_radius,
                0.1,
                WHITE,
            )
            .rectangle(DrawMode::fill(), Rect::new(0.0, 0.0, width, height), WHITE)
            .build(context)
    }

    fn create_spike_mesh(context: &mut Context, width: f32, height: f32) -> GameResult<Mesh> {
        MeshBuilder::new()
            .triangles(
                &[
                    Point2::new(width / 2.0, 0.0),
                    Point2::new(width, height),
                    Point2::new(0.0, height),
                ],
                Color::from_rgb(81, 81, 82),
            )?
            .build(context)
    }
}
