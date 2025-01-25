use std::collections::HashMap;
use crate::entities::{DigimonEntity, HumanEntity, RoomEntity, ItemEntity};
use colored::*;

pub fn create_initial_digimon() -> (Box<DigimonEntity>, Box<DigimonEntity>, Box<DigimonEntity>, Box<DigimonEntity>, Box<DigimonEntity>) {
    let metal_greymon = Box::new(DigimonEntity::new(
        String::from("MetalGreymon"),
        300,
        100,
        vec![],
        200,
        400
    ));

    let greymon = Box::new(DigimonEntity::new(
        String::from("Greymon"),
        250,
        60,
        vec![metal_greymon.clone()],
        100,
        100
    ));

    let agumon = Box::new(DigimonEntity::new(
        String::from("Agumon"),
        200,
        20,
        vec![greymon.clone()],
        0,
        200
    ));

    let gabumon = Box::new(DigimonEntity::new(
        String::from("Gabumon"),
        180,
        18,
        vec![],
        0,
        180
    ));

    let garurumon = Box::new(DigimonEntity::new(
        String::from("Garurumon"),
        220,
        25,
        vec![],
        100,
        220
    ));

    (metal_greymon, greymon, agumon, gabumon, garurumon)
}

pub fn create_character_pairs(agumon: Box<DigimonEntity>, gabumon: Box<DigimonEntity>, garurumon: Box<DigimonEntity>) -> Vec<HumanEntity> {
    vec![
        HumanEntity::new(
            String::from("Taichi"),
            agumon,
            vec![],
        ),
        HumanEntity::new(
            String::from("Yamato"),
            gabumon,
            vec![],
        ),
        HumanEntity::new(
            String::from("Sora"),
            garurumon,
            vec![],
        ),
    ]
}

pub fn setup_map() -> HashMap<&'static str, RoomEntity<'static>> {
    let mut cave_map = HashMap::new();

    // Create rooms
    let entrance = RoomEntity::new(
        String::from("Entrance Chamber"),
        String::from("A dimly lit chamber with fallen debris. Digital code glows faintly on the walls, providing just enough light to see. The hole you fell through is too high to reach."),
        HashMap::from([("north", "Weapons Storage")]),
    );

    let mut weapons_storage = RoomEntity::new(
        String::from("Weapons Storage"),
        String::from("Ancient weapon racks line the walls, most empty but some still holding mysterious devices. The air feels charged with digital energy."),
        HashMap::from([
            ("south", "Entrance Chamber"),
            ("east", "Training Ground"),
            ("west", "Power Core"),
            ("north", "Hidden Armory")
        ]),
    );

    let power_core = RoomEntity::new(
        String::from("Power Core"),
        String::from("A massive cylindrical chamber housing what appears to be an ancient power source. Corrupted data streams crackle through the air."),
        HashMap::from([("east", "Weapons Storage")]),
    );

    let mut training_ground = RoomEntity::new(
        String::from("Training Ground"),
        String::from("A large open chamber with battle-scarred walls. This seems to be where ancient Digimon warriors once trained."),
        HashMap::from([
            ("west", "Weapons Storage"),
            ("north", "Exit Tunnel"),
            ("east", "Data Library")
        ]),
    );

    let mut exit_tunnel = RoomEntity::new(
        String::from("Exit Tunnel"),
        String::from("A long tunnel sloping upward. Daylight can be seen in the distance, but a powerful corrupted Digimon blocks the path."),
        HashMap::from([("south", "Training Ground"), ("north", "Surface Gateway")]),
    );

    let mut hidden_armory = RoomEntity::new(
        String::from("Hidden Armory"),
        String::from("A concealed room filled with rare and powerful artifacts. The air is heavy with an aura of strength."),
        HashMap::from([("south", "Weapons Storage")]),
    );

    let data_library = RoomEntity::new(
        String::from("Data Library"),
        String::from("Rows of glowing panels and ancient data terminals. Knowledge about Digivolutions and tactics might be found here."),
        HashMap::from([("west", "Training Ground")]),
    );

    let mut surface_gateway = RoomEntity::new(
        String::from("Surface Gateway"),
        String::from("A bright room leading to the surface. The exit is tantalizingly close, but it is guarded by the strongest corrupted Digimon."),
        HashMap::from([("south", "Exit Tunnel")]),
    );

    // Items
    let healing_disk = ItemEntity::new(
        String::from("Recovery Disk"),
        String::from("A glowing disk containing healing data."),
        true,
        100,
    );

    let power_chip = ItemEntity::new(
        String::from("Power Chip"),
        String::from("A rare chip that temporarily boosts a Digimon's attack power."),
        true,
        50,
    );

    let defense_module = ItemEntity::new(
        String::from("Defense Module"),
        String::from("A device that increases a Digimon's defense."),
        true,
        50,
    );

    // Add items to rooms
    weapons_storage.add_item(power_chip);
    hidden_armory.add_item(defense_module);
    training_ground.add_item(healing_disk);

    // Add enemy Digimon with progressive difficulty
    let corrupted_gazimon = DigimonEntity::new(
        String::from("Corrupted Gazimon"),
        100,
        15,
        vec![],
        25,
        100
    );

    let corrupted_monochromon = DigimonEntity::new(
        String::from("Corrupted Monochromon"),
        200,
        25,
        vec![],
        75,
        200
    );

    let corrupted_andromon = DigimonEntity::new(
        String::from("Corrupted Andromon"),
        300,
        35,
        vec![],
        150,
        300
    );

    let corrupted_megakabuterimon = DigimonEntity::new(
        String::from("Corrupted MegaKabuterimon"),
        400,
        45,
        vec![],
        200,
        400
    );

    let corrupted_machinedramon = DigimonEntity::new(
        String::from("Corrupted Machinedramon"),
        500,
        50,
        vec![],
        300,
        500
    );

    // Add enemies to rooms
    weapons_storage.add_enemy(corrupted_gazimon);
    training_ground.add_enemy(corrupted_monochromon);
    exit_tunnel.add_enemy(corrupted_andromon);
    hidden_armory.add_enemy(corrupted_megakabuterimon);
    surface_gateway.add_enemy(corrupted_machinedramon);

    // Add rooms to map
    cave_map.insert("Entrance Chamber", entrance);
    cave_map.insert("Weapons Storage", weapons_storage);
    cave_map.insert("Power Core", power_core);
    cave_map.insert("Training Ground", training_ground);
    cave_map.insert("Exit Tunnel", exit_tunnel);
    cave_map.insert("Hidden Armory", hidden_armory);
    cave_map.insert("Data Library", data_library);
    cave_map.insert("Surface Gateway", surface_gateway);

    cave_map
}

pub fn load_game() {
    println!("{}", "\nDIGITAL WORLD EMERGENCY SITUATION\n".red().bold());
    println!("{}", "After a fierce battle with a mysterious enemy Digimon, you and your partner");
    println!("{}", "have been separated from the rest of the DigiDestined team.");
    println!("{}", "You've fallen into what appears to be an ancient underground facility...");
    println!("{}", "\nYour Digivice is picking up a signal from the surface, but you'll need");
    println!("{}", "to find your way through this labyrinth while avoiding or battling");
    println!("{}", "the corrupted Digimon that have made this place their home.");
    println!("{}", "\nBe careful - your partner Digimon's energy is limited, and you'll need");
    println!("{}", "to find resources to help them recover and possibly evolve...\n");
}
