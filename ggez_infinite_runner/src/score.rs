use super::{EventSystem, Events};

pub struct Score(u64);

impl Score {
    pub fn new(jumped_over_obstacle_event: &EventSystem) -> Score {
        let score = Score(0);

        score
    }

    pub fn increment(&mut self) {
        self.0 += 1;
    }

    pub fn get(&self) -> u64 {
        self.0
    }

    pub fn reset(&mut self) {
        self.0 = 0;
    }

    pub fn onNotify(&self, event: &Events) {}

    fn register_for_event(&self, event_system: &EventSystem, event: Events) {
        event_system.addObserver(observer: Score)
    }
}
