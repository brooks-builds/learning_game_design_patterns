use ggez::event::EventHandler;
use ggez::graphics::{Color, DrawMode, DrawParam, Mesh, MeshBuilder, Rect, BLACK, WHITE};
use ggez::nalgebra::Point2;
use ggez::{graphics, Context, GameResult};

const GRAVITY: f32 = 0.5;

pub struct GameState {
    scene_graph: GraphNode,
    events: Vec<Events>,
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
            PhysicsComponent::new(false, true),
            GameObjectTypes::Cap,
        );
        let rocket_engine = GraphNode::new(
            Some(flame_mesh),
            Box::new(RocketEngineUpdate {}),
            Point2::new(0.0, 0.0),
            PhysicsComponent::new(false, false),
            GameObjectTypes::Engine,
        );
        let mut rocket_body = GraphNode::new(
            Some(rocket_body_mesh),
            Box::new(RocketUpdate::new()),
            Point2::new(screen_width / 2.0 - 5.0, screen_height - 50.0),
            PhysicsComponent::new(true, false),
            GameObjectTypes::Rocket,
        );
        rocket_body.children.push(rocket_engine);
        rocket_body.children.push(cap);
        let mut scene_graph = GraphNode::new(
            None,
            Box::new(SceneUpdate {}),
            Point2::new(0.0, 0.0),
            PhysicsComponent::new(false, false),
            GameObjectTypes::Scene,
        );
        scene_graph.children.push(rocket_body);
        Ok(GameState {
            scene_graph,
            events: vec![],
        })
    }
}

