use std::io::{self, Read};
use std::fs::File;
use lazy_static::lazy_static;
use fancy_regex::Regex;

fn filename_to_string(s: &str) -> io::Result<String> {
    let mut file = File::open(s)?;
    let mut s = String::new();
    file.read_to_string(&mut s)?;
    Ok(s)
}

#[derive(Debug)]
enum Mode {
    On,
    Off,
    Toggle
}

#[derive(Debug)]
struct Coordinate {
    x: usize,
    y: usize
}

#[derive(Debug)]
struct Command {
    mode: Mode,
    start: Coordinate,
    end: Coordinate
}

fn parse(s: &str) -> Command {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(?<mode>(toggle|turn on|turn off)).(?<start_x>\d+),(?<start_y>\d+)(\D)*(?<end_x>\d+),(?<end_y>\d+)").unwrap(); // capture groups to get coordinates and mode
    }
    let binding = RE.captures(s);
    let captures = binding.as_ref().unwrap();
    let mode = match captures.as_ref().expect("Matches").name("mode").expect("").as_str() {
        "toggle" => Mode::Toggle,
        "turn on" => Mode::On,
        "turn off" => Mode::Off,
        &_ => todo!()
    };
    let start_x: usize = captures.as_ref().expect("Matches").name("start_x").expect("").as_str().trim().parse().unwrap();
    let start_y: usize = captures.as_ref().expect("Matches").name("start_y").expect("").as_str().trim().parse().unwrap();
    let end_x: usize = captures.as_ref().expect("Matches").name("end_x").expect("").as_str().trim().parse().unwrap();
    let end_y: usize = captures.as_ref().expect("Matches").name("end_y").expect("").as_str().trim().parse().unwrap();
    Command {
       mode,
       start: Coordinate {
           x: start_x,
           y: start_y
       },
       end: Coordinate {
           x: end_x,
           y: end_y
       }
    }
}

fn process(command: &Command, lights: &mut [[usize; 1000]; 1000]) {
    for i in command.start.x..=command.end.x {
        for j in command.start.y..=command.end.y {
            lights[i][j] = match command.mode {
                Mode::On => { 1 },
                Mode::Off => { 0 },
                Mode::Toggle => { if lights[i][j] == 1 { 0 } else { 1 } }
            }
        }
    }
}

fn count(lights: &[[usize; 1000]; 1000]) -> i32 {
    let mut result = 0;
    for i in 0..lights.len() {
        for j in 0..lights[i].len() {
            if lights[i][j] == 1 {
              result += 1;
            }
        }
    }
    result
}

fn main() {
    println!("--- Day 6: Probably a Fire Hazard ---");
    const FILE_PATH : &str = "./input.txt";
    // Read input.txt
    let input = filename_to_string(FILE_PATH);
    let mut lights_lit_count = 0;
    let mut lights: [[usize; 1000]; 1000] = [[0; 1000]; 1000];
    input.unwrap().split("\n").for_each(|line| {
        println!("{line}");
        let command = parse(line);
        process(&command, &mut lights);
    });
    lights_lit_count = count(&lights);
    println!("Number of lights are lit: {lights_lit_count}"); // 377891
}