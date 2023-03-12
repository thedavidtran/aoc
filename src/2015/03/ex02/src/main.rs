use std::collections::HashSet;
use std::fs;

#[derive(PartialEq, Eq, Hash, Clone)]
struct Coordinate<> {
    x: i32,
    y: i32,
}

fn move_entity(entity : String, coordinate : &mut Coordinate, direction: char) {
    match direction {
        '<' => { // left
            println!("{direction} left");
            coordinate.x -= 1;
        }
        '>' => { // right
            println!("{direction} right");
            coordinate.x += 1;
        }
        '^' => { // up
            println!("{direction} up");
            coordinate.y += 1;
        }
        'v' => { // down
            println!("{direction} down");
            coordinate.y -= 1;
        }
        _ => todo!()
    }
    println!("{} is at ({}, {})", entity, coordinate.x, coordinate.y);
}

fn main() {
    println!("--- Day 3: Perfectly Spherical Houses in a Vacuum ---");
    const FILE_PATH : &str = "./input.txt";
    // Read input.txt
    let input = fs::read_to_string(FILE_PATH)
        .expect("Should have been able to read the file");
    println!("Input from '{FILE_PATH}':\n{input}");

    let mut santa_coordinates = Coordinate { x: 0, y: 0 };
    let mut robo_santa_coordinates = Coordinate { x: 0, y: 0 };
    let mut visited: HashSet<Coordinate> = HashSet::new();
    println!("santa started at ({}, {})", santa_coordinates.x, santa_coordinates.y);
    println!("robo santa started at ({}, {})", robo_santa_coordinates.x, robo_santa_coordinates.y);
    // insert initial starting spots
    visited.insert(santa_coordinates.clone());
    visited.insert(robo_santa_coordinates.clone());
    let mut turn = true;
    for c in input.chars() {
        if (turn) {
            move_entity("santa".to_string(), &mut santa_coordinates, c);
            visited.insert(santa_coordinates.clone());
            turn = false;
        } else {
            move_entity("robo santa".to_string(), &mut robo_santa_coordinates, c);
            visited.insert(robo_santa_coordinates.clone());
            turn = true;
        }
    }
    // for history in visited.iter() {
    //    println!("{}, {}", history.x, history.y);
    // }
    println!("santa and robo santa visited {} unique houses", visited.len()); // 2341
}