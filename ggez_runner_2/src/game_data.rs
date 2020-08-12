use super::{CustomError, Types};
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
    start_width: f32,
    end_width: f32,
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
            start_width: self.start_width,
            end_width: self.end_width,
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
    pub cell_size: f32,
    pub camera_width: f32,
    pub camera_height: f32,
    pub floor_y: f32,
    pub world_height: u8,
    pub camera_chase_x: f32,
    pub gravity_force: f32,
    pub player: PlayerData,
    pub level: Vec<Types>,
    pub start_width: f32,
    pub end_width: f32,
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
pub struct PlayerData {
    pub head_size: f32,
    pub body_height: f32,
    pub body_width: f32,
    pub start_x: f32,
    pub start_y: f32,
    pub speed: f32,
    pub jump_force: f32,
}
