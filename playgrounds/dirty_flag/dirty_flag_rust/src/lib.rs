use ggez::event::EventHandler;
use ggez::graphics::{Color, DrawMode, DrawParam, Mesh, MeshBuilder, Rect, BLACK, WHITE};
use ggez::nalgebra::Point2;
use ggez::{graphics, Context, GameResult};

const GRAVITY: f32 = 0.5;

pub struct GameState {
    scene_graph: GraphNode,
}

impl GameState {
    pub fn new(context: &mut Context) -> GameResult<GameState> {
        let (screen_width, screen_height) = graphics::drawable_size(context);
        let rocket_body_mesh = MeshBuilder::new()
            .rectangle(DrawMode::fill(), Rect::new(0.0, 0.0, 10.0, 50.0), WHITE)
            .build(context)?;
        let flame_mesh = MeshBuilder::new()
            .triangles(
                &[
                    Point2::new(0.0, 50.0),
                    Point2::new(10.0, 50.0),
                    Point2::new(5.0, 60.0),
                ],
                Color::new(1.0, 0.0, 0.0, 1.0),
            )?
            .build(context)?;
        let cap_mesh = MeshBuilder::new()
            .triangles(
                &[
                    Point2::new(0.0, 0.0),
                    Point2::new(5.0, -3.0),
                    Point2::new(10.0, 0.0),
                ],
                WHITE,
            )?
            .build(context)?;
        let cap = GraphNode::new(
            Some(cap_mesh),
            Box::new(CapUpdate {}),
            Point2::new(0.0, 0.0),
            false,
        );
        let rocket_engine = GraphNode::new(
            Some(flame_mesh),
            Box::new(RocketEngineUpdate {}),
            Point2::new(0.0, 0.0),
            false,
        );
        let mut rocket_body = GraphNode::new(
            Some(rocket_body_mesh),
            Box::new(RocketUpdate::new()),
            Point2::new(screen_width / 2.0 - 5.0, screen_height - 50.0),
            true,
        );
        rocket_body.children.push(rocket_engine);
        rocket_body.children.push(cap);
        let mut scene_graph =
            GraphNode::new(None, Box::new(SceneUpdate {}), Point2::new(0.0, 0.0), false);
        scene_graph.children.push(rocket_body);
        Ok(GameState { scene_graph })
    }
}

impl EventHandler for GameState {
    fn update(&mut self, _context: &mut Context) -> GameResult {
        self.scene_graph.run();
        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> GameResult {
        graphics::clear(context, BLACK);

        self.scene_graph.render(&Transform::origin(), context)?;

        graphics::present(context)
    }
}

#[derive(Clone, Copy)]
struct Transform {
    pub location: Point2<f32>,
    velocity: Point2<f32>,
    acceleration: Point2<f32>,
}

impl Transform {
    pub fn origin() -> Transform {
        Transform {
            location: Point2::new(0.0, 0.0),
            velocity: Point2::new(0.0, 0.0),
            acceleration: Point2::new(0.0, 0.0),
        }
    }

    pub fn combine(&mut self, other: &Transform) {
        self.location.x += other.location.x;
        self.location.y += other.location.y;
    }

    pub fn apply_force(&mut self, force: Point2<f32>) {
        self.acceleration.x += force.x;
        self.acceleration.y += force.y;
    }
}

struct GraphNode {
    mesh: Option<Mesh>,
    pub local: Transform,
    pub children: Vec<GraphNode>,
    update: Box<dyn Update>,
    is_affected_by_gravity: bool,
}

impl GraphNode {
    pub fn new(
        mesh: Option<Mesh>,
        update: Box<dyn Update>,
        location: Point2<f32>,
        is_affected_by_gravity: bool,
    ) -> GraphNode {
        let mut local = Transform::origin();
        local.location = location;
        GraphNode {
            mesh,
            local,
            children: vec![],
            update,
            is_affected_by_gravity,
        }
    }

    pub fn render(&mut self, parent_world: &Transform, context: &mut Context) -> GameResult<()> {
        let mut world = self.local.clone();
        world.combine(parent_world);
        if let Some(mesh) = &self.mesh {
            render_mesh(&mesh, &world, context)?;
        }
        for child in &mut self.children {
            child.render(&world, context)?;
        }
        Ok(())
    }

    pub fn run(&mut self) {
        self.update.update(&mut self.local);
        for child in &mut self.children {
            self.local.apply_force(Point2::new(0.0, GRAVITY));
            child.run();
        }
    }
}

fn render_mesh(mesh: &Mesh, transform: &Transform, context: &mut Context) -> GameResult<()> {
    graphics::draw(context, mesh, DrawParam::new().dest(transform.location))
}

trait Update {
    fn update(&mut self, transform: &mut Transform);
}

struct SceneUpdate {}

impl Update for SceneUpdate {
    fn update(&mut self, _transform: &mut Transform) {}
}

struct RocketUpdate {
    fuel: u8,
    upward_movement_force: Point2<f32>,
}

impl RocketUpdate {
    pub fn new() -> RocketUpdate {
        RocketUpdate {
            fuel: 20,
            upward_movement_force: Point2::new(0.0, -1.5),
        }
    }
}

impl Update for RocketUpdate {
    fn update(&mut self, transform: &mut Transform) {
        if self.fuel > 0 {
            transform.apply_force(self.upward_movement_force);
            self.fuel -= 1;
        }

        transform.velocity.x += transform.acceleration.x;
        transform.velocity.y += transform.acceleration.y;
        transform.location.x += transform.velocity.x;
        transform.location.y += transform.velocity.y;

        transform.acceleration.x = 0.0;
        transform.acceleration.y = 0.0;
    }
}

struct RocketEngineUpdate {}

impl Update for RocketEngineUpdate {
    fn update(&mut self, transform: &mut Transform) {
        // have engine move up and down based on time
    }
}

struct CapUpdate {}

impl Update for CapUpdate {
    fn update(&mut self, transform: &mut Transform) {}
}
