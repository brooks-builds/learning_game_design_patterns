mod player;
mod score;

use player::Player;
use score::Score;
use std::sync::{Arc, Mutex};

pub trait Subject {
    fn add_observer(&mut self, score: Arc<Mutex<Score>>);
    fn notify(&self, event: &'static str);
}

pub trait Observer {
    fn on_notify(&mut self, event: &'static str);
}

pub struct GameState {
    wrapped_score: Arc<Mutex<Score>>,
    player: Player,
}

impl GameState {
    pub fn new() -> GameState {
        let wrapped_score = Arc::new(Mutex::new(Score::new()));
        let mut player = Player::new();

        player.add_observer(wrapped_score.clone());

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
