use super::DigimonEntity;
use super::ItemEntity;

#[derive(Clone)]

pub struct HumanEntity {
    pub name: String,
    pub linked_digimon: Box<DigimonEntity>, // The Digimon that the human is linked to
    pub inventory: Vec<ItemEntity>
}

impl HumanEntity {
    pub fn new(name: String, linked_digimon: Box<DigimonEntity>, inventory: Vec<ItemEntity>) -> Self {
        HumanEntity {
            name,
            linked_digimon,
            inventory
        }
    }   

    pub fn add_to_inventory(&mut self, item: ItemEntity) {
        self.inventory.push(item);
    }

    pub fn remove_from_inventory(&mut self, item_name: &str) -> Option<ItemEntity> {
        if let Some(pos) = self.inventory.iter().position(|item| item.name.to_lowercase() == item_name.to_lowercase()) {
            Some(self.inventory.remove(pos))
        } else {
            None
        }
    }

    pub fn has_item(&self, item_name: &str) -> bool {
        println!("{}", item_name);
        self.inventory
            .iter()
            .any(|item| item.name.to_lowercase() == item_name.to_lowercase())
    }
}
