use ggez::{event, ContextBuilder};
use ggez_runner_2::{game_data, GameState};

fn main() {
    let (context, event_loop) =
        &mut match ContextBuilder::new("GGEZ Runner 2", "Brooks Patton").build() {
            Ok((context, event_loop)) => (context, event_loop),
            Err(error) => panic!(error),
        };

    let game_data = match game_data::load_from_file("game_data.json") {
        Err(error) => panic!(error),
        Ok(game_data) => game_data,
    };
    let game_state = &mut GameState::new(game_data);

    match event::run(context, event_loop, game_state) {
        Ok(_) => println!("Thanks for playing!"),
        Err(error) => println!("Error occurred: {}", error),
    };
}
