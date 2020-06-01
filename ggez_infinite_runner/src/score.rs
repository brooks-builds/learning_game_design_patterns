use super::{Event, Observer};

pub struct Score(u64);

impl Score {
    pub fn new() -> Score {
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
}

impl Observer for Score {
    fn on_notify(&mut self, event: &Event) {
        if let Event::PlayerJumpedOverObstacle = event {
            self.increment();
        }
    }
}
