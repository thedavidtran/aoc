use std::collections::HashSet;
use std::fs;

#[derive(PartialEq, Eq, Hash, Clone)]
struct Coordinate<> {
    x: i32,
    y: i32,
}

fn main() {
    println!("--- Day 3: Perfectly Spherical Houses in a Vacuum ---");
    const FILE_PATH : &str = "./input.txt";
    // Read input.txt
    let input = fs::read_to_string(FILE_PATH)
        .expect("Should have been able to read the file");
    println!("Input from '{FILE_PATH}':\n{input}");

    let mut coordinates = Coordinate { x: 0, y: 0 };
    let mut visited: HashSet<Coordinate> = HashSet::new();
    println!("santa started at ({}, {})", coordinates.x, coordinates.y);
    visited.insert(coordinates.clone()); // insert initial starting spot
    for c in input.chars() {
        match c {
            '<' => { // left
                println!("{c} left");
                coordinates.x -= 1;
            }
            '>' => { // right
                println!("{c} right");
                coordinates.x += 1;
            }
            '^' => { // up
                println!("{c} up");
                coordinates.y += 1;
            }
            'v' => { // down
                println!("{c} down");
                coordinates.y -= 1;
            }
            _ => todo!()
        }
        println!("santa is at ({}, {})", coordinates.x, coordinates.y);
        visited.insert(coordinates.clone());
    }
    for history in visited.iter() {
        println!("{}, {}", history.x, history.y);
    }
    println!("Santa visited {} unique houses", visited.len()); // 2081
}