mod player_data;
mod player_states;

use super::{Commands, Event, Observer, Obstacle, PossibleObserver, Subject};
use ggez::graphics::{DrawMode, Mesh, MeshBuilder, Rect, WHITE};
use ggez::nalgebra::{Point2, Vector2};
use ggez::{Context, GameResult};
use player_data::PlayerData;
use player_states::PlayerStates;

pub struct Player {
    player_data: PlayerData,
    observers: Vec<PossibleObserver>,
    pub state: PlayerStates,
}

// trying to make state dynamically generic
// maybe use a box - Vrixyz
impl Player {
    pub fn new(x: f32, y: f32) -> Player {
        Player {
            player_data: PlayerData::new(x, y),
            observers: vec![],
            state: PlayerStates::InAir,
        }
    }

    pub fn create_mesh(&self, context: &mut Context) -> GameResult<Mesh> {
        let rect_bounds = Rect::new(0.0, 0.0, self.player_data.width, self.player_data.height);
        let mesh = MeshBuilder::new()
            .rectangle(DrawMode::fill(), rect_bounds, WHITE)
            .build(context)?;

        Ok(mesh)
    }

    pub fn get_location(&self) -> Point2<f32> {
        Point2::new(self.player_data.location.x, self.player_data.location.y)
    }

    pub fn reset(&mut self) {
        self.player_data.location = self.player_data.starting_location;
        self.state = PlayerStates::InAir;
        self.player_data.acceleration *= 0.0;
    }

    pub fn apply_force(&mut self, force: Vector2<f32>) {
        self.player_data.acceleration += force;
    }

    pub fn run(&mut self, command: &Commands, screen_height: f32) {
        if let Some(new_state) = self.state.handle_input(&command) {
            self.state = new_state;
        }
        match self.state {
            PlayerStates::Jumping => self.apply_force(self.player_data.jump_force),
            PlayerStates::InAir => {
                self.player_data.velocity += self.player_data.acceleration;
                self.player_data.location += self.player_data.velocity;
                self.player_data.acceleration *= 0.0;
            }
            PlayerStates::Standing => (),
        }

        if let Some(new_state) = self.state.update(&self.player_data, screen_height) {
            self.state = new_state;
        }
    }

    pub fn hit_ground(&mut self, arena_height: f32) {
        if self.player_data.location.y + self.player_data.height > arena_height {
            self.player_data.location.y = arena_height - self.player_data.height;
            self.player_data.velocity.y = 0.0;
            self.player_data.is_jumping = false;
        }
    }

    pub fn handle_running_into_obstacle(&mut self, obstacle: &Obstacle) {
        let obstacle_location = obstacle.get_location(0.0);
        let (obstacle_width, obstacle_height) = obstacle.get_size();

        if self.player_data.location.x < obstacle_location.x + obstacle_width
            && self.player_data.location.x + self.player_data.width > obstacle_location.x
            && self.player_data.location.y < obstacle_location.y + obstacle_height
            && self.player_data.location.y + self.player_data.height > obstacle_location.y
        {
            self.notify(Event::PlayerRanIntoObstacle);
        }
    }

    pub fn get_location_center(&self) -> Point2<f32> {
        Point2::new(
            self.player_data.location.x + self.player_data.width / 2.0,
            self.player_data.location.y + self.player_data.height / 2.0,
        )
    }
}

impl Subject for Player {
    fn add_observer(&mut self, observer: PossibleObserver) {
        self.observers.push(observer);
    }

    fn notify(&mut self, event: Event) {
        for observer in self.observers.clone() {
            if let PossibleObserver::GameState(wrapped_game_state) = observer {
                wrapped_game_state.lock().unwrap().on_notify(&event);
            }
        }
    }
}
