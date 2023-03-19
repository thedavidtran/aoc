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

#[derive(Debug)]
enum Mode {
    Gain,
    Lose
}

fn parse(s: &str, actors: &mut HashSet<String>, scores: &mut HashMap<String, isize>) {
    lazy_static! {
        static ref RE_ENTRY: Regex = Regex::new(r"(?<actor>[A-Za-z]+).*(?<mode>(gain|lose)).* (?<score>\d+).* (?<neighbour>([A-Za-z]+))\.").unwrap();
    }
    if RE_ENTRY.is_match(s).unwrap() {
        let binding = RE_ENTRY.captures(s);
        let captures = binding.as_ref().unwrap();
        let mode = match captures.as_ref().expect("Matches").name("mode").expect("").as_str() {
            "gain" => Mode::Gain,
            "lose" => Mode::Lose,
            &_ => todo!()
        };
        let actor = captures.as_ref().expect("Matches").name("actor").expect("").as_str();
        let neighbour = captures.as_ref().expect("Matches").name("neighbour").expect("").as_str();
        let score: isize = match mode {
            Mode::Gain => {
                captures.as_ref().expect("Matches").name("score").expect("").as_str().trim().parse().unwrap()
            },
            Mode::Lose => {
                -captures.as_ref().expect("Matches").name("score").expect("").as_str().trim().parse::<isize>().unwrap()
            },
            _ => todo!()
        };
        println!("matches! {:#?}, '{:#?}', '{}', '{}'", mode, score, actor, neighbour);
        actors.insert(String::from(actor));
        scores.insert(format!("{}_{}", actor, neighbour), score);
    }
}

fn calc_happiness_score(arrangement: &Vec<&String>, scores: &HashMap<String, isize>) -> isize {
    let mut score: isize = 0;
    println!("arrangement {:#?}", arrangement);
    for x in 0..arrangement.len() {
        let actor = arrangement.get(x).unwrap();
        let mut neighbour1 = match x {
            0 => arrangement.get(arrangement.len() -1).unwrap(),
            _ => arrangement.get(x - 1).unwrap()
        };
        let key = format!("{}_{}", actor, neighbour1);
        let s = scores.get(&key).unwrap();
        println!("neighbour 1 key: {} score: {}", key, s);
        score += s;
        let mut neighbour2 = if x == arrangement.len() - 1 {
            arrangement.get(0).unwrap()
        } else {
            arrangement.get(x + 1).unwrap()
        };
        let key = format!("{}_{}", actor, neighbour2);
        let s = scores.get(&key).unwrap();
        println!("neighbour 2 key: {} score: {}", key, s);
        score += s;
    }
    return score;
}

fn main() {
    println!("--- Day 13: Knights of the Dinner Table ---");
    const FILE_PATH : &str = "./input.txt";
    // Read input.txt
    let input = filename_to_string(FILE_PATH);
    let mut actors: HashSet<String> = HashSet::new();
    let mut scores: HashMap<String, isize> = HashMap::new();
    input.unwrap().split("\n").for_each(|line| {
        println!("{line}");
        parse(&line, &mut actors, &mut scores);
    });
    println!("{:#?}", actors);
    println!("{:#?}", scores);
    println!("{:#?}", actors.iter().permutations(actors.len()).unique().map(|arrangement| calc_happiness_score(&arrangement, &scores)).collect::<Vec<_>>());
    let happiness_scores = actors.iter().permutations(actors.len()).unique().map(|arrangement| calc_happiness_score(&arrangement, &scores)).collect::<Vec<_>>();
    println!("max happiness score: {:#?}", happiness_scores.iter().max().unwrap()); // 618
}