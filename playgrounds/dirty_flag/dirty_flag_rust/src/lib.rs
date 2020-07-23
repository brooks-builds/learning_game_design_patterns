use ggez::event::EventHandler;
use ggez::graphics::{Color, DrawMode, DrawParam, Mesh, MeshBuilder, Rect, BLACK, WHITE};
use ggez::nalgebra::Point2;
use ggez::{graphics, Context, GameResult};

pub struct GameState {
    scene_graph: GraphNode,
}

impl GameState {
    pub fn new(context: &mut Context) -> GameResult<GameState> {
        let rocket_body_mesh = MeshBuilder::new()
            .rectangle(DrawMode::fill(), Rect::new(0.0, 0.0, 10.0, 50.0), WHITE)
            .build(context)?;
        let flame_mesh = MeshBuilder::new()
            .triangles(
                &[
                    Point2::new(-5.0, 0.0),
                    Point2::new(10.0, 0.0),
                    Point2::new(0.0, 10.0),
                ],
                Color::new(1.0, 0.0, 0.0, 1.0),
            )?
            .build(context)?;
        let rocket_engine = GraphNode::new(Some(flame_mesh), Box::new(RocketEngineUpdate {}));
        let mut rocket_body = GraphNode::new(Some(rocket_body_mesh), Box::new(RocketUpdate {}));
        rocket_body.children.push(rocket_engine);
        let mut scene_graph = GraphNode::new(None, Box::new(SceneUpdate {}));
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
    location: Point2<f32>,
}

impl Transform {
    pub fn origin() -> Transform {
        Transform {
            location: Point2::new(0.0, 0.0),
        }
    }

    pub fn combine(&mut self, other: &Transform) {
        self.location.x += other.location.x;
        self.location.y += other.location.y;
    }
}

struct GraphNode {
    mesh: Option<Mesh>,
    pub local: Transform,
    pub children: Vec<GraphNode>,
    update: Box<dyn Update>,
    location: Point2<f32>,
}

impl GraphNode {
    pub fn new(mesh: Option<Mesh>, update: Box<dyn Update>) -> GraphNode {
        GraphNode {
            mesh,
            local: Transform::origin(),
            location: Point2::new(0.0, 0.0),
            children: vec![],
            update,
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

struct RocketUpdate {}

impl Update for RocketUpdate {
    fn update(&mut self, transform: &mut Transform) {
        transform.location.x += 1.0;
        transform.location.y += 1.0;
    }
}

struct RocketEngineUpdate {}

impl Update for RocketEngineUpdate {
    fn update(&mut self, transform: &mut Transform) {
        // have engine move up and down based on time
    }
}
