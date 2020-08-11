use ggez::{event, ContextBuilder};
use ggez_runner_2::GameState;

fn main() {
    let (context, event_loop) =
        &mut match ContextBuilder::new("GGEZ Runner 2", "Brooks Patton").build() {
            Ok((context, event_loop)) => (context, event_loop),
            Err(error) => panic!(error),
        };

    let game_state = &mut GameState::new();

    match event::run(context, event_loop, game_state) {
        Ok(_) => println!("Thanks for playing!"),
        Err(error) => println!("Error occurred: {}", error),
    };
}
