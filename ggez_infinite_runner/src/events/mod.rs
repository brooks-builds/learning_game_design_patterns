use super::{GameState, Score};
use std::sync::{Arc, Mutex};

pub type WrappedScore = Arc<Mutex<Score>>;
pub type WrappedGameState = Arc<Mutex<GameState>>;

pub enum Event {
    PlayerJumpedOverObstacle,
    PlayerRanIntoObstacle,
}

#[derive(Clone)]
pub enum PossibleObserver {
    Score(WrappedScore),
    GameState(WrappedGameState),
}

pub trait Subject {
    fn add_observer(&mut self, observer: PossibleObserver);
    fn notify(&mut self, event: Event);
}

pub trait Observer {
    fn on_notify(&mut self, event: &Event);
}
