use super::{Observer, Observers, Subject};

pub struct Player {
    observers: Vec<Observers>,
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
    fn add_observer(&mut self, observer: Observers) {
        self.observers.push(observer);
    }

    fn notify(&self, event: &'static str) {
        for wrapped_observer in self.observers.clone() {
            match wrapped_observer {
                Observers::Score(wrapped_score) => {
                    let mut observer = wrapped_score.lock().unwrap();
                    observer.on_notify(event);
                }
                _ => unimplemented!(),
            }
        }
    }
}
