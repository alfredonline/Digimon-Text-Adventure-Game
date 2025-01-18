use std::collections::HashMap;
use crate::entities::{DigimonEntity, HumanEntity, RoomEntity, ItemEntity};

pub fn create_initial_digimon() -> (Box<DigimonEntity>, Box<DigimonEntity>, Box<DigimonEntity>, Box<DigimonEntity>, Box<DigimonEntity>) {
    let metal_greymon = Box::new(DigimonEntity::new(
        String::from("MetalGreymon"),
        300,
        35,
        vec![],
        200,
    ));

    let greymon = Box::new(DigimonEntity::new(
        String::from("Greymon"),
        250,
        30,
        vec![metal_greymon.clone()],
        150,
    ));

    let agumon = Box::new(DigimonEntity::new(
        String::from("Agumon"),
        200,
        20,
        vec![greymon.clone()],
        0,
    ));

    let gabumon = Box::new(DigimonEntity::new(
        String::from("Gabumon"),
        180,
        18,
        vec![],
        0,
    ));

    let garurumon = Box::new(DigimonEntity::new(
        String::from("Garurumon"),
        220,
        25,
        vec![],
        100,
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

    let usb_key = ItemEntity::new(
        String::from("USB Key"),
        String::from("A mysterious USB key that might contain important data."),
        true,
        50,
    );

    let entrance = RoomEntity::new(
        String::from("Entrance"),
        String::from("You are at the entrance of the cave. It is dark and damp."),
        HashMap::from([("north", "Second Room")]),
    );

    let mut second_room = RoomEntity::new(
        String::from("Second Room"),
        String::from("You are in the second room. It's slightly warmer here."),
        HashMap::from([("south", "Entrance"), ("east", "Third Room")]),
    );

    let mut third_room = RoomEntity::new(
        String::from("Third Room"),
        String::from("This room has strange markings on the walls."),
        HashMap::from([("west", "Second Room")]),
    );

    // Add items and enemies to rooms
    second_room.add_item(usb_key);
    
    let enemy = DigimonEntity::new(
        String::from("Wild Gabumon"),
        150,
        15,
        vec![],
        0,
    );
    third_room.add_enemy(enemy);

    cave_map.insert("Entrance", entrance);
    cave_map.insert("Second Room", second_room);
    cave_map.insert("Third Room", third_room);

    cave_map
}

pub fn load_game() {
    println!("You have fallen down a hole and find yourself in an abandoned weapons warehouse.");
    println!("There are a pile of weapons in front of you.");
    println!("");
}