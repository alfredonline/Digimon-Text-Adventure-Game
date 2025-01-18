#[derive(Clone)]

pub struct DigimonEntity {
    pub name: String,
    pub health_points: i32,
    pub attack_points: i32,
    pub evolutions: Vec<Box<DigimonEntity>>,
    pub experience: i32,
}

impl DigimonEntity {
    pub fn new(
        name: String,
        health_points: i32,
        attack_points: i32,
        evolutions: Vec<Box<DigimonEntity>>,
        experience: i32,
    ) -> DigimonEntity {
        DigimonEntity {
            name: String::from(name),
            health_points,
            attack_points,
            evolutions,
            experience,
        }
    }

    pub fn subtract_health(&mut self, attack_points: i32) -> i32 {
        self.health_points -= attack_points;
        if self.health_points <= 0 {
            self.health_points = 0;
        }
        self.health_points
    }

    pub fn add_experience(&mut self, exp: i32) {
        self.experience += exp;
    }

    pub fn can_evolve(&self) -> bool {
        if let Some(evolution) = self.evolutions.get(0) {
            self.experience >= evolution.experience
        } else {
            false
        }
    }

    pub fn get_next_evolution(&self) -> Option<&Box<DigimonEntity>> {
        self.evolutions.get(0)
    }
}
