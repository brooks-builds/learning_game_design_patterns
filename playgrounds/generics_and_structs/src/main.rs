const STANDING: &'static str = "standing";

fn main() {
    let mut player = Player::new();
    let input = "jump".to_owned();
    player.handle_input(input);
    loop {
        dbg!("player location y: {}", player.player_data.location[1]);
        player.update();
        if player.state.get_name() == STANDING {
            break;
        }
    }
}

trait State {
    fn handle_input(&self, player_data: &mut PlayerData, input: String) -> Option<Box<dyn State>>;
    fn update(&self, player_data: &PlayerData) -> Option<Box<dyn State>>;
    fn get_name(&self) -> &'static str;
}

struct StandingState {
    name: &'static str,
}

impl StandingState {
    pub fn new() -> StandingState {
        println!("we are now standing");
        StandingState { name: "standing" }
    }
}

impl State for StandingState {
    fn handle_input(&self, player_data: &mut PlayerData, input: String) -> Option<Box<dyn State>> {
        if input == "jump" {
            Some(Box::new(JumpingState::new(player_data)))
        } else {
            None
        }
    }

    fn update(&self, player_data: &PlayerData) -> Option<Box<dyn State>> {
        None
    }
}

struct JumpingState {
    name: &'static str,
}

impl JumpingState {
    pub fn new(player_data: &mut PlayerData) -> JumpingState {
        println!("starting to jump");
        player_data.acceleration[1] -= 1.0;
        JumpingState { name: "jumping" }
    }
}

impl State for JumpingState {
    fn handle_input(&self, player_data: &mut PlayerData, input: String) -> Option<Box<dyn State>> {
        None
    }

    fn update(&self, player_data: &PlayerData) -> Option<Box<dyn State>> {
        if player_data.location[1] >= 0.0 {
            Some(Box::new(StandingState::new()))
        } else {
            None
        }
    }
}

#[derive(Debug)]
struct PlayerData {
    pub acceleration: [f32; 2],
    pub location: [f32; 2],
    pub velocity: [f32; 2],
}

impl PlayerData {
    pub fn new() -> PlayerData {
        PlayerData {
            acceleration: [0.0, 0.0],
            location: [0.0, 0.0],
            velocity: [0.0, 0.0],
        }
    }
}

struct Player {
    state: Box<dyn State>,
    pub player_data: PlayerData,
}

impl Player {
    pub fn new() -> Player {
        Player {
            state: Box::new(StandingState::new()),
            player_data: PlayerData::new(),
        }
    }

    pub fn handle_input(&mut self, input: String) {
        if let Some(new_state) = self.state.handle_input(&mut self.player_data, input) {
            self.state = new_state;
        }
    }

    pub fn update(&mut self) {
        self.player_data.acceleration[1] += 0.2; // apply gravity
        self.player_data.velocity[0] += self.player_data.acceleration[0];
        self.player_data.velocity[1] += self.player_data.acceleration[1];
        self.player_data.location[0] += self.player_data.velocity[0];
        self.player_data.location[1] += self.player_data.velocity[1];
        self.player_data.acceleration[0] = 0.0;
        self.player_data.acceleration[1] = 0.0;

        self.state.update(&self.player_data);
    }
}
