use super::Score;

pub enum Events {
    JUMPED_OVER_OBSTACLE,
}

pub struct EventSystem<'a> {
    observers: Vec<&'a Score>,
}

impl<'a> EventSystem<'a> {
    pub fn new() -> EventSystem<'a> {
        EventSystem { observers: vec![] }
    }

    pub fn addObserver(&mut self, observer: &'a Score) {
        self.observers.push(&observer);
    }

    pub fn notify(&self, event: Events) {
        for observer in &self.observers {
            observer.onNotify(&event);
        }
    }
}
