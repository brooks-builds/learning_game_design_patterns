use super::Observer;

pub struct Score(u64);

impl Score {
    pub fn new() -> Score {
        Score(0)
    }

    pub fn print_score(&self) {
        println!("Score: {}", self.0);
    }

    pub fn increment(&mut self) {
        self.0 += 1;
    }
}

impl Observer for Score {
    fn on_notify(&mut self, event: &'static str) {
        if event == "jumped over obstacle" {
            self.increment();
        }
    }
}
