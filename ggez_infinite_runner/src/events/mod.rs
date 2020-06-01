use super::Score;
use std::sync::{Arc, Mutex};

pub type WrappedScore = Arc<Mutex<Score>>;

pub enum Event {
    PlayerJumpedOverObstacle,
}

#[derive(Clone)]
pub enum PossibleObserver {
    Score(WrappedScore),
}

pub trait Subject {
    fn add_observer(&mut self, observer: PossibleObserver);
    fn notify(&mut self, event: Event);
}

pub trait Observer {
    fn on_notify(&mut self, event: &Event);
}
