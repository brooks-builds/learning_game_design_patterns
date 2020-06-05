use super::{Event, Observer};

#[derive(Clone)]
pub enum GameState {
    Playing,
    GameOver,
    Help,
}

impl GameState {
    pub fn playing(&mut self) {
        *self = Self::Playing;
    }
}

impl Observer for GameState {
    fn on_notify(&mut self, event: &Event) {
        // if let Event::PlayerRanIntoObstacle = event {
        //     *self = Self::GameOver;
        // }
    }
}
