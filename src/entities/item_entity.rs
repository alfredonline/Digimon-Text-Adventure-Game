#[derive(Clone)]
pub struct ItemEntity {
    pub name: String,
    pub description: String,
    pub gives_health: bool,
    pub health_points: i32,
}

impl ItemEntity {
    pub fn new(name: String, description: String, gives_health: bool, health_points: i32) -> Self {
        ItemEntity {
            name,
            description,
            gives_health,
            health_points,
        }
    }
    
}