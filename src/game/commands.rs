use crate::entities::{DigimonEntity, RoomEntity, ItemEntity};

pub fn print_valid_commands() {
    println!("There are a few commands you can use:");
    println!("1. Attack - Attack the enemy you are facing.");
    println!("2. Evolve - Evolve your digimon. You can only do this once you have enough experience points.");
    println!("3. Stats - Print your Digimon's stats.");
    println!("4. Go <direction> - Move in a direction.");
    println!("5. Use <item> - Use an item in your inventory.");
    println!("6. Add <item> - Add an item to your inventory.");
    println!("7. Inventory - Show your current inventory.");
    println!("8. Character - Show your character information.");
    println!("9. Quit - Exit the game.");
}

pub fn evolve_digimon(digimon: &mut DigimonEntity) -> bool {
    if digimon.can_evolve() {
        if let Some(evolution) = digimon.get_next_evolution() {
            println!("Evolving into {}", evolution.name);
            return true;
        }
    }
    false
}

pub fn navigate_between_rooms<'a>(current_room: &RoomEntity<'a>, direction: &str) -> Option<&'a str> {
    current_room.connections.get(direction).copied()
}

pub fn attack_enemy(enemy: &mut DigimonEntity, attack_points: i32) -> bool {
    enemy.subtract_health(attack_points);
    enemy.health_points == 0
}

pub fn print_stats(digimon: &DigimonEntity) {
    println!("Name: {}", digimon.name);
    println!("Health Points: {}", digimon.health_points);
    println!("Attack Points: {}", digimon.attack_points);
    println!("Experience Points: {}", digimon.experience);
}

pub fn pick_up_item(room: &mut RoomEntity) -> Option<ItemEntity> {
    if !room.items.is_empty() {
        Some(room.items.remove(0))
    } else {
        None
    }
}

pub fn use_item(item: &ItemEntity, digimon: &mut DigimonEntity) {
    if item.gives_health {
        digimon.health_points += item.health_points;
        println!(
            "You used the item: {}. Your health points are now: {}",
            item.name, digimon.health_points
        );
    }
}

// ... other existing command functions ...
