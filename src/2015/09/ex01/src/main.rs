use std::io::{self, Read};
use std::fs::File;
use std::collections::HashMap;
use lazy_static::lazy_static;
use fancy_regex::Regex;

fn filename_to_string(s: &str) -> io::Result<String> {
    let mut file = File::open(s)?;
    let mut s = String::new();
    file.read_to_string(&mut s)?;
    Ok(s)
}

fn parse(s: &str, weights: &mut HashMap<String, usize>) {
    lazy_static! {
        static ref RE_DIST: Regex = Regex::new(r"(?<from>[A-Za-z]+) to (?<to>[A-Za-z]+) = (?<dist>[\d]+)").unwrap();
    }
    if RE_DIST.is_match(s).unwrap() {
        let binding = RE_DIST.captures(s);
        let captures = binding.as_ref().unwrap();
        let from: &str = captures.as_ref().expect("Matches").name("from").expect("").as_str();
        let to: &str = captures.as_ref().expect("Matches").name("to").expect("").as_str();
        let dist: usize = captures.as_ref().expect("Matches").name("dist").expect("").as_str().parse().unwrap();
        println!("from: {}, to: {}, dist: {}", from, to, dist);
        weights.insert(format!("{from}_{to}"), dist);
        weights.insert(format!("{to}_{from}"), dist);
    }
}

fn main() {
    println!("--- Day 9: All in a Single Night ---");
    const FILE_PATH : &str = "./example.txt";
    // Read input.txt
    let input = filename_to_string(FILE_PATH);
    let mut weights: HashMap<String, usize> = HashMap::new();
    let visited: HashMap<String, bool> = HashMap::new();
    input.unwrap().split("\n").for_each(|line| {
        println!("{line}");
        parse(&line, &mut weights);
    });
    println!("weights: {:#?}", weights);
    println!("visited: {:#?}", visited);
}