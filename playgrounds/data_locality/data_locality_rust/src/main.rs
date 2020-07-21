const MAX_PARTICLES: usize = 100_000;

enum Errors {
    ParticleAlreadyActive,
    ParticleAlreadyInactive,
}

enum Animations {
    Walk,
}

enum LootType {
    SwordPlusOne,
}

struct GameEntity {
    pub ai: AIComponent,
    pub physics: PhysicsComponent,
    pub render: RenderComponent,
}

impl GameEntity {
    pub fn new(ai: AIComponent, physics: PhysicsComponent, render: RenderComponent) -> GameEntity {
        GameEntity {
            ai,
            physics,
            render,
        }
    }
}

struct AIComponent {
    animation: Animations,
    energy: f32,
    goal_position: Vec<f32>,
    loot_index: usize,
}

impl AIComponent {
    pub fn new(
        animation: Animations,
        drop: LootType,
        min_drops: i32,
        max_drops: i32,
        drop_chance: f32,
        all_loots: &mut Loots,
    ) -> AIComponent {
        AIComponent {
            animation,
            energy: 100.0,
            goal_position: vec![0.0, 0.0],
            // loot: LootDrop::new(drop, min_drops, max_drops, drop_chance),
            loot_index: all_loots.add_loot(LootDrop::new(drop, min_drops, max_drops, drop_chance)),
        }
    }
    pub fn update(&mut self) {
        // do the update code here
    }
}

struct LootDrop {
    drop: LootType,
    min_drops: i32,
    max_drops: i32,
    drop_chance: f32,
}

impl LootDrop {
    pub fn new(drop: LootType, min_drops: i32, max_drops: i32, drop_chance: f32) -> LootDrop {
        LootDrop {
            drop,
            min_drops,
            max_drops,
            drop_chance,
        }
    }
}

struct Loots {
    all_loots: Vec<LootDrop>,
}

impl Loots {
    pub fn new() -> Loots {
        Loots { all_loots: vec![] }
    }

    pub fn add_loot(&mut self, loot: LootDrop) -> usize {
        self.all_loots.push(loot);
        self.all_loots.len() - 1
    }

    pub fn get_loot(&self, id: usize) -> &LootDrop {
        &self.all_loots[id]
    }
}

struct PhysicsComponent {}

impl PhysicsComponent {
    pub fn update(&mut self) {
        // do the update code here
    }
}

struct RenderComponent {}

impl RenderComponent {
    pub fn update(&mut self) {
        // do the update code here
    }
}

#[derive(Copy, Clone)]
struct Particle {
    is_active: bool,
}

impl Particle {
    pub fn update(&mut self) {
        // update the movement of the particle
    }
}

struct ParticleSystem {
    num_particles: usize,
    particles: [Particle; MAX_PARTICLES],
    num_active: usize,
}

impl ParticleSystem {
    pub fn new() -> ParticleSystem {
        ParticleSystem {
            num_particles: 0,
            particles: [Particle { is_active: false }; MAX_PARTICLES],
            num_active: 0,
        }
    }

    pub fn update(&mut self) {
        for particle in &mut self.particles.iter_mut() {
            if particle.is_active {
                particle.update();
            }
        }
    }

    pub fn activate_particle(&mut self, index: usize) -> Result<(), Errors> {
        if index < self.num_active {
            return Err(Errors::ParticleAlreadyActive);
        }

        let temp = self.particles[self.num_active];
        self.particles[self.num_active] = self.particles[index];
        self.particles[index] = temp;

        self.num_active += 1;
        Ok(())
    }

    pub fn deactivate_particle(&mut self, index: usize) -> Result<(), Errors> {
        if index >= self.num_active {
            return Err(Errors::ParticleAlreadyInactive);
        }

        self.num_active -= 1;
        let temp = self.particles[self.num_active];
        self.particles[self.num_active] = self.particles[index];
        self.particles[index] = temp;

        Ok(())
    }
}

fn main() {
    // setup
    let mut all_loots = Loots::new();
    let mut ai_componenets = vec![
        AIComponent::new(
            Animations::Walk,
            LootType::SwordPlusOne,
            1,
            5,
            0.25,
            &mut all_loots,
        ),
        AIComponent::new(
            Animations::Walk,
            LootType::SwordPlusOne,
            1,
            5,
            0.25,
            &mut all_loots,
        ),
        AIComponent::new(
            Animations::Walk,
            LootType::SwordPlusOne,
            1,
            5,
            0.25,
            &mut all_loots,
        ),
    ];
    let mut physics_components = vec![
        PhysicsComponent {},
        PhysicsComponent {},
        PhysicsComponent {},
    ];
    let mut render_components = vec![RenderComponent {}, RenderComponent {}, RenderComponent {}];
    let gameOver = false;

    //main game loop
    while !gameOver {
        for ai_component in &mut ai_componenets {
            ai_component.update();
        }
        for physics_component in &mut physics_components {
            physics_component.update();
        }
        for render_components in &mut render_components {
            render_components.update();
        }
    }
}
