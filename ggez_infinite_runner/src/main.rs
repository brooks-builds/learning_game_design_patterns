// extern crate ggez_infinite_runner;

use ggez::conf::{WindowMode, WindowSetup};
use ggez::ContextBuilder;
use ggez_infinite_runner::game_loop::run;
use ggez_infinite_runner::MyGame;

fn main() {
    // Make a Context.
    let window_mode = WindowMode::default().dimensions(1850.0, 1000.0);
    let window_setup = WindowSetup::default().vsync(false);

    let (mut context, mut event_loop) = ContextBuilder::new("Infinite Runner", "Brookzerker")
        .window_mode(window_mode)
        .window_setup(window_setup)
        .build()
        .expect("aieee, could not create ggez context!");

    // Create an instance of your event handler.
    // Usually, you should provide it with the Context object to
    // use when setting your game up.
    let mut my_game = MyGame::new(&mut context).unwrap();

    // Run!
    match run(&mut context, &mut event_loop, &mut my_game) {
        Ok(_) => println!("Exited cleanly."),
        Err(e) => println!("Error occured: {}", e),
    }
}
