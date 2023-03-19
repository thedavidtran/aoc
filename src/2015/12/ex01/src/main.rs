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

fn parse(s: &str) -> i32 {
    lazy_static! {
        static ref RE_NUMBERS: Regex = Regex::new(r"(-?\d+)").unwrap();
    }
    let numbers: Vec<i32> = RE_NUMBERS.find_iter(s).filter_map(|digits| digits.expect("").as_str().parse::<i32>().ok()).collect();
    let sum: i32 = numbers.iter().sum();
    return sum;
}

fn main() {
    println!("--- Day 12: JSAbacusFramework.io ---");
    const FILE_PATH : &str = "./input.txt";
    // Read input.txt
    let input = filename_to_string(FILE_PATH);
    let mut total: i32 = 0;
    input.unwrap().split("\n").for_each(|line| {
        println!("{line}");
        total += parse(&line);
    });
    println!("Total: {}", total); // 156366
}