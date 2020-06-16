use super::Commands;
use super::PlayerData;

pub enum PlayerStates {
    Standing,
    Jumping,
    InAir,
}

impl PlayerStates {
    pub fn handle_input(&self, command: &Commands) -> Option<PlayerStates> {
        if let PlayerStates::Standing = self {
            if let Commands::Jump = command {
                return Some(PlayerStates::Jumping);
            }
        }

        None
    }

    pub fn update(&self, player_data: &PlayerData, screen_height: f32) -> Option<PlayerStates> {
        match self {
            PlayerStates::Jumping => Some(PlayerStates::InAir),
            PlayerStates::InAir => {
                if player_data.location.y + player_data.height >= screen_height {
                    Some(PlayerStates::Standing)
                } else {
                    None
                }
            }
            PlayerStates::Standing => None,
        }
    }
}
