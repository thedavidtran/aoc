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

fn contains_pair_repeating_not_overlapping(s: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"([a-z]{2,}).*\1{1,}").unwrap();
    }
    return RE.is_match(s).unwrap();
}

fn contains_one_letter_repeating_with_at_exactly_one_character_between(s: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"([a-z]).{1}\1{1,}").unwrap();
    }
    return RE.is_match(s).unwrap();
}

fn main() {
    println!("--- Day 5: Doesn't He Have Intern-Elves For This? ---");
    println!("--- Part 2 ---");
    const FILE_PATH : &str = "./input.txt";
    // Read input.txt
    let input = filename_to_string(FILE_PATH);
    let mut nice_count = 0;
    input.unwrap().split("\n").for_each(|line| {
        println!("{line}");
        // println!("pair repeating not overlapping: {}", contains_pair_repeating_not_overlapping(line));
        // println!("contains one letter repeating with character between: {}", contains_one_letter_repeating_with_at_exactly_one_character_between(line));
        if contains_pair_repeating_not_overlapping(line) &&
            contains_one_letter_repeating_with_at_exactly_one_character_between(line) {
            nice_count += 1;
        }
    });

    println!("Number of nice strings: {nice_count}"); // 55
}