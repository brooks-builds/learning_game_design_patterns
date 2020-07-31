use ggez::conf::WindowMode;
use ggez::{event, Context, ContextBuilder};
use spatial_partition_rust::GameState;

fn main() {
    let (context, event_loop) =
        &mut match ContextBuilder::new("Spatial Partition in Rust and GGEZ", "Brooks Patton")
            .window_mode(WindowMode::default().dimensions(800.0, 800.0))
            .build()
        {
            Ok((context, event_loop)) => (context, event_loop),
            Err(error) => panic!(error),
        };

    let game_state = &mut GameState::new(context).unwrap();

    match event::run(context, event_loop, game_state) {
        Ok(_) => println!("Thanks for playing!"),
        Err(error) => println!("Error occurred: {}", error),
    };
}
