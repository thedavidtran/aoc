use std::cmp::max;
use std::io::{self, Read};
use std::fs::File;
use lazy_static::lazy_static;
use fancy_regex::Regex;
use itertools::Itertools;
use std::collections::HashMap;

fn filename_to_string(s: &str) -> io::Result<String> {
    let mut file = File::open(s)?;
    let mut s = String::new();
    file.read_to_string(&mut s)?;
    Ok(s)
}

#[derive(Debug,Clone)]
struct Ingredient {
    name: String,
    capacity: isize,
    durability: isize,
    flavor: isize,
    texture: isize,
    calories: isize
}

fn process(s: &str) -> Ingredient {
    lazy_static! {
        static ref RE_INGREDIENT: Regex = Regex::new(r"(?<name>[A-Za-z]+): .* (?<capacity>-?\d+), .* (?<durability>-?\d+), .* (?<flavor>-?\d+), .* (?<texture>-?\d+), .* (?<calories>-?\d+)").unwrap();
    }
    if RE_INGREDIENT.is_match(s).unwrap() {
        let binding = RE_INGREDIENT.captures(s);
        let captures = binding.as_ref().unwrap();
        let name: &str = captures.as_ref().expect("Matches").name("name").expect("").as_str();
        let capacity: isize = captures.as_ref().expect("Matches").name("capacity").expect("").as_str().trim().parse().unwrap();
        let durability: isize = captures.as_ref().expect("Matches").name("durability").expect("").as_str().trim().parse().unwrap();
        let flavor: isize = captures.as_ref().expect("Matches").name("flavor").expect("").as_str().trim().parse().unwrap();
        let texture: isize = captures.as_ref().expect("Matches").name("texture").expect("").as_str().trim().parse().unwrap();
        let calories: isize = captures.as_ref().expect("Matches").name("calories").expect("").as_str().trim().parse().unwrap();
        return Ingredient {
            name: String::from(name),
            capacity,
            durability,
            flavor,
            texture,
            calories
        }
    }
    panic!("Parsing failed! Double check Regex");
}

fn main() {
    let mut ingredients: Vec<Ingredient> = vec![];
    let mut max_score = 0;
    let mut best_recipe: HashMap<String, usize> = HashMap::new();
    println!("--- Day 15: Science for Hungry People ---");
    const FILE_PATH : &str = "./input.txt";
    let input = filename_to_string(FILE_PATH);
    input.unwrap().split("\n").for_each(|line| {
        println!("{line}");
        let ingredient = process(&line);
        ingredients.push(ingredient.clone());
    });
    let combinations = ingredients.iter().combinations_with_replacement(100);
    for com in combinations {
        let mut capacity_score = 0;
        let mut durability_score = 0;
        let mut flavor_score = 0;
        let mut texture_score = 0;
        for ingredient in &com {
            capacity_score += ingredient.capacity;
            durability_score += ingredient.durability;
            flavor_score += ingredient.flavor;
            texture_score += ingredient.texture;
        }
        if capacity_score < 0 {
            capacity_score = 0;
        }
        if durability_score < 0 {
            durability_score = 0;
        }
        if flavor_score < 0 {
            flavor_score = 0;
        }
        if texture_score < 0 {
            texture_score = 0;
        }
        let total_score = capacity_score * durability_score * flavor_score * texture_score;
        if total_score > max_score {
            println!("capacity {} durability {} flavor {}, texture {}", capacity_score, durability_score, flavor_score, texture_score);
            println!("total score {}", total_score);
            let name_frequency = com.iter().counts_by(|i| i.name.clone());
            best_recipe = name_frequency.clone();
            max_score = total_score;
        }
    }
    println!("Highest score: {}", max_score); // 222870
    println!("Best recipe: {:#?}", best_recipe);
}