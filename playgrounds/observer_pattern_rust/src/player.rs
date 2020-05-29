use super::{Observer, Score, Subject};
use std::sync::{Arc, Mutex};

pub struct Player {
    observers: Vec<Arc<Mutex<Score>>>,
}

impl Player {
    pub fn new() -> Player {
        Player { observers: vec![] }
    }

    pub fn update(&self) {
        // going to jump over an obstacle
        // need to send an event letting the score know that I jumped over the obstacle
        println!("I jumped over an obstacle");
        self.notify("jumped over obstacle");
    }
}

impl Subject for Player {
    fn add_observer(&mut self, score: Arc<Mutex<Score>>) {
        self.observers.push(score);
    }

    fn notify(&self, event: &'static str) {
        for wrapped_observer in self.observers.clone() {
            let mut observer = wrapped_observer.lock().unwrap();
            observer.on_notify(event);
        }
    }
}
