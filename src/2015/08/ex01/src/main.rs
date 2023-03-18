use std::io::{self, Read};
use std::fs::File;

fn filename_to_string(s: &str) -> io::Result<String> {
    let mut file = File::open(s)?;
    let mut s = String::new();
    file.read_to_string(&mut s)?;
    Ok(s)
}

struct Count {
    character: u16,
}

fn parse(s: &str) -> (u16, u16) {

}

fn main() {
    println!("--- Day 8: Matchsticks ---");
    const FILE_PATH : &str = "./example.txt";
    // Read input.txt
    let input = filename_to_string(FILE_PATH);
    input.unwrap().split("\n").for_each(|line| {
        println!("{line}");
    });
}