use std::io;
mod entities;
mod game;

use game::commands;
use game::setup::{create_character_pairs, create_initial_digimon};
use game::state::GameState;

fn main() {
    game::load_game();

    let (_metal_greymon, _greymon, agumon, gabumon, garurumon) = create_initial_digimon();
    let valid_character_pairs = create_character_pairs(agumon, gabumon, garurumon);
    let cave_map = game::setup_map();

    let mut game_state = GameState::new(valid_character_pairs, cave_map);
    game_state.select_character();

    game::print_valid_commands();

    loop {
        let current_room = game_state.get_current_room();

        if current_room.has_enemies() {
            if let Some(enemy) = current_room.get_first_enemy() {
                println!("You see an enemy: {}", enemy.name);
            }
        } else {
            println!("The room is empty.");
            println!("You can go in the following directions: ");
            for direction in current_room.get_available_directions() {
                println!("{}", direction);
            }
        }

        if current_room.has_items() {
            println!("You see some items in this room!");
            for item in &current_room.items {
                println!("- {}", item.name);
            }
        }

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let input = input.trim().to_lowercase();

        match input.as_str() {
            "quit" => {
                println!("You have quit the game. Thank you for playing!");
                break;
            }

            "attack" => {
                let attack_points = game_state.selected_character.attack_points;
                let current_room = game_state.get_current_room();
                
                if let Some(enemy) = current_room.get_first_enemy_mut() {
                    if commands::attack_enemy(enemy, attack_points) {
                        println!("You defeated {}!", enemy.name);
                        current_room.entities.remove(0);
                        game_state.selected_character.add_experience(50);
                    } else {
                        println!(
                            "You attacked {}. Remaining health: {}",
                            enemy.name, enemy.health_points
                        );
                    }
                } else {
                    println!("There are no enemies to attack.");
                }
            }

            "evolve" => {
                if commands::evolve_digimon(&mut game_state.selected_character) {
                    println!("Evolving...");
                    if let Some(evolution) = game_state.selected_character.get_next_evolution() {
                        game_state.selected_character = evolution.clone();
                        println!("Your digimon has evolved into: {}", game_state.selected_character.name);
                    }
                } else {
                    println!("You do not have enough experience points to evolve.");
                }
            }

            "stats" => commands::print_stats(&game_state.selected_character),

            "pick up" => {
                let current_room = game_state.get_current_room();
                if let Some(item) = commands::pick_up_item(current_room) {
                    println!("You picked up the item: {}", item.name);
                    game_state.add_item_to_inventory(item);
                } else {
                    println!("There are no items to pick up.");
                }
            }

            input if input.starts_with("use") => {
                let split_input: Vec<&str> = input.split_whitespace().collect();
                if split_input.len() < 2 {
                    println!("Invalid command. Please specify an item to use.");
                    continue;
                }
                
                let item_name = split_input[1];
                if game_state.has_item(item_name) {
                    if let Some(item) = game_state.remove_item_from_inventory(item_name) {
                        commands::use_item(&item, &mut game_state.selected_character);
                    }
                } else {
                    println!("You do not have that item in your inventory.");
                }
            }

            "character" => {
                let character = game_state.get_selected_character();
                println!("Current character: {}", character.name);
                println!("Digimon partner: {}", character.linked_digimon.name);
                println!("\nInventory:");
                game_state.print_inventory();
            }

            direction @ ("north" | "south" | "east" | "west") => {
                let current_room = game_state.get_current_room();
                if let Some(next_room_key) =
                    commands::navigate_between_rooms(current_room, direction)
                {
                    game_state.current_room_key = next_room_key;
                    let next_room = game_state.get_current_room();
                    println!("You are now in the room: {}", next_room.name);
                    println!("{}", next_room.description);
                    if !next_room.items.is_empty() {
                        println!("You see an item: {}", next_room.items[0].name);
                    }
                } else {
                    println!("You cannot go that direction.");
                }
            }

            "inventory" => {
                game_state.print_inventory();
            }

            _ => {
                println!("Invalid command. Please enter a valid command.");
                game::print_valid_commands();
            }
        }
    }
}
