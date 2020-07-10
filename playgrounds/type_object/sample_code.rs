struct Breed {
    health: i32,
    attack: &'static str
}

impl Breed {
    pub fn new(health: i32, attack: &'static str) -> Breed {
        Breed {
            health,
            attack
        }
    }

    pub fn get_health(&self) -> i32 {
        self.health
    }

    pub fn get_attack(&self) -> &str {
        self.attack
    }
}