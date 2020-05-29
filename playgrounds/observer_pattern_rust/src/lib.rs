mod player;
mod score;

use player::Player;
use score::Score;
use std::sync::{Arc, Mutex};

pub trait Subject {
    fn add_observer(&mut self, observer: Observers);
    fn notify(&self, event: &'static str);
}

pub trait Observer {
    fn on_notify(&mut self, event: &'static str);
}

#[derive(Clone)]
pub enum Observers {
    Score(Arc<Mutex<Score>>),
    GameState(Arc<Mutex<GameState>>),
}

pub struct GameState {
    wrapped_score: Arc<Mutex<Score>>,
    player: Player,
}

impl GameState {
    pub fn new() -> GameState {
        let wrapped_score = Arc::new(Mutex::new(Score::new()));
        let mut player = Player::new();

        let observer = Observers::Score(wrapped_score.clone());

        player.add_observer(observer);

        GameState {
            wrapped_score,
            player,
        }
    }

    pub fn run(&self) {
        println!("Starting");

        {
            let score = self.wrapped_score.lock().unwrap();
            score.print_score();
        }
        self.player.update();
        {
            let score = self.wrapped_score.lock().unwrap();
            score.print_score();
        }
    }
}
