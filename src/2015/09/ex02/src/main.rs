use std::io::{self, Read};
use std::fs::File;
use std::collections::HashMap;
use std::collections::HashSet;
use lazy_static::lazy_static;
use fancy_regex::Regex;
use itertools::Itertools;

fn filename_to_string(s: &str) -> io::Result<String> {
    let mut file = File::open(s)?;
    let mut s = String::new();
    file.read_to_string(&mut s)?;
    Ok(s)
}

fn parse(s: &str, weights: &mut HashMap<String, usize>, cities: &mut HashSet<String>) {
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
        cities.insert(String::from(from));
        cities.insert(String::from(to));
    }
}

fn calc_distance(path: &Vec<&String>, weights: &HashMap<String, usize>) -> usize {
    let mut dist: usize = 0;
    for x in 0..path.len()-1 {
        let key = format!("{}_{}", path.get(x).unwrap(), path.get(x+1).unwrap());
        let distance = weights.get(&key).unwrap();
        dist += distance;
    }
    return dist;
}

fn main() {
    println!("--- Day 9: All in a Single Night ---");
    println!("--- Part 2 ---");
    const FILE_PATH : &str = "./input.txt";
    // Read input.txt
    let input = filename_to_string(FILE_PATH);
    let mut weights: HashMap<String, usize> = HashMap::new();
    let mut cities: HashSet<String> = HashSet::new();
    input.unwrap().split("\n").for_each(|line| {
        println!("{line}");
        parse(&line, &mut weights, &mut cities);
    });
    println!("weights: {:#?}", weights);
    println!("cities: {:#?}", cities);
    let distances: Vec<usize> = cities.iter().permutations(cities.len()).unique().map(|path| calc_distance(&path, &weights)).collect();
    println!("number of paths: {}", distances.len());
    let min = distances.iter().min();
    let max = distances.iter().max();
    println!("minimum distance is {:#?}", min.unwrap()); // 251
    println!("maximum distance is {:#?}", max.unwrap()); // 898
}