use ggez::graphics::{Color, DrawParam, Mesh, Scale};
use ggez::nalgebra::Point2;
use ggez::{graphics, Context, GameResult};

#[derive(Clone, Copy)]
pub struct Particle {
    location: Point2<f32>,
    frames_left: u32,
    velocity: Point2<f32>,
    red: f32,
    green: f32,
    blue: f32,
}

impl Particle {
    pub fn new() -> Particle {
        let location = Point2::new(0.0, 0.0);
        let frames_left = 0;
        let velocity = Point2::new(0.0, 0.0);
        let red = 1.0;
        let green = 0.0;
        let blue = 0.0;

        Particle {
            location,
            frames_left,
            velocity,
            red,
            green,
            blue,
        }
    }

    pub fn init(&mut self, location: Point2<f32>, velocity: Point2<f32>, lifetime: u32) {
        self.location = location;
        self.velocity = velocity;
        self.frames_left = lifetime;
        self.red = 1.0;
        self.green = 0.0;
        self.blue = 0.0;
    }

    pub fn draw(&self, context: &mut Context, mesh: &Mesh) -> GameResult<()> {
        let color = Color::new(self.red, self.green, self.blue, 1.0);
        graphics::draw(
            context,
            mesh,
            DrawParam::new().dest(self.location).color(color),
        )
    }

    pub fn animate(&mut self) {
        self.location.x += self.velocity.x;
        self.location.y += self.velocity.y;
        self.frames_left -= 1;
        if self.red > 0.0 && self.blue <= 0.0 {
            self.red -= 0.01;
            self.green += 0.01;
        }
        if self.green > 0.0 && self.red <= 0.0 {
            self.green -= 0.01;
            self.blue += 0.01;
        }
        if self.blue > 0.0 && self.green <= 0.0 {
            self.red += 0.01;
            self.blue -= 0.01;
        }
    }

    pub fn in_use(&self) -> bool {
        self.frames_left > 0
    }
}