impl EventHandler for GameState {
    fn update(&mut self, _context: &mut Context) -> GameResult {
        self.scene_graph.run(&mut self.events);
        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> GameResult {
        graphics::clear(context, BLACK);

        self.scene_graph.render(&Transform::origin(), context)?;

        graphics::present(context)
    }
}

#[derive(Clone, Copy, Debug)]
struct Transform {
    pub location: Point2<f32>,
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
    physics: PhysicsComponent,
    my_type: GameObjectTypes,
}

impl GraphNode {
    pub fn new(
        mesh: Option<Mesh>,
        update: Box<dyn Update>,
        location: Point2<f32>,
        physics: PhysicsComponent,
        my_type: GameObjectTypes,
    ) -> GraphNode {
        let mut local = Transform::origin();
        local.location = location;
        GraphNode {
            mesh,
            local,
            children: vec![],
            update,
            physics,
            my_type,
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

    pub fn run(&mut self, events: &mut Vec<Events>) {
        let mut index_to_transfer = None;
        for (index, event) in events.iter().enumerate() {
            if let Events::TransferTo {
                target,
                payload: _payload,
            } = event
            {
                if *target == self.my_type {
                    index_to_transfer = Some(index);
                }
            }
        }
        if let Some(index) = index_to_transfer {
            let game_component = events.remove(index);
            if let Events::TransferTo {
                target: _target,
                payload,
            } = game_component
            {
                self.children.push(payload);
            }
        }
        self.update.update(
            &mut self.local,
            &mut self.physics,
            &mut self.children,
            events,
            &mut self.mesh,
        );
        self.physics.run(&mut self.local);
        for child in &mut self.children {
            child.run(events);
        }
    }
}

fn render_mesh(mesh: &Mesh, transform: &Transform, context: &mut Context) -> GameResult<()> {
    graphics::draw(context, mesh, DrawParam::new().dest(transform.location))
}

trait Update {
    fn update(
        &mut self,
        transform: &mut Transform,
        physics: &mut PhysicsComponent,
        children: &mut Vec<GraphNode>,
        events: &mut Vec<Events>,
        mesh: &mut Option<Mesh>,
    );
}

struct SceneUpdate {}

impl Update for SceneUpdate {
    fn update(
        &mut self,
        _transform: &mut Transform,
        physics: &mut PhysicsComponent,
        children: &mut Vec<GraphNode>,
        events: &mut Vec<Events>,
        mesh: &mut Option<Mesh>,
    ) {
    }
}

struct RocketUpdate {
    fuel: u32,
    upward_movement_force: Point2<f32>,
    did_run_out_of_fuel: bool,
    component_launch_velocity: Point2<f32>,
}

impl RocketUpdate {
    pub fn new() -> RocketUpdate {
        RocketUpdate {
            fuel: 1000,
            upward_movement_force: Point2::new(0.0, -0.5001),
            did_run_out_of_fuel: false,
            component_launch_velocity: Point2::new(0.0, -1.5),
        }
    }
}

impl Update for RocketUpdate {
    fn update(
        &mut self,
        transform: &mut Transform,
        physics: &mut PhysicsComponent,
        children: &mut Vec<GraphNode>,
        events: &mut Vec<Events>,
        _mesh: &mut Option<Mesh>,
    ) {
        if self.fuel > 0 {
            physics.apply_force(self.upward_movement_force);
            self.fuel -= 1;
        } else if self.fuel == 0 && !self.did_run_out_of_fuel {
            events.push(Events::EngineOff);
            let mut index_of_child_to_remove = None;
            for (index, child) in children.iter_mut().enumerate() {
                if child.physics.launchable {
                    child.physics.apply_force(self.component_launch_velocity);
                    child.physics.is_affected_by_gravity = true;
                    child.physics.launchable = false;
                    child.local.location = transform.location.clone();
                    index_of_child_to_remove = Some(index);
                }
            }
            self.did_run_out_of_fuel = true;
            if let Some(index) = index_of_child_to_remove {
                let child_component = children.remove(index);
                events.push(Events::TransferTo {
                    target: GameObjectTypes::Scene,
                    payload: child_component,
                });
            }
        }
    }
}

struct RocketEngineUpdate {}

impl Update for RocketEngineUpdate {
    fn update(
        &mut self,
        _transform: &mut Transform,
        _physics: &mut PhysicsComponent,
        _children: &mut Vec<GraphNode>,
        events: &mut Vec<Events>,
        mesh: &mut Option<Mesh>,
    ) {
        let mut index_to_remove = None;
        for (index, event) in events.iter().enumerate() {
            if let Events::EngineOff = event {
                println!("turning engine off");
                *mesh = None;
                index_to_remove = Some(index);
            }
        }

        if let Some(index) = index_to_remove {
            events.remove(index);
        }
    }
}

struct CapUpdate {}

impl Update for CapUpdate {
    fn update(
        &mut self,
        transform: &mut Transform,
        physics: &mut PhysicsComponent,
        children: &mut Vec<GraphNode>,
        events: &mut Vec<Events>,
        mesh: &mut Option<Mesh>,
    ) {
    }
}

struct PhysicsComponent {
    pub velocity: Point2<f32>,
    acceleration: Point2<f32>,
    is_affected_by_gravity: bool,
    gravity: Point2<f32>,
    pub launchable: bool,
}

impl PhysicsComponent {
    pub fn new(is_affected_by_gravity: bool, launchable: bool) -> PhysicsComponent {
        PhysicsComponent {
            velocity: Point2::new(0.0, 0.0),
            acceleration: Point2::new(0.0, 0.0),
            is_affected_by_gravity,
            gravity: Point2::new(0.0, GRAVITY),
            launchable,
        }
    }

    pub fn apply_force(&mut self, force: Point2<f32>) {
        self.acceleration.x += force.x;
        self.acceleration.y += force.y;
    }

    pub fn run(&mut self, transform: &mut Transform) {
        if self.is_affected_by_gravity {
            self.apply_force(self.gravity);
        }

        self.velocity.x += self.acceleration.x;
        self.velocity.y += self.acceleration.y;
        transform.location.x += self.velocity.x;
        transform.location.y += self.velocity.y;
        self.acceleration.x = 0.0;
        self.acceleration.y = 0.0;
    }
}

enum Events {
    EngineOff,
    TransferTo {
        target: GameObjectTypes,
        payload: GraphNode,
    },
}

#[derive(PartialEq)]
enum GameObjectTypes {
    Scene,
    Rocket,
    Cap,
    Engine,
}
