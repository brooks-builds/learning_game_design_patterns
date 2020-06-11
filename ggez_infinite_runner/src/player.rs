use super::states::standing_state::StandingState;
use super::{Commands, Event, Observer, Obstacle, PossibleObserver, State, Subject};
use ggez::graphics::{DrawMode, Mesh, MeshBuilder, Rect, WHITE};
use ggez::nalgebra::{Point2, Vector2};
use ggez::{Context, GameResult};

pub struct Player<STATE: State> {
    location: Vector2<f32>,
    starting_location: Vector2<f32>,
    height: f32,
    width: f32,
    acceleration: Vector2<f32>,
    velocity: Vector2<f32>,
    jump_force: Vector2<f32>,
    is_jumping: bool,
    observers: Vec<PossibleObserver>,
    pub state: STATE,
}

// trying to make state dynamically generic
// maybe use a box - Vrixyz
impl<STATE: State> Player<STATE<dyn State>> {
    pub fn new(x: f32, y: f32) -> Player<STATE> {
        let acceleration = Vector2::new(0.0, 0.0);
        let velocity = Vector2::new(0.0, 0.0);
        let jump_force = Vector2::new(0.0, -9.0);
        let is_jumping = false;
        let location = Vector2::new(x, y);
        let starting_location = location;

        Player {
            location,
            starting_location,
            height: 50.0,
            width: 15.0,
            acceleration,
            velocity,
            jump_force,
            is_jumping,
            observers: vec![],
            state: StandingState::new(),
        }
    }

    pub fn create_mesh(&self, context: &mut Context) -> GameResult<Mesh> {
        let rect_bounds = Rect::new(0.0, 0.0, self.width, self.height);
        let mesh = MeshBuilder::new()
            .rectangle(DrawMode::fill(), rect_bounds, WHITE)
            .build(context)?;

        Ok(mesh)
    }

    pub fn get_location(&self) -> Point2<f32> {
        Point2::new(self.location.x, self.location.y)
    }

    pub fn reset_location(&mut self) {
        self.location = self.starting_location;
    }

    pub fn apply_force(&mut self, force: &Vector2<f32>) {
        self.acceleration += force;
    }

    pub fn run(&mut self, command: &Commands) {
        self.velocity += self.acceleration;
        self.location += self.velocity;
        self.acceleration *= 0.0;
        self.state.handle_input(command);
    }

    pub fn hit_ground(&mut self, arena_height: f32) {
        if self.location.y + self.height > arena_height {
            self.location.y = arena_height - self.height;
            self.velocity.y = 0.0;
            self.is_jumping = false;
        }
    }

    pub fn jump(&mut self) {
        if !self.is_jumping {
            let jump_force = self.jump_force.clone();
            self.apply_force(&jump_force);
            self.is_jumping = true;
        }
    }

    pub fn handle_running_into_obstacle(&mut self, obstacle: &Obstacle) {
        let obstacle_location = obstacle.get_location();
        let (obstacle_width, obstacle_height) = obstacle.get_size();

        if self.location.x < obstacle_location.x + obstacle_width
            && self.location.x + self.width > obstacle_location.x
            && self.location.y < obstacle_location.y + obstacle_height
            && self.location.y + self.height > obstacle_location.y
        {
            self.notify(Event::PlayerRanIntoObstacle);
        }
    }

    pub fn get_location_center(&self) -> Point2<f32> {
        Point2::new(
            self.location.x + self.width / 2.0,
            self.location.y + self.height / 2.0,
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
