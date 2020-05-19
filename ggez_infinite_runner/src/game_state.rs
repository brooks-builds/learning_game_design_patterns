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