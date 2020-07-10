use serde::{Deserialize, Serialize};
use serde_json::Result;

struct Breed {
    health: i32,
    attack: &'static str,
}

impl Breed {
    pub fn new(
        parent: Option<&Breed>,
        given_health: Option<i32>,
        given_attack: Option<&'static str>,
    ) -> Breed {
        let health = if let Some(health) = given_health {
            health
        } else {
            if let Some(parent) = parent {
                parent.get_health()
            } else {
                panic!("There must be a parent breed");
            }
        };

        let attack = if let Some(attack) = given_attack {
            attack
        } else {
            if let Some(parent) = parent {
                parent.get_attack()
            } else {
                panic!("There must be a parent breed");
            }
        };
        Breed { health, attack }
    }

    pub fn get_health(&self) -> i32 {
        self.health
    }

    pub fn get_attack(&self) -> &'static str {
        self.attack
    }

    pub fn new_monster(&self) -> Monster {
        Monster::new(&self)
    }
}

struct Monster {
    health: i32,
    attack: &'static str,
}

impl Monster {
    pub fn new(breed: &Breed) -> Monster {
        Monster {
            health: breed.get_health(),
            attack: breed.get_attack(),
        }
    }

    pub fn get_attack(&self) -> &str {
        self.attack
    }
}

pub struct MonsterData {
    parent: String,
    health: i32,
    attack: String,
}

fn main() {
    let base_dragon = Breed::new(
        None,
        Some(248),
        Some("The Dragon roars and breathes flame at you!"),
    );
    let green_dragon = Breed::new(
        Some(&base_dragon),
        None,
        Some("The Dragon spouts green flame at you!"),
    );
    let danny = base_dragon.new_monster();
    let sally = green_dragon.new_monster();
    println!("{}", danny.get_attack());
    println!("{}", sally.get_attack());

    let raw_breed_data = r#"
    [
        "troll": {
          "parent": "",
          "health": 45,
          "attack": "the troll hits you in the head"
        },
        "cave_troll": {
          "parent": "troll",
          "health": 0,
          "attack": "the cave troll swipes a grimy hand at you"
        },
        "mountain_troll": {
            "parent": "troll",
            "health": 0,
            "attack": ""
        }
      ]
    "#;
}
