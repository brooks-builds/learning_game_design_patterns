use super::{CustomError, States, Types};
use serde::{Deserialize, Serialize};
use std::fs::read_to_string;

#[derive(Serialize, Deserialize, Debug)]
pub struct RawGameData {
    cell_size: f32,
    camera_width: f32,
    camera_height: f32,
    floor_y: f32,
    world_height: u8,
    camera_chase_x: f32,
    gravity_force: f32,
    player: PlayerData,
    level: Vec<String>,
}

impl RawGameData {
    pub fn new_game_data(self) -> GameData {
        let level = self
            .level
            .iter()
            .map(|raw_type| self.convert_raw_type(raw_type))
            .collect();
        GameData {
            cell_size: self.cell_size,
            camera_width: self.camera_width,
            camera_height: self.camera_height,
            floor_y: self.floor_y,
            world_height: self.world_height,
            camera_chase_x: self.camera_chase_x,
            gravity_force: self.gravity_force,
            player: self.player,
            level,
        }
    }

    fn convert_raw_type(&self, raw_type: &str) -> Types {
        match raw_type {
            "player" => Types::Player,
            "floor" => Types::Floor,
            "start" => Types::Start,
            "spikeUp" => Types::SpikeUp,
            "end" => Types::End,
            "space" => Types::Space,
            _ => panic!("unsupported level type"),
        }
    }
}

pub struct GameData {
    cell_size: f32,
    camera_width: f32,
    camera_height: f32,
    floor_y: f32,
    world_height: u8,
    camera_chase_x: f32,
    gravity_force: f32,
    player: PlayerData,
    level: Vec<Types>,
}

pub fn load_from_file(file_name: &str) -> Result<GameData, CustomError> {
    let raw_game_data = match read_to_string(file_name) {
        Ok(raw_string) => raw_string,
        Err(error) => return Err(CustomError::IoError(error)),
    };

    let raw_game_data: RawGameData = match serde_json::from_str(&raw_game_data) {
        Ok(json) => json,
        Err(error) => {
            println!("error loading game data {}", error);
            return Err(CustomError::LoadGameDataError(error));
        }
    };
    let game_data = raw_game_data.new_game_data();

    Ok(game_data)
}

#[derive(Serialize, Deserialize, Debug)]
struct PlayerData {
    head_size: f32,
    body_height: f32,
    body_width: f32,
    start_x: f32,
    start_y: f32,
    speed: f32,
    jump_force: f32,
}
