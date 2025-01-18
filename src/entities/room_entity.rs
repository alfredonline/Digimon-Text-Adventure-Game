use super::ItemEntity;
use super::DigimonEntity;
use std::collections::HashMap as Hashmap;

#[derive(Clone)]
pub struct RoomEntity<'a> {
    pub name: String,
    pub entities: Vec<DigimonEntity>,
    pub description: String,
    pub connections: Hashmap<&'a str, &'a str>,
    pub items: Vec<ItemEntity>,
}

impl<'a> RoomEntity<'a> {
    pub fn new(
        name: String,
        description: String,
        connections: Hashmap<&'a str, &'a str>,
    ) -> Self {
        RoomEntity {
            name,
            description,
            connections,
            entities: Vec::new(),
            items: Vec::new(),
        }
    }

    pub fn add_enemy(&mut self, enemy: DigimonEntity) {
        self.entities.push(enemy);
    }

    pub fn add_item(&mut self, item: ItemEntity) {
        self.items.push(item);
    }

    pub fn has_enemies(&self) -> bool {
        !self.entities.is_empty()
    }

    pub fn has_items(&self) -> bool {
        !self.items.is_empty()
    }

    pub fn get_available_directions(&self) -> Vec<&str> {
        self.connections.keys().copied().collect()
    }

    pub fn get_first_enemy(&self) -> Option<&DigimonEntity> {
        self.entities.first()
    }

    pub fn get_first_enemy_mut(&mut self) -> Option<&mut DigimonEntity> {
        self.entities.first_mut()
    }
}

