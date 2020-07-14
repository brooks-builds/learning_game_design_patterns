trait Input {
    fn update(&self, velocity: &mut (f32, f32));
}

trait Graphics {
    fn draw(&self, sprite: Sprites);
    fn update(&self, velocity: &(f32, f32));
}

trait Physics {
    fn update(&self, x: &mut f32, y: &mut f32, velocity: (f32, f32), world: &mut World);
}

enum Direction {
    Left,
    _Right,
    _NoDirection,
}

#[derive(Copy, Clone)]
enum Sprites {
    BjornStand,
    BjornRunRight,
    BjornRunLeft,
}

struct GameObject {
    velocity: (f32, f32),
    x: f32,
    y: f32,
    input: Box<dyn Input>,
    physics: Box<dyn Physics>,
    graphics: Box<dyn Graphics>,
}

impl GameObject {
    pub fn new(
        x: f32,
        y: f32,
        input: Box<dyn Input>,
        physics: Box<dyn Physics>,
        graphics: Box<dyn Graphics>,
    ) -> GameObject {
        GameObject {
            velocity: (0.0, 0.0),
            x,
            y,
            input,
            physics,
            graphics,
        }
    }

    pub fn update(&mut self, world: &mut World) {
        self.input.update(&mut self.velocity);
        self.physics
            .update(&mut self.x, &mut self.y, self.velocity, world);
        self.graphics.update(&self.velocity);
    }

    pub fn print_details(&self) {
        println!("x: {}", self.x);
        println!("velocity x: {}", self.velocity.0);
    }
}

struct World {}

impl World {
    pub fn resolve_collision(&mut self, _volume: f32, _x: f32, _y: f32, _velocity: (f32, f32)) {}
}

struct PlayerGraphics {
    standing_sprite: Sprites,
    left_sprite: Sprites,
    right_sprite: Sprites,
}

impl PlayerGraphics {
    pub fn new() -> PlayerGraphics {
        PlayerGraphics {
            standing_sprite: Sprites::BjornStand,
            left_sprite: Sprites::BjornRunLeft,
            right_sprite: Sprites::BjornRunRight,
        }
    }
}

impl Graphics for PlayerGraphics {
    fn draw(&self, _sprite: Sprites) {}

    fn update(&self, velocity: &(f32, f32)) {
        let sprite = if velocity.0 < 0.0 {
            self.left_sprite
        } else if velocity.0 > 0.0 {
            self.right_sprite
        } else {
            self.standing_sprite
        };
        self.draw(sprite);
    }
}

struct PlayerInput {
    walk_acceleration: f32,
}

impl PlayerInput {
    pub fn new(walk_acceleration: f32) -> PlayerInput {
        PlayerInput { walk_acceleration }
    }

    fn get_controller_direction(&self) -> Direction {
        Direction::Left
    }
}

impl Input for PlayerInput {
    fn update(&self, velocity: &mut (f32, f32)) {
        match self.get_controller_direction() {
            Direction::Left => velocity.0 -= self.walk_acceleration,
            Direction::_Right => velocity.0 += self.walk_acceleration,
            Direction::_NoDirection => velocity.0 = 0.0,
        };
    }
}
struct DemoInput {
    walk_acceleration: f32,
}

impl DemoInput {
    pub fn new() -> DemoInput {
        DemoInput {
            walk_acceleration: 1.0,
        }
    }
}

impl Input for DemoInput {
    fn update(&self, velocity: &mut (f32, f32)) {
        // ai code to control the player
        velocity.0 += self.walk_acceleration;
    }
}

struct PlayerPhysics {
    volume: f32,
}

impl PlayerPhysics {
    pub fn new() -> PlayerPhysics {
        PlayerPhysics { volume: 1.0 }
    }
}

impl Physics for PlayerPhysics {
    fn update(&self, x: &mut f32, y: &mut f32, velocity: (f32, f32), world: &mut World) {
        *x += velocity.0;
        world.resolve_collision(self.volume, *x, *y, velocity);
    }
}

fn main() {
    // game setup
    let _bjorn_input = Box::new(PlayerInput::new(1.0));
    let demo_input = Box::new(DemoInput::new());
    let bjorn_physics = Box::new(PlayerPhysics::new());
    let bjorn_graphics = Box::new(PlayerGraphics::new());
    let mut bjorn = GameObject::new(0.0, 250.0, demo_input, bjorn_physics, bjorn_graphics);
    let mut world = World {};

    // let dragon_input = Box::new(DragonInput::new());
    // let dragon_physics = Box::new(DragonPhysics::new());
    // let dragon_graphics = Box::new(DragonGraphics::new());
    // let mut dragon = GameObject::new(0.0, 0.0, dragon_input, dragon_physics, dragon_graphics);
    // update loop
    bjorn.print_details();
    bjorn.update(&mut world);
    bjorn.print_details();
}
