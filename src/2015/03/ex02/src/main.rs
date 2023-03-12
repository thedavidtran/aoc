use std::collections::HashSet;
use std::fs;

#[derive(PartialEq, Eq, Hash, Clone)]
struct Coordinate<> {
    x: i32,
    y: i32,
}

struct Entity {
    name: String,
    location: Coordinate
}

fn move_entity(entity : &mut Entity, direction: char) {
    match direction {
        '<' => { // left
            println!("{direction} left");
            entity.location.x -= 1;
        }
        '>' => { // right
            println!("{direction} right");
            entity.location.x += 1;
        }
        '^' => { // up
            println!("{direction} up");
            entity.location.y += 1;
        }
        'v' => { // down
            println!("{direction} down");
            entity.location.y -= 1;
        }
        _ => todo!()
    }
    println!("{} is at ({}, {})", entity.name, entity.location.x, entity.location.y);
}

fn main() {
    println!("--- Day 3: Perfectly Spherical Houses in a Vacuum ---");
    const FILE_PATH : &str = "./input.txt";
    // Read input.txt
    let input = fs::read_to_string(FILE_PATH)
        .expect("Should have been able to read the file");
    println!("Input from '{FILE_PATH}':\n{input}");

    let mut santa = Entity {
        name: "santa".to_string(),
        location: Coordinate {
            x: 0,
            y: 0
        }
    };
    let mut robo_santa = Entity {
        name: "robo santa".to_string(),
        location: Coordinate {
            x: 0,
            y: 0
        }
    };
    let mut visited: HashSet<Coordinate> = HashSet::new();
    println!("{} started at ({}, {})", santa.name, santa.location.x, santa.location.y);
    println!("{} started at ({}, {})", robo_santa.name, robo_santa.location.x, robo_santa.location.y);
    // insert initial starting spots
    visited.insert(santa.location.clone());
    visited.insert(robo_santa.location.clone());
    let mut turn = true;
    for c in input.chars() {
        if turn {
            move_entity( &mut santa, c);
            visited.insert(santa.location.clone());
            turn = false;
        } else {
            move_entity(&mut robo_santa, c);
            visited.insert(robo_santa.location.clone());
            turn = true;
        }
    }
    // for history in visited.iter() {
    //    println!("{}, {}", history.x, history.y);
    // }
    println!("santa and robo santa visited {} unique houses", visited.len()); // 2341
}