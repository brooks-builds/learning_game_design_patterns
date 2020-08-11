use ggez::conf::WindowMode;
use ggez::{event, ContextBuilder};
use ggez_runner_2::{game_data, GameState};

fn main() {
    let game_data = match game_data::load_from_file("game_data.json") {
        Err(error) => panic!(error),
        Ok(game_data) => game_data,
    };

    let (context, event_loop) = &mut match ContextBuilder::new("GGEZ Runner 2", "Brooks Patton")
        .window_mode(
            WindowMode::default().dimensions(game_data.camera_width, game_data.camera_height),
        )
        .build()
    {
        Ok((context, event_loop)) => (context, event_loop),
        Err(error) => panic!(error),
    };

    let game_state = &mut match GameState::new(game_data, context) {
        Ok(game_state) => game_state,
        Err(error) => panic!("Error loading game state {}", error),
    };

    match event::run(context, event_loop, game_state) {
        Ok(_) => println!("Thanks for playing!"),
        Err(error) => println!("Error occurred: {}", error),
    };
}
