use std::cmp::max;
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
struct Reindeer {
    name: String,
    speed: isize,
    duration: isize,
    rest: isize
}

impl Reindeer {
    fn traveled(&self, duration: isize) -> isize {
        let mut distance: isize = 0;
        let mut d: isize = duration;
        while d > 0 {
            // travel
            if d >= self.duration {
                distance += self.duration * self.speed;
                d -= self.duration;
            } else {
                distance += d * self.speed;
                d -= d;
            }
            // rest
            d -= self.rest;
        }
        distance
    }
}

fn process(s: &str) -> Reindeer {
    lazy_static! {
        static ref RE_REINDEER: Regex = Regex::new(r"^(?<name>[A-Za-z]+) can fly (?<speed>\d+) km/s for (?<duration>\d+) seconds, but then must rest for (?<rest>\d+) seconds.").unwrap();
    }
    if RE_REINDEER.is_match(s).unwrap() {
        let binding = RE_REINDEER.captures(s);
        let captures = binding.as_ref().unwrap();
        let name: &str = captures.as_ref().expect("Matches").name("name").expect("").as_str();
        let speed: isize = captures.as_ref().expect("Matches").name("speed").expect("").as_str().parse().unwrap();
        let duration: isize = captures.as_ref().expect("Matches").name("duration").expect("").as_str().parse().unwrap();
        let rest: isize = captures.as_ref().expect("Matches").name("rest").expect("").as_str().parse().unwrap();
        return Reindeer {
            name: String::from(name),
            speed,
            duration,
            rest
        };
    }
    panic!("Parsing did not match!");
}

fn main() {
    println!("--- Day 14: Reindeer Olympics ---");
    const TIME_DURATION: isize = 2503;
    const FILE_PATH : &str = "./input.txt";
    // Read input.txt
    let input = filename_to_string(FILE_PATH);
    let mut max_distance = 0;
    input.unwrap().split("\n").for_each(|line| {
        println!("{line}");
        let reindeer = process(&line);
        let traveled = reindeer.traveled(TIME_DURATION);
        max_distance = max(max_distance, traveled);
    });
    println!("The max traveled distance is: {}", max_distance); // 2655
}