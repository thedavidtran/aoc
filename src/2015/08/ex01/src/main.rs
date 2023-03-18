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
struct Count {
    string: usize, // count of length in code
    memory: usize // count of length in memory
}

fn parse(s: &str) -> Count {
    lazy_static! {
        static ref RE_BACKSLASH: Regex = Regex::new(r"[\\]{2}").unwrap();
        static ref RE_DOUBLEQUOTE: Regex = Regex::new(r#"\\\""#).unwrap();
        static ref RE_HEXIDECIMAL: Regex = Regex::new(r"\\x[\h]{2}").unwrap();
    }
    let mut trimmed = s.trim();
    let len = trimmed.len();
    let mut memory = len;
    trimmed = &trimmed[1..len-1];
    memory -= 2;
    // count
    // - \\ -> \
    // - \" -> "
    // - \x## -> single character with that ASCII code, where # -> 0-9a-f
    if RE_BACKSLASH.is_match(trimmed).unwrap() {
        let backslash_count = RE_BACKSLASH.find_iter(trimmed).count();
        println!("{} has backslash counts {}", trimmed, backslash_count);
        memory -= backslash_count;
    }
    if RE_DOUBLEQUOTE.is_match(trimmed).unwrap() {
        let doublequote_count = RE_DOUBLEQUOTE.find_iter(trimmed).count();
        println!("{} has double quote counts {}", trimmed, doublequote_count);
        memory -= doublequote_count;
    }
    if RE_HEXIDECIMAL.is_match(trimmed).unwrap() {
        let hexidecimal_count = RE_HEXIDECIMAL.find_iter(trimmed).count();
        println!("{} has hexidecimal counts {}", trimmed, hexidecimal_count);
        memory -= (hexidecimal_count * 3);
    }
    return Count {
        string: len,
        memory
    }
}

fn main() {
    println!("--- Day 8: Matchsticks ---");
    const FILE_PATH : &str = "./input.txt";
    // Read input.txt
    let input = filename_to_string(FILE_PATH);
    let mut total_string = 0;
    let mut total_memory = 0;
    input.unwrap().split("\n").for_each(|line| {
        println!("{line}");
        let count = parse(line);
        total_string += count.string;
        total_memory += count.memory;
    });
    println!("total string: {}, total memory: {}", total_string, total_memory);
    println!("difference: {}", total_string - total_memory); // 1371
}