use std::io;
mod entities;
mod game;

use colored::*;
use game::commands::{self};
use game::setup::{create_character_pairs, create_initial_digimon};
use game::state::GameState;
use rand::Rng;

fn main() {
    game::load_game();

    let (_metal_greymon, _greymon, agumon, gabumon, garurumon,
    _weregarurumon, _piyomon, _birdramon) = create_initial_digimon();
    let valid_character_pairs = create_character_pairs(agumon, gabumon, garurumon);
    let cave_map = game::setup_map();

    let mut game_state = GameState::new(valid_character_pairs, cave_map);
    game_state.select_character();

    game::print_valid_commands();



    while game_state.game_running {
        let current_room = game_state.get_current_room();

        if current_room.has_enemies() {
            if let Some(enemy) = current_room.get_first_enemy() {
                if enemy.health_points < enemy.starting_health {
                    println!(
                        "{} is still alive and has {} health! Keep attacking",
                        enemy.name, enemy.health_points
                    );
                } else {
                    println!(
                        "{}",
                        format!("You see an enemy: {} ({})", enemy.name, enemy.health_points)
                            .green()
                    );
                    println!(
                        "{}",
                        format!("You need to delete {} to get to the next room.", enemy.name)
                            .green()
                    )
                }
            }
        } else {
            println!("{}", "The room is empty.".green());
            println!("{}", "You can go in the following directions: ".green());
            println!("---------------------------------------------------");
            for direction in current_room.get_available_directions() {
                println!("{}", direction.blue());
            }
            println!("---------------------------------------------------");
        }

        if current_room.has_items() {
            println!(
                "{}",
                "There is an item in this room. Use the 'pick up' command to collect it.".green()
            );
            println!("---------------------------------------------------");
            for item in &current_room.items {
                println!("- {}", item.name.blue());
            }
            println!("---------------------------------------------------");
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
                        println!("{}", format!("You defeated! {}", enemy.name).yellow());
                        current_room.entities.remove(0);
                        game_state.selected_character.add_experience(50);
                    } else {
                        println!(
                            "{}",
                            format!("You attacked {} for {} damage.", enemy.name, attack_points)
                                .yellow()
                        );

                        let mut player_position = 1;

                        println!("{} is preparing an attack. Where do you want to go? Remain, left or right?", enemy.name);

                        loop {
                            let mut player_digimon_position = String::new();
                            io::stdin()
                                .read_line(&mut player_digimon_position)
                                .expect("Failed to read line");

                            let player_digimon_position = player_digimon_position.trim().to_lowercase();


                            match player_digimon_position.as_str() {
                                "remain" => {
                                    player_position = 1;
                                    break;
                                },
                                "left" => {
                                    player_position = 0;
                                    break;
                                },
                                "right" => {
                                    player_position = 2;
                                    break;
                                }
                                _ => {
                                    println!("Invalid value. Please enter either remain, left or right.")
                                }
                            }

                        
                        }

                        let random_number = rand::thread_rng().gen_range(0..3);

                        if player_position == random_number {
                            let damage_to_take = enemy.attack_points;        
                            game_state.selected_character.subtract_health(damage_to_take);
                            if game_state.selected_character.health_points == 0 {
                                println!("{} has been deleted and you have failed the mission!", game_state.selected_character.name);
                                game_state.game_running = false;
                            }
                            println!("You took {} damage!", damage_to_take);
                        } else {
                            println!("{} attacked but you dodged the attack.", enemy.name);
                        }

                        
                    }
                } else {
                    println!("{}", "There are no enemies to attack.".red());
                }
            }


            "evolve" => {
                if commands::evolve_digimon(&mut game_state.selected_character) {
                    println!("Evolving...");
                    println!("{} evolution!!!!!!!!!!!!!!!!!!!", game_state.selected_character.name);
                    if let Some(evolution) = game_state.selected_character.get_next_evolution() {
                        game_state.selected_character = evolution.clone();
                        println!(
                            "Your digimon has evolved into: {}",
                            game_state.selected_character.name
                        );
                    }
                } else {
                    println!("You do not have enough experience points to evolve.");
                }
            }

            "stats" => commands::print_stats(&game_state.selected_character),

            "pick up" => {
                let current_room = game_state.get_current_room();
                if let Some(item) = commands::pick_up_item(current_room) {
                    println!(
                        "{}",
                        format!("You picked up the item: {}", item.name).yellow()
                    );
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




                let item_name = split_input[1..].join(" ");
                if game_state.has_item(&item_name) {
                    if let Some(item) = game_state.remove_item_from_inventory(&item_name) {
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
            }
        }
    }
}
