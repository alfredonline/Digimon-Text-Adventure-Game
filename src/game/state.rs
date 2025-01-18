use crate::entities::{DigimonEntity, HumanEntity, RoomEntity, ItemEntity};
use std::collections::HashMap;
use std::io;

pub struct GameState {
    pub valid_character_pairs: Vec<HumanEntity>,
    pub selected_character_index: usize,
    pub selected_character: Box<DigimonEntity>,
    pub cave_map: HashMap<&'static str, RoomEntity<'static>>,
    pub current_room_key: &'static str,
}

impl GameState {
    pub fn new(
        valid_character_pairs: Vec<HumanEntity>,
        cave_map: HashMap<&'static str, RoomEntity<'static>>,
    ) -> Self {
        let selected_character = valid_character_pairs[0].linked_digimon.clone();
        
        Self {
            valid_character_pairs,
            selected_character_index: 0,
            selected_character,
            cave_map,
            current_room_key: "Entrance",
        }
    }

    pub fn select_character(&mut self) {
        println!(
            "You are playing (by default) as {} and your digimon is {}",
            self.valid_character_pairs[self.selected_character_index].name,
            self.valid_character_pairs[self.selected_character_index].linked_digimon.name
        );

        println!("You can change your character by typing in the name of the character you want to play as.");
        println!("The valid characters are: ");
        for character in &self.valid_character_pairs {
            println!("{}", character.name);
        }

        let mut character_selected = false;
        while !character_selected {
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");
            let input = input.trim().to_lowercase();

            match input.as_str() {
                "taichi" => {
                    println!("You are now playing as Taichi and your digimon is Agumon.");
                    self.set_selected_character(0);
                    character_selected = true;
                }
                "yamato" => {
                    println!("You are now playing as Yamato and your digimon is Gabumon.");
                    self.set_selected_character(1);
                    character_selected = true;
                }
                "sora" => {
                    println!("You are now playing as Sora and your digimon is Garurumon.");
                    self.set_selected_character(2);
                    character_selected = true;
                }
                _ => {
                    println!("Invalid character. Please select a valid character.");
                }
            }
        }
    }

    pub fn add_item_to_inventory(&mut self, item: ItemEntity) {
        self.valid_character_pairs[self.selected_character_index].add_to_inventory(item);
    }

    pub fn remove_item_from_inventory(&mut self, item_name: &str) -> Option<ItemEntity> {
        self.valid_character_pairs[self.selected_character_index].remove_from_inventory(item_name)
    }

    pub fn get_current_room(&mut self) -> &mut RoomEntity<'static> {
        self.cave_map.get_mut(self.current_room_key).unwrap()
    }

    pub fn set_selected_character(&mut self, index: usize) {
        self.selected_character_index = index;
        self.selected_character = self.valid_character_pairs[index].linked_digimon.clone();
    }

    pub fn has_item(&self, item_name: &str) -> bool {
        self.valid_character_pairs[self.selected_character_index].has_item(item_name)
    }

    pub fn print_inventory(&self) {
        let inventory = &self.valid_character_pairs[self.selected_character_index].inventory;
        if inventory.is_empty() {
            println!("Your inventory is empty.");
        } else {
            println!("Your inventory contains:");
            for item in inventory {
                println!("- {} ({})", item.name, item.description);
            }
        }
    }

    pub fn get_selected_character(&self) -> &HumanEntity {
        &self.valid_character_pairs[self.selected_character_index]
    }
}