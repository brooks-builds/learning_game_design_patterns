use dirty_flag_rust::GameState;
use ggez::{event, ContextBuilder};

fn main() {
    let (context, event_loop) =
        &mut match ContextBuilder::new("Learning Dirty Flag", "Brooks Patton").build() {
            Ok((context, event_loop)) => (context, event_loop),
            Err(error) => panic!(error),
        };

    let game_state = &mut GameState::new(context).unwrap();

    match event::run(context, event_loop, game_state) {
        Ok(_) => println!("Thanks for playing!"),
        Err(error) => println!("Error occurred: {}", error),
    };
}
