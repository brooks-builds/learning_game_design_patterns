use super::{Event, Observer, Player, PossibleObserver, Subject};
use ggez::graphics::{DrawMode, Mesh, MeshBuilder, WHITE};
use ggez::nalgebra::{Point2, Vector2};
use ggez::{Context, GameResult};

pub struct Obstacle {
    location: Vector2<f32>,
    location_to_reset_to: Vector2<f32>,
    starting_location: Vector2<f32>,
    velocity: Vector2<f32>,
    width: f32,
    height: f32,
    speed_increase_rate: f32,
    observers: Vec<PossibleObserver>,
}

impl Obstacle {
    pub fn new(location_x: f32, location_y: f32, size: f32, arena_width: f32) -> Obstacle {
        let location = Vector2::new(location_x, location_y);
        let location_to_reset_to = Vector2::new(arena_width + size, location_y);
        let starting_location = location;
        let velocity = Vector2::new(-5.0, 0.0);
        let width = size;
        let height = size;
        let speed_increase_rate = -0.1;
        let observers = vec![];

        Obstacle {
            location,
            location_to_reset_to,
            starting_location,
            velocity,
            width,
            height,
            speed_increase_rate,
            observers,
        }
    }

    pub fn create_mesh(&self, context: &mut Context) -> GameResult<Mesh> {
        let triangle_points = [
            Point2::new(0.0, 0.0),
            Point2::new(self.width / 2.0, self.height),
            Point2::new(self.width, 0.0),
        ];
        MeshBuilder::new()
            .polyline(DrawMode::fill(), &triangle_points, WHITE)?
            .build(context)
    }

    pub fn get_location(&self) -> Point2<f32> {
        Point2::new(self.location.x, self.location.y)
    }

    pub fn reset_to_start(&mut self) {
        self.location = self.starting_location;
    }

    pub fn run(&mut self, player: &Player) {
        let location_x_before = self.location.x;
        let player_location_center = player.get_location_center();

        self.location += self.velocity;

        if location_x_before > player_location_center.x
            && self.location.x < player_location_center.x
        {
            // player jumped over obstacle
            self.notify(Event::PlayerJumpedOverObstacle);
        }
    }

    pub fn reset_location(&mut self) {
        self.location = self.location_to_reset_to;
    }

    pub fn is_offscreen(&self) -> bool {
        self.location.x + self.width < 0.0
    }

    pub fn increase_speed(&mut self) {
        self.velocity.x += self.speed_increase_rate;
    }

    pub fn get_size(&self) -> (f32, f32) {
        (self.width, self.height)
    }
}

impl Subject for Obstacle {
    fn add_observer(&mut self, observer: PossibleObserver) {
        self.observers.push(observer)
    }

    fn notify(&mut self, event: Event) {
        for possible_observer in self.observers.clone() {
            if let PossibleObserver::Score(observer) = possible_observer {
                observer.lock().unwrap().on_notify(&event);
            }
        }
    }
}
