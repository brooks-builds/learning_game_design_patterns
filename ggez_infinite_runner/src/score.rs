pub struct Score (u64);

impl Score {
    pub fn new() -> Score {
        Score(0)
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